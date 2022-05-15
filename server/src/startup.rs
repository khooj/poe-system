use crate::configuration::Settings;
use crate::{
    application::{build_calculating::BuildCalculating, stash_receiver::StashReceiver},
    infrastructure::{public_stash_retriever::Client, repositories::create_repositories},
};

use crate::infrastructure::controller::{get_build, new_build};
use actix_web::{web, App, HttpServer};
use std::net::TcpListener;
use tracing::error;
use tracing_actix_web::TracingLogger;

const USER_AGENT: &str = "OAuth poe-system/0.0.1 (contact: bladoff@gmail.com)";

pub struct Application {
    listener: TcpListener,
    stash_receiver: StashReceiver,
    interval: u64,
    enable: bool,
    build_calculating: BuildCalculating,
}

impl Application {
    pub async fn build(configuration: Settings) -> Result<Self, std::io::Error> {
        Application::setup_tracing();

        let (raw_items, tasks, builds) = create_repositories(&configuration.database)
            .await
            .expect("can't create repositories");

        let client = Client::new(USER_AGENT.to_owned());

        let mut stash_receiver = StashReceiver::new(
            raw_items.clone(),
            client,
            configuration.application.only_leagues,
        );

        if configuration.start_change_id.is_some() {
            stash_receiver
                .ensure_stash_id(configuration.start_change_id.unwrap())
                .await
                .expect("can't insert start change id");
        }

        let build_calculating = BuildCalculating::new(raw_items.clone(), tasks, builds);

        let addr = format!(
            "{}:{}",
            configuration.application.host, configuration.application.port
        );

        let enable_items_refresh = configuration.application.enable_items_refresh;
        let refresh_interval_secs = configuration.application.refresh_interval_secs;

        let listener = TcpListener::bind(&addr)?;

        Ok(Self {
            listener,
            stash_receiver,
            interval: refresh_interval_secs,
            enable: enable_items_refresh,
            build_calculating,
        })
    }

    fn setup_tracing() {
        use tracing_subscriber::{fmt, prelude::*, EnvFilter};

        let fmt_subscriber = fmt::layer();

        let env_subscriber = EnvFilter::try_from_default_env()
            .or_else(|_| EnvFilter::try_new("info"))
            .unwrap();

        let tracer = opentelemetry_jaeger::new_pipeline()
            .with_service_name("poe-system")
            .install_batch(opentelemetry::runtime::Tokio)
            .expect("can't init opentelementry");

        let opentelemetry_jaeger = tracing_opentelemetry::layer().with_tracer(tracer);
        tracing_subscriber::registry()
            .with(fmt_subscriber)
            .with(env_subscriber)
            .with(opentelemetry_jaeger)
            .try_init()
            .expect("can't init tracing subscribers");
    }

    // TODO: graceful shutdown?
    pub async fn run(self) -> Result<(), std::io::Error> {
        use tokio::time::{interval, Duration};

        let Application {
            listener: l,
            stash_receiver: mut sr,
            build_calculating: bc,
            ..
        } = self;
        if self.enable {
            let mut interval = interval(Duration::from_secs(self.interval));
            tokio::spawn(async move {
                loop {
                    interval.tick().await;
                    if let Err(e) = sr.receive().await {
                        error!("{}", e);
                    }
                }
            });
        }

        let bc1 = bc.clone();
        tokio::spawn(async move {
            let mut interval = interval(Duration::from_millis(100));
            loop {
                interval.tick().await;
                let ret = bc1.calculate_next_build().await;
                if let Err(e) = ret {
                    error!("error calculating next build: {}", e);
                    tokio::time::sleep(Duration::from_secs(1)).await;
                }
            }
        });

        HttpServer::new(move || {
            App::new()
                .wrap(TracingLogger::default())
                .app_data(web::Data::new(bc.clone()))
                .service(new_build)
                .service(get_build)
        })
        .workers(4)
        .listen(l)
        .expect("cant listen actix-web server")
        .run()
        .await
        .expect("cant run actix-web server");
        Ok(())
    }
}
