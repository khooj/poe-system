use crate::configuration::Settings;
use crate::{
    application::stash_receiver::StashReceiver,
    infrastructure::{public_stash_retriever::Client, repositories::create_repositories},
};

use actix_web::{dev::Server, web, App, HttpServer};
use sqlx::postgres::PgPool;
use std::net::TcpListener;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::{thread, thread::JoinHandle};
use tracing::error;
use tracing_actix_web::TracingLogger;

const USER_AGENT: &str = "OAuth poe-system/0.0.1 (contact: bladoff@gmail.com)";

pub struct Application {
    listener: TcpListener,
    stash_receiver: StashReceiver,
    interval: u64,
    enable: bool,
}

impl Application {
    pub async fn build(configuration: Settings) -> Result<Self, std::io::Error> {
        Application::setup_tracing();

        let (raw_items,) = create_repositories(&configuration.database)
            .await
            .expect("can't create repositories");

        let client = Client::new(USER_AGENT.to_owned());

        let mut stash_receiver =
            StashReceiver::new(raw_items, client, configuration.application.only_leagues);

        if configuration.start_change_id.is_some() {
            stash_receiver
                .ensure_stash_id(configuration.start_change_id.unwrap())
                .await
                .expect("can't insert start change id");
        }

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
        })
    }

    fn setup_tracing() {
        use tracing_subscriber::{fmt, prelude::*, registry::Registry, EnvFilter};

        let fmt_subscriber = fmt::layer();

        let env_subscriber = EnvFilter::try_from_default_env()
            .or_else(|_| EnvFilter::try_new("info"))
            .unwrap();

        let collector = Registry::default()
            .with(fmt_subscriber)
            .with(env_subscriber);

        tracing_log::LogTracer::init().expect("cant set log tracer");
        tracing::subscriber::set_global_default(collector).expect("could not set global default");
    }

    // TODO: graceful shutdown?
    pub async fn run(self) -> Result<(), std::io::Error> {
        let Application {
            listener: l,
            stash_receiver: mut sr,
            ..
        } = self;
        if self.enable {
            let mut interval =
                tokio::time::interval(tokio::time::Duration::from_secs(self.interval));
            tokio::spawn(async move {
                loop {
                    interval.tick().await;
                    if let Err(e) = sr.receive().await {
                        error!("{}", e);
                    }
                }
            });
        }

        HttpServer::new(move || App::new().wrap(TracingLogger::default()))
            .workers(4)
            .listen(l)
            .expect("cant listen actix-web server")
            .run()
            .await
            .expect("cant run actix-web server");
        Ok(())
    }
}
