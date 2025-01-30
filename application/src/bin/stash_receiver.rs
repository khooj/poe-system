use std::time::Duration;

use application::{
    storage::{
        postgresql::ItemRepository,
        redis::{RedisIndexOptions, RedisIndexRepository},
        DynItemRepository, LatestStashId,
    },
    typed_item::TypedItem,
};
use clap::{Parser, Subcommand};
use public_stash::{
    client::{Client, Error as StashError},
    models::PublicStashData,
};
use thiserror::Error;
use tracing::{error, info, instrument, trace};
use utils::DEFAULT_USER_AGENT;

#[derive(Error, Debug)]
pub enum StashReceiverError {
    #[error("client error")]
    ClientError(#[from] StashError),
    #[error("skipping this iteration")]
    Skip,
}

pub struct StashReceiver {
    repository: DynItemRepository,
    redis_index: RedisIndexRepository,
    only_leagues: Vec<String>,
}

impl StashReceiver {
    pub fn new(
        repository: DynItemRepository,
        redis_index: RedisIndexRepository,
        only_leagues: Vec<String>,
    ) -> StashReceiver {
        StashReceiver {
            repository,
            redis_index,
            only_leagues,
        }
    }
}

impl StashReceiver {
    pub async fn get_latest_stash(&mut self) -> Result<LatestStashId, anyhow::Error> {
        Ok(self.repository.get_stash_id().await?)
    }

    #[instrument(err, skip(self))]
    pub async fn receive(
        &mut self,
        mut k: PublicStashData,
    ) -> Result<Option<String>, anyhow::Error> {
        if k.stashes.is_empty() {
            return Ok(self.repository.get_stash_id().await.map(|ls| ls.id)?);
        }

        if !self.only_leagues.is_empty() {
            k.stashes.retain(|el| {
                self.only_leagues
                    .contains(el.league.as_ref().unwrap_or(&String::new()))
            });
        }

        for d in k.stashes {
            if d.account_name.is_none() || d.stash.is_none() {
                trace!("skipping stash because of empty account name or stash");
                continue;
            }
            let stash = d.stash.as_ref().unwrap();

            if d.items.is_empty() {
                let ids = self.repository.clear_stash(stash).await?;
                self.redis_index.delete_items(ids).await?;
                continue;
            }

            let items = d
                .items
                .into_iter()
                .filter_map(|i| TypedItem::try_from(i).ok())
                .collect::<Vec<_>>();
            self.repository.insert_items(items.clone(), stash).await?;

            self.redis_index.insert_items(items).await?;
        }
        self.repository
            .set_stash_id(LatestStashId {
                id: Some(k.next_change_id.clone()),
            })
            .await?;
        info!(id = %k.next_change_id, "successfully received and inserted");
        Ok(if k.next_change_id.is_empty() {
            None
        } else {
            Some(k.next_change_id)
        })
    }
}

#[derive(Parser)]
#[command()]
struct Args {
    dsn: String,
    redis: String,

    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Stash,
    Directory { dir: String },
}

async fn launch_with_dir(mut receiver: StashReceiver, dir: &str) -> anyhow::Result<()> {
    let stashes = utils::stream_stashes::open_stashes(dir);

    for (_, content) in stashes {
        let data: PublicStashData = serde_json::from_str(&content)?;
        receiver.receive(data).await?;
    }

    Ok(())
}

async fn launch_with_api(mut receiver: StashReceiver) -> anyhow::Result<()> {
    let mut client = Client::new(DEFAULT_USER_AGENT);

    let latest_stash = receiver.get_latest_stash().await?;
    let mut latest_stash = latest_stash.id;

    loop {
        match client
            .get_latest_stash(latest_stash.clone())
            .await
        {
            Ok(stash) => {
                latest_stash = receiver.receive(stash).await?;
            }
            Err(StashError::TooManyRequests) | Err(StashError::NextCycle) => {
                tokio::time::sleep(Duration::from_secs(5)).await;
            }
            e => e.map(|_| ())?,
        };
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let receiver = StashReceiver::new(
        ItemRepository::new(&args.dsn).await?,
        RedisIndexOptions::default().set_uri(&args.redis).build()?,
        vec![],
    );
    match args.command {
        Command::Stash => launch_with_api(receiver).await?,
        Command::Directory { dir } => launch_with_dir(receiver, dir.as_ref()).await?,
    };
    Ok(())
}
