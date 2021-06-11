use crate::application::configuration::Settings;
use crate::implementations::http_controller::{calculate_pob, get_build_price};
use crate::{
    actors::stash_receiver::StashReceiverActor,
    implementations::item_repository::DieselItemRepository,
    implementations::public_stash_retriever::Client,
    implementations::public_stash_timer::PublicStashTimer,
};

use actix::prelude::*;
use actix_web::{dev::Server, web, App, HttpServer};
use diesel::{Connection, SqliteConnection};
use jsonrpc_v2::Server as JsonrpcServer;
use log::error;
use std::net::TcpListener;
use std::sync::mpsc::{channel, Receiver};
use std::sync::{Arc, Mutex};
use std::{thread, thread::JoinHandle};
use tokio::sync::Mutex as AsyncMutex;

const USER_AGENT: &str = "OAuth poe-system/0.0.1 (contact: bladoff@gmail.com)";

pub struct Application {
    server: Server,
    handle: JoinHandle<Result<(), std::io::Error>>,
    rx: Receiver<System>,
}

embed_migrations!("migrations");

impl Application {
    pub async fn build(configuration: Settings) -> Result<Self, std::io::Error> {
        env_logger::init();

        let addr = format!(
            "{}:{}",
            configuration.application.host, configuration.application.port
        );
        let listener = TcpListener::bind(&addr)?;
        let server = run(listener)?;

        let db_conn = SqliteConnection::establish(&configuration.database)
            .expect("cannot establish sqlite conn");

        embedded_migrations::run(&db_conn).expect("cant run migration");

        let repo = DieselItemRepository::new(db_conn).expect("cant create item repository");

        if configuration.start_change_id.is_some() {
            let stash = repo.get_stash_id().expect("cant get latest stash id");
            if stash.latest_stash_id.is_none() {
                repo.set_stash_id(configuration.start_change_id.as_ref().unwrap())
                    .expect("cant set stash id");
            }
        }

        let repo = Arc::new(Mutex::new(repo));
        let client = Arc::new(AsyncMutex::new(Client::new(USER_AGENT.to_owned())));

        let (tx, rx) = channel::<actix::System>();

        let handle = thread::spawn(move || {
            let system_runner = System::new();

            let system = System::try_current().expect("cant get thread system");

            tx.send(system).expect("cant send running system");

            system_runner.block_on(async {
                let actor = StashReceiverActor::new(repo.clone(), client.clone());

                let actor = actor.start();

                let timer = PublicStashTimer {
                    actor: actor.clone(),
                    interval: std::time::Duration::from_secs(
                        configuration.application.refresh_interval_secs,
                    ),
                    repo: repo.clone(),
                    client: client.clone(),
                };
                let _ = timer.start();
            });
            system_runner.run()
        });

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
}

fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let rpc = JsonrpcServer::new()
        .with_method("calculate_pob", calculate_pob)
        .finish();

    let rpc_get = JsonrpcServer::new()
        .with_method("get_build_price", get_build_price)
        .finish();

    let server = HttpServer::new(move || {
        let rpc = rpc.clone();
        let rpc_get = rpc_get.clone();
        App::new()
            .service(
                web::service("/api")
                    .guard(actix_web::guard::Post())
                    .finish(rpc.into_web_service()),
            )
            .service(
                web::service("/read")
                    .guard(actix_web::guard::Get())
                    .finish(rpc_get.into_web_service()),
            )
    })
    .listen(listener)?
    .run();

    Ok(server)
}
