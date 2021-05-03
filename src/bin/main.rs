#[macro_use]
extern crate diesel_migrations;

use actix::prelude::*;
use diesel::{Connection, SqliteConnection};
use dotenv::dotenv;
use env_logger;
use log::error;
use poe_system::actors::stash_receiver::{StartReceiveMsg, StashReceiverActor};
use poe_system::implementations::{
    item_repository::DieselItemRepository, public_stash_retriever::Client,
};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex,
};
use std::{env, time::Duration};
use tokio::sync::Mutex as AsyncMutex;

const USER_AGENT: &str = "OAuth poe-system/0.0.1 (contact: bladoff@gmail.com)";

embed_migrations!("migrations");

#[actix::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    dotenv().ok().expect("cant load dotenv");

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })
    .expect("error setting ctrl-c handler");

    let db_url = env::var("DATABASE_URL").expect("cant get DATABASE_URL");
    let db_connection = SqliteConnection::establish(&db_url).expect("cannot establish sqlite conn");

    embedded_migrations::run(&db_connection).expect("cant run migration");

    let repo = DieselItemRepository::new(db_connection).expect("cant create repo");

    let client = Client::new(USER_AGENT.to_owned());

    let actor = StashReceiverActor::new(
        Arc::new(Mutex::new(repo)),
        Arc::new(AsyncMutex::new(client)),
    );

    let actor = actor.start();

    while running.load(Ordering::SeqCst) {
        match actor.try_send(StartReceiveMsg) {
            Err(e) => error!("cant send receive msg to actor: {}", e),
            Ok(_) => {}
        };
        actix::clock::sleep(Duration::from_secs(1)).await;
    }

    System::current().stop();
    Ok(())
}
