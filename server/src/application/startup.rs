use crate::{
    actors::build_calculator::BuildCalculatorActor, application::configuration::Settings,
    ports::outbound::repository,
};
use crate::{
    actors::stash_receiver::StashReceiverActor,
    implementations::item_repository::DieselItemRepository,
    implementations::public_stash_retriever::Client,
    implementations::public_stash_timer::PublicStashTimer,
};
use crate::{
    actors::{builds_repository::BuildsRepositoryActor, item_repository::ItemsRepositoryActor},
    implementations::{
        builds_repository::DieselBuildsRepository,
        http_controller::{calculate_pob, get_build_price},
    },
};

use super::connection_pool::ConnectionPool;
use actix::prelude::*;
use actix_web::{dev::Server, web, App, HttpServer};
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::Connection;
use jsonrpc_v2::{Data, Server as JsonrpcServer};
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

impl Application {
    pub async fn build(configuration: Settings) -> Result<Self, std::io::Error> {
        env_logger::init();

        let manager = diesel::r2d2::ConnectionManager::<diesel::SqliteConnection>::new(
            &configuration.database,
        );
        let pool = Pool::new(manager).expect("cant create diesel pool");
        let pool = ConnectionPool { pool };

        let repo = DieselItemRepository::new(pool.clone()).expect("cant create item repository");

        if configuration.start_change_id.is_some() {
            let stash = repo.get_stash_id().expect("cant get latest stash id");
            if stash.latest_stash_id.is_none() {
                repo.set_stash_id(configuration.start_change_id.as_ref().unwrap())
                    .expect("cant set stash id");
            }
        }

        let build_repo = DieselBuildsRepository { conn: pool.clone() };
        let client = Arc::new(AsyncMutex::new(Client::new(USER_AGENT.to_owned())));

        let (tx, rx) = channel::<actix::System>();
        let (tx2, rx2) = channel::<Addr<BuildCalculatorActor>>();

        let addr = format!(
            "{}:{}",
            configuration.application.host, configuration.application.port
        );

        let handle = thread::spawn(move || {
            let system_runner = System::new();

            let system = System::try_current().expect("cant get thread system");

            tx.send(system).expect("cant send running system");

            system_runner.block_on(async {
                let repo = ItemsRepositoryActor { repo }.start();
                let build_repo = BuildsRepositoryActor { repo: build_repo }.start();
                let actor = StashReceiverActor::new(repo.clone(), client.clone());

                let actor = actor.start();

                let timer = PublicStashTimer {
                    actor: actor.clone(),
                    interval: std::time::Duration::from_secs(
                        configuration.application.refresh_interval_secs,
                    ),
                };
                let _ = timer.start();

                let t = BuildCalculatorActor {
                    item_repo: repo.clone(),
                    repo: build_repo.clone(),
                }
                .start();

                tx2.send(t.clone()).expect("cant send actor");
            });
            system_runner.run()
        });

        let build_actor = rx2.recv().expect("cant get actor");

        let listener = TcpListener::bind(&addr)?;
        let server = run(listener, build_actor)?;

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

fn run(listener: TcpListener, addr: Addr<BuildCalculatorActor>) -> Result<Server, std::io::Error> {
    let rpc = JsonrpcServer::new()
        .with_data(Data::new(addr))
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
