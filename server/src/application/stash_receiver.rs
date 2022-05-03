use crate::infrastructure::{
    public_stash_retriever::Client, repositories::mongo::items_repository::ItemsRepository,
};
use crate::interfaces::public_stash_retriever::Error;
use std::time::{Duration, SystemTime};
use thiserror::Error;
use tracing::{error, event, info, instrument, Level};

#[derive(Error, Debug)]
pub enum StashReceiverError {
    #[error("client error")]
    ClientError(#[from] Error),
    #[error("skipping this iteration")]
    Skip,
}

pub struct StashReceiver {
    repository: ItemsRepository,
    client: Client,
    only_leagues: Vec<String>,
}

impl StashReceiver {
    pub fn new(
        repository: ItemsRepository,
        client: Client,
        only_leagues: Vec<String>,
    ) -> StashReceiver {
        StashReceiver {
            repository,
            client,
            only_leagues,
        }
    }
}

impl StashReceiver {
    #[instrument(err, skip(self))]
    pub async fn receive(&mut self) -> Result<(), anyhow::Error> {
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap();
        info!("handling message: {}", now.as_secs());
        let res = self.repository.get_stash_id().await?;
        info!("latest stash id from repo: {:?}", res.latest_stash_id);
        // TODO: make async
        let mut k = self.client.get_latest_stash(Some(&res.latest_stash_id))?;
        info!("received stash with next id: {}", k.next_change_id);
        if self.only_leagues.len() > 0 {
            k.stashes = k
                .stashes
                .into_iter()
                .filter(|el| {
                    self.only_leagues
                        .iter()
                        .any(|l| l == el.league.as_ref().unwrap_or(&String::new()))
                })
                .collect();
        }
        self.repository.insert_raw_item(k).await?;
        event!(Level::INFO, "successfully inserted");
        Ok(())
    }
}
