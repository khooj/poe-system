use std::{
    cell::RefCell,
    io,
    sync::{
        mpsc::{self, Receiver},
        Arc, Mutex,
    },
    thread::{self, JoinHandle},
    time::{Duration, Instant},
};

use application::*;
use clipboard::{ClipboardContext, ClipboardProvider};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use pob::{Pob, PobError};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Gauge, Tabs, Widget},
    Frame, Terminal,
};
use tui_textarea::{CursorMove, Input, Key, TextArea};

#[derive(Default, Clone, Copy, PartialEq)]
enum AppTab {
    #[default]
    EnterBuild,
    BuildResults,
}

const TABS_IDX: &[(usize, AppTab, &str)] = &[
    (0, AppTab::EnterBuild, "Enter build"),
    (1, AppTab::BuildResults, "Build results"),
];

enum ProcessInput {
    Cycle,
    Exit,
}

const ERROR_BLINK_DURATION: Duration = Duration::from_secs(1);
const TICK_RATE: Duration = Duration::from_millis(100);

struct App<'a> {
    current_tab: usize,
    enter_build_textarea: TextArea<'a>,
    error_build_input: Option<PobError>,
    error_build_input_blick: Duration,
    calculating_state: CalculatingState,
    progress: Arc<Mutex<usize>>,
    calc_err: Arc<Mutex<Option<Result<(), CalculateBuildError>>>>,
}

impl<'a> Default for App<'a> {
    fn default() -> Self {
        App {
            current_tab: 0,
            enter_build_textarea: TextArea::default(),
            error_build_input_blick: Duration::ZERO,
            error_build_input: None,
            calculating_state: CalculatingState::default(),
            progress: Arc::new(Mutex::new(0)),
            calc_err: Arc::new(Mutex::new(None)),
        }
    }
}

impl<'a> App<'a> {
    pub fn new() -> Self {
        let mut app = App::default();
        app.enter_build_textarea
            .set_cursor_line_style(Style::default());
        app
    }

    pub fn on_tick(&mut self) {
        self.error_build_input_blick = self.error_build_input_blick.saturating_sub(TICK_RATE);
        if self.error_build_input_blick.is_zero() {
            self.error_build_input = None;
        }

        let block_style = match self.error_build_input {
            Some(_) => Style::default().fg(Color::Red),
            None => Style::default(),
        };
        let block = Block::default().style(block_style);
        self.enter_build_textarea.set_block(block);
    }

    pub fn next(&mut self) {
        self.current_tab += 1;
        if self.current_tab >= TABS_IDX.len() {
            self.current_tab = TABS_IDX.len() - 1;
        }
    }

    pub fn previous(&mut self) {
        self.current_tab = self.current_tab.saturating_sub(1);
    }

    pub fn tab(&self) -> AppTab {
        TABS_IDX.iter().find(|e| e.0 == self.current_tab).unwrap().1
    }

    fn shared_process_input(&mut self, input: &Input) -> Option<ProcessInput> {
        match *input {
            Input { key: Key::Esc, .. } => Some(ProcessInput::Exit),
            Input {
                key: Key::Right, ..
            } => {
                self.next();
                None
            }
            Input { key: Key::Left, .. } => {
                self.previous();
                None
            }
            _ => None,
        }
    }

    fn switch_to_tab(&mut self, tab: AppTab) {
        self.current_tab = TABS_IDX.iter().find(|e| e.1 == tab).unwrap().0;
    }

    fn run_calculating_build(&mut self) {
        let (tx, rx) = mpsc::channel();
        let (tx_res, rx_res) = mpsc::channel();
        let cb = self.calculating_state.clone();
        let _ = thread::spawn(move || {
            let mut builder = tokio::runtime::Builder::new_multi_thread();
            let rt = builder.enable_all().build().unwrap();
            rt.block_on(async move {
                let res = cb.calculate_build_cost(tx).await;
                tx_res.send(res).unwrap();
            })
        });

        let calc_err = self.calc_err.clone();
        let progress = self.progress.clone();
        let _ = thread::spawn(move || loop {
            let p = rx.try_recv().ok();
            if let Some(p) = p {
                let mut pg = progress.lock().unwrap();
                *pg += p;
            }
            let mut res_err = calc_err.lock().unwrap();
            let recv_err = rx_res.try_recv();
            if let Ok(ok) = recv_err {
                *res_err = Some(ok);
                return;
            }
            thread::sleep(Duration::from_millis(250));
        });
    }

    fn enter_build_input(&mut self, input: Input) -> ProcessInput {
        match input {
            Input {
                key: Key::Enter, ..
            } => {
                let res = self
                    .calculating_state
                    .parse_pob(self.enter_build_textarea.lines()[0].clone());
                if res.is_err() {
                    self.error_build_input = res.err();
                    self.error_build_input_blick = ERROR_BLINK_DURATION;
                } else {
                    self.error_build_input = None;
                    self.switch_to_tab(AppTab::BuildResults);
                    self.run_calculating_build();
                }
            }
            Input {
                key: Key::Char('v'),
                ctrl: true,
                ..
            } => {
                let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
                let content = ctx.get_contents().unwrap();
                self.enter_build_textarea.insert_str(content);
            }
            Input {
                key: Key::Char('d'),
                ..
            } => {
                self.enter_build_textarea.move_cursor(CursorMove::Head);
                self.enter_build_textarea.delete_line_by_end();
            }
            _ => {}
        };
        ProcessInput::Cycle
    }

    fn build_results_input(&mut self, input: Input) -> ProcessInput {
        match input {
            Input {
                key: Key::Char('q'),
                ..
            } => return ProcessInput::Exit,
            _ => {}
        };
        ProcessInput::Cycle
    }

    pub fn process_input(&mut self, input: Input) -> ProcessInput {
        if let Some(pi) = self.shared_process_input(&input) {
            return pi;
        }

        match self.tab() {
            AppTab::EnterBuild => self.enter_build_input(input),
            AppTab::BuildResults => self.build_results_input(input),
        }
    }
}

fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    mut app: App,
    tick_rate: Duration,
) -> io::Result<()> {
    let mut last_tick = Instant::now();
    loop {
        terminal.draw(|f| ui(f, &app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));
        if crossterm::event::poll(timeout)? {
            let input = event::read()?.into();
            match app.process_input(input) {
                ProcessInput::Exit => return Ok(()),
                _ => {}
            };
        }
        if last_tick.elapsed() >= tick_rate {
            app.on_tick();
            last_tick = Instant::now();
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(5)
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(f.size());

    // let block = Block::default().style(Style::default().bg(Color::White).fg(Color::Black));
    // f.render_widget(block, f.size());

    let titles = TABS_IDX
        .iter()
        .map(|(_, _, t)| {
            let (first, rest) = t.split_at(1);
            Spans::from(vec![
                Span::styled(first, Style::default().fg(Color::Yellow)),
                Span::styled(rest, Style::default().fg(Color::Green)),
            ])
        })
        .collect();

    let tabs = Tabs::new(titles)
        .block(Block::default().borders(Borders::ALL).title("Tabs"))
        .select(app.current_tab)
        .style(Style::default().fg(Color::Cyan))
        .highlight_style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .bg(Color::DarkGray),
        );
    f.render_widget(tabs, chunks[0]);

    match app.tab() {
        AppTab::EnterBuild => f.render_widget(enter_build_widget(app), chunks[1]),
        AppTab::BuildResults => f.render_widget(build_results_widget(app), chunks[1]),
    };
}

fn enter_build_widget<'a>(app: &'a App) -> impl Widget + 'a {
    app.enter_build_textarea.widget()
}

fn build_results_widget(app: &App) -> impl Widget {
    let pg = app.progress.lock().unwrap();
    let ratio = (*pg as f64) / (app.calculating_state.max_progress() as f64);
    Gauge::default()
        .block(Block::default().title("progress").borders(Borders::ALL))
        .gauge_style(Style::default().fg(Color::Magenta))
        .ratio(ratio)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let app = App::new();
    let res = run_app(&mut terminal, app, TICK_RATE);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err);
    }

    // let pob = include_str!("pob.txt");
    // calculate_build_cost(pob)
    //     .await
    //     .expect("can't calculate build cost");
    Ok(())
}
