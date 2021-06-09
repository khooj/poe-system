use crate::application::configuration::Settings;
use crate::implementations::http_controller::{calculate_pob, get_build_price};
use crate::{
    actors::stash_receiver::StashReceiverActor,
    implementations::item_repository::DieselItemRepository,
    implementations::public_stash_retriever::Client,
    implementations::public_stash_timer::PublicStashTimer,
};

use actix::Actor;
use actix_web::{dev::Server, web, App, HttpServer};
use diesel::{Connection, SqliteConnection};
use jsonrpc_v2::Server as JsonrpcServer;
use std::net::TcpListener;
use std::sync::{Arc, Mutex};
use tokio::sync::Mutex as AsyncMutex;

const USER_AGENT: &str = "OAuth poe-system/0.0.1 (contact: bladoff@gmail.com)";

pub struct Application {
    server: Server,
}

embed_migrations!("migrations");

impl Application {
    pub async fn build(configuration: Settings) -> Result<Self, std::io::Error> {
        env_logger::init();

        let addr = format!("{}:{}", "0.0.0.0", 3000);
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

        Ok(Self { server })
    }

    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}

fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let rpc = JsonrpcServer::new()
        .with_method("calculate_pob", calculate_pob)
        .with_method("get_build_price", get_build_price)
        .finish();

    let server = HttpServer::new(move || {
        let rpc = rpc.clone();
        App::new().service(
            web::service("/api")
                .guard(actix_web::guard::Post())
                .finish(rpc.into_web_service()),
        )
    })
    .listen(listener)?
    .run();

    Ok(server)
}
