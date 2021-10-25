use crate::application::configuration::Settings;
use crate::implementations::http_service_layer::HttpServiceLayer;
use crate::{
    actors::stash_receiver::StashReceiverActor, implementations::public_stash_retriever::Client,
    implementations::public_stash_timer::PublicStashTimer, implementations::ItemsRepository,
};

use actix::prelude::*;
use actix_web::{dev::Server, web, App, HttpServer};
use jsonrpc_v2::{Data, Server as JsonrpcServer};
use std::net::TcpListener;
use std::sync::mpsc::{channel, Receiver};
use std::{thread, thread::JoinHandle};
use tracing::error;
use tracing_actix_web::TracingLogger;

const USER_AGENT: &str = "OAuth poe-system/0.0.1 (contact: bladoff@gmail.com)";

pub struct Application {
    server: Server,
    handle: JoinHandle<Result<(), std::io::Error>>,
    rx: Receiver<System>,
}

impl Application {
    pub async fn build(configuration: Settings) -> Result<Self, std::io::Error> {
        Application::setup_tracing();

        let client_opts = mongodb::options::ClientOptions::parse(&configuration.mongo)
            .await
            .expect("cant parse mongo db url");
        let client =
            mongodb::Client::with_options(client_opts).expect("cant create mongodb client");
        let repo = ItemsRepository {
            client,
            database: "poe-system".into(),
        };

        if configuration.start_change_id.is_some() {
            if let Err(_) = repo.get_stash_id().await {
                repo.set_stash_id(configuration.start_change_id.as_ref().unwrap())
                    .await
                    .expect("cant set stash id");
            }
        }

        let svc = HttpServiceLayer {
            item_repo: repo.clone(),
        };

        let (tx, rx) = channel::<actix::System>();

        let addr = format!(
            "{}:{}",
            configuration.application.host, configuration.application.port
        );

        let enable_items_refresh = configuration.application.enable_items_refresh;
        let refresh_interval_secs = configuration.application.refresh_interval_secs;

        let handle = thread::spawn(move || {
            let repo = repo.clone();
            let system_runner = System::new();

            let system = System::try_current().expect("cant get thread system");

            tx.send(system).expect("cant send running system");

            system_runner.block_on(async move {
                let actor = SyncArbiter::start(1, move || {
                    StashReceiverActor::new(
                        repo.clone(),
                        Client::new(USER_AGENT.to_owned().clone()),
                        configuration.application.only_leagues.clone(),
                    )
                });

                let timer = PublicStashTimer {
                    actor: actor.clone(),
                    interval: std::time::Duration::from_secs(refresh_interval_secs),
                };

                if enable_items_refresh {
                    let _ = timer.start();
                }
            });
            system_runner.run()
        });

        let listener = TcpListener::bind(&addr)?;
        let server = run(listener, svc)?;

        Ok(Self { server, handle, rx })
    }

    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        let result = self.server.await;
        let system = self.rx.recv().expect("cant get system from child thread");
        system.stop();
        let result2 = match self.handle.join() {
            Ok(k) => k,
            Err(e) => {
                error!("child thread panicked: {:?}", e);
                Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "child thread panicked",
                ))
            }
        };
        result.and(result2)
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
}

fn run(listener: TcpListener, svc: HttpServiceLayer) -> Result<Server, std::io::Error> {
    let rpc = JsonrpcServer::new().with_data(Data::new(svc)).finish();

    let server = HttpServer::new(move || {
        let rpc = rpc.clone();
        App::new()
            .service(
                web::service("/api")
                    .guard(actix_web::guard::Post())
                    .finish(rpc.into_web_service()),
            )
            .wrap(TracingLogger)
    })
    .workers(4)
    .listen(listener)?
    .run();

    Ok(server)
}
