use std::time::Duration;
use application::{
    stash_receiver::StashReceiver,
    storage::{postgresql::ItemRepository, redis::RedisIndexOptions},
};
use clap::{Parser, Subcommand};
use public_stash::{
    client::{Client, Error as StashError},
    models::PublicStashData,
};
use serde::Deserialize;
use utils::DEFAULT_USER_AGENT;
use metrics::{histogram, counter};
use tokio::time::Instant;

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

async fn launch_with_dir(mut receiver: StashReceiver, dir: &str) -> anyhow::Result<()> {
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

async fn launch_with_api(mut receiver: StashReceiver) -> anyhow::Result<()> {
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
    redis: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let settings: Settings = config::Config::builder()
        .add_source(config::File::with_name("config").format(config::FileFormat::Toml).required(false))
        .add_source(config::Environment::with_prefix("APP"))
        .build()?
        .try_deserialize()?;

    let receiver = StashReceiver::new(
        ItemRepository::new(&settings.pg).await?,
        RedisIndexOptions::default().set_uri(&settings.redis).build()?,
        vec![],
    );
    match settings.mode {
        Command::Stash => {} //launch_with_api(receiver).await?,
        Command::Directory { dir } => launch_with_dir(receiver, dir.as_ref()).await?,
    };
    Ok(())
}
