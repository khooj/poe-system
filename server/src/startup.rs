use crate::application::configuration::Settings;
use crate::implementations::http_controller::get_maps_list;
use crate::implementations::http_service_layer::HttpServiceLayer;
use crate::{
    application::actors::stash_receiver::StashReceiverActor, implementations::public_stash_retriever::Client,
    implementations::public_stash_timer::PublicStashTimer, implementations::ItemsRepository,
};

use actix::{Actor, SyncArbiter, System};
use actix_web::{dev::Server, web, App, HttpServer};
use jsonrpc_v2::{Data, Server as JsonrpcServer};
use std::net::TcpListener;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::{thread, thread::JoinHandle};
use tracing::error;
use tracing_actix_web::TracingLogger;

const USER_AGENT: &str = "OAuth poe-system/0.0.1 (contact: bladoff@gmail.com)";

pub struct Application {
    web_handle: JoinHandle<()>,
    handle: JoinHandle<()>,
    rx: Receiver<System>,
    rx2: Receiver<actix_web::rt::System>,
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

        let svc = HttpServiceLayer {
            item_repo: repo.clone(),
        };

        let (tx, rx) = channel::<actix::System>();
        let (tx2, rx2) = channel::<actix_web::rt::System>();

        let addr = format!(
            "{}:{}",
            configuration.application.host, configuration.application.port
        );

        let enable_items_refresh = configuration.application.enable_items_refresh;
        let refresh_interval_secs = configuration.application.refresh_interval_secs;

        let handle = thread::spawn(move || {
            let repo = repo.clone();
            let system_runner = System::new();

            let system = System::current();

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

            tx.send(system).expect("cant send running system");
            system_runner.run().unwrap()
        });

        let listener = TcpListener::bind(&addr)?;
        let web_handle = run(tx2, listener, svc)?;

        Ok(Self {
            web_handle,
            handle,
            rx,
            rx2,
        })
    }

    pub async fn stop(self) -> Result<(), std::io::Error> {
        let system = self.rx.recv().expect("cant get system from child thread");
        system.stop();
        let system = self.rx2.recv().expect("cant get actix_rt system");
        system.stop();

        match self.handle.join() {
            Ok(k) => {}
            Err(e) => {
                error!("child thread panicked: {:?}", e);
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "child thread panicked",
                ));
            }
        };
        match self.web_handle.join() {
            Ok(k) => {}
            Err(e) => {
                error!("child thread panicked: {:?}", e);
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "actix_rt child thread panicked",
                ));
            }
        };
        Ok(())
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

fn run(
    tx: Sender<actix_web::rt::System>,
    listener: TcpListener,
    svc: HttpServiceLayer,
) -> Result<JoinHandle<()>, std::io::Error> {
    let web_thread = thread::spawn(move || {
        let rpc = JsonrpcServer::new()
            .with_data(Data::new(svc))
            .with_method("get_maps_list", get_maps_list)
            .finish();
        let mut system = actix_web::rt::System::new("actix-web");

        system.block_on(async move {
            HttpServer::new(move || {
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
            .listen(listener)
            .expect("cant listen actix-web server")
            .run()
            .await
            .expect("cant run actix-web server");
        });

        let s = actix_web::rt::System::current();

        tx.send(s).expect("cant send actix_rt system");

        system.run().unwrap()
    });

    Ok(web_thread)
}
