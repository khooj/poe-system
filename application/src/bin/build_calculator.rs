use application::{stash_receiver::StashReceiver, storage::{postgresql::ItemRepository, DynItemRepository}};
use metrics::histogram;
use public_stash::{
    client::{Client, Error as StashError},
    models::PublicStashData,
};
use serde::Deserialize;
use std::time::Duration;
use tokio::time::Instant;
use utils::DEFAULT_USER_AGENT;

async fn process_builds(items_repo: DynItemRepository, redis_cl: redis::Client) ->  anyhow::Result<()> {
    Ok(())
}

#[derive(Deserialize, Debug)]
struct Settings {
    pg: String,
    redis: String,
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

    let item_repo = ItemRepository::new(&settings.pg).await?;
    let redis_cl = redis::Client::open(settings.redis)?;
    process_builds(item_repo, redis_cl).await?;
    Ok(())
}
