use std::{
    io,
    time::{Duration, Instant},
};

use application::*;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Tabs, Widget},
    Frame, Terminal,
};
use tui_textarea::{Input, Key, TextArea};

#[derive(Default, Clone, Copy)]
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

#[derive(Default)]
struct App<'a> {
    current_tab: usize,
    enter_build_textarea: TextArea<'a>,
}

impl<'a> App<'a> {
    pub fn new() -> Self {
        let mut app = App::default();
        app.enter_build_textarea
            .set_cursor_line_style(Style::default());
        app
    }

    pub fn on_tick(&mut self) {}

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

    fn enter_build_input(&mut self, input: Input) -> ProcessInput {
        match input {
            Input {
                key: Key::Enter, ..
            } => {}
            input => {
                self.enter_build_textarea.input(input);
            }
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
    Block::default()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let tick_rate = Duration::from_millis(250);
    let app = App::new();
    let res = run_app(&mut terminal, app, tick_rate);

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
