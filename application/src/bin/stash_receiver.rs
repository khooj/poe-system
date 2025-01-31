use application::{
    stash_receiver::{PgStashReceiver, StashReceiver},
    storage::postgresql::items::ItemRepository,
};
use clap::{Parser, Subcommand};
use metrics::histogram;
use public_stash::{
    client::{Client, Error as StashError},
    models::PublicStashData,
};
use serde::Deserialize;
use sqlx::postgres::PgPoolOptions;
use std::time::Duration;
use tokio::time::Instant;
use utils::DEFAULT_USER_AGENT;

#[derive(Parser)]
#[command()]
struct Args {
    dsn: String,
    redis: String,

    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug, Deserialize)]
enum Command {
    Stash,
    Directory { dir: String },
}

async fn launch_with_dir(mut receiver: PgStashReceiver, dir: &str) -> anyhow::Result<()> {
    let stashes = utils::stream_stashes::open_stashes(dir);

    for (_, content) in stashes {
        let start = Instant::now();
        let data: PublicStashData = serde_json::from_str(&content)?;
        receiver.receive(data).await?;
        let delta = start.elapsed();
        histogram!("stash_receiver.dir.time").record(delta);
    }

    Ok(())
}

#[allow(unused)]
async fn launch_with_api(mut receiver: PgStashReceiver) -> anyhow::Result<()> {
    let mut client = Client::new(DEFAULT_USER_AGENT);

    let latest_stash = receiver.get_latest_stash().await?;
    let mut latest_stash = latest_stash.id;

    loop {
        let start = Instant::now();
        match client.get_latest_stash(latest_stash.clone()).await {
            Ok(stash) => {
                latest_stash = receiver.receive(stash).await?;
            }
            Err(StashError::TooManyRequests) | Err(StashError::NextCycle) => {
                tokio::time::sleep(Duration::from_secs(5)).await;
            }
            e => e.map(|_| ())?,
        };
        let delta = start.elapsed();
        histogram!("stash_receiver.stash_api.time").record(delta);
    }
}

#[derive(Deserialize, Debug)]
struct Settings {
    mode: Command,
    pg: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let settings: Settings = config::Config::builder()
        .add_source(
            config::File::with_name("config")
                .format(config::FileFormat::Toml)
                .required(false),
        )
        .add_source(config::Environment::with_prefix("APP"))
        .build()?
        .try_deserialize()?;

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&settings.pg)
        .await?;
    let receiver = StashReceiver::new(ItemRepository::new(pool).await?, vec![]);
    match settings.mode {
        Command::Stash => {} //launch_with_api(receiver).await?,
        Command::Directory { dir } => launch_with_dir(receiver, dir.as_ref()).await?,
    };
    Ok(())
}
