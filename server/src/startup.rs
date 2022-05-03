use crate::configuration::Settings;
use crate::{
    application::stash_receiver::StashReceiver,
    infrastructure::{
        public_stash_retriever::Client, repositories::mongo::items_repository::ItemsRepository,
    },
};

use actix_web::{dev::Server, web, App, HttpServer};
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

        let client_opts = mongodb::options::ClientOptions::parse(&configuration.mongo)
            .await
            .expect("cant parse mongo db url");
        let client =
            mongodb::Client::with_options(client_opts).expect("cant create mongodb client");
        let repo = ItemsRepository::new(client, "poe-system".into())
            .await
            .expect("cant create items repository");

        if configuration.start_change_id.is_some() {
            if let Err(_) = repo.get_stash_id().await {
                repo.set_stash_id(configuration.start_change_id.as_ref().unwrap())
                    .await
                    .expect("cant set stash id");
            }
        }

        let addr = format!(
            "{}:{}",
            configuration.application.host, configuration.application.port
        );

        let enable_items_refresh = configuration.application.enable_items_refresh;
        let refresh_interval_secs = configuration.application.refresh_interval_secs;

        let listener = TcpListener::bind(&addr)?;

        let client = Client::new(USER_AGENT.to_owned());

        let stash_receiver =
            StashReceiver::new(repo, client, configuration.application.only_leagues);

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
        let Application{listener: l, stash_receiver: mut sr, .. } = self;
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
