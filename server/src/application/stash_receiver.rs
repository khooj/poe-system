use crate::infrastructure::{
    public_stash_retriever::Client, repositories::postgres::raw_item_repository::RawItemRepository,
};
use crate::interfaces::public_stash_retriever::Error;
use thiserror::Error;
use tracing::{debug, error, info, instrument, trace};

#[derive(Error, Debug)]
pub enum StashReceiverError {
    #[error("client error")]
    ClientError(#[from] Error),
    #[error("skipping this iteration")]
    Skip,
}

pub struct StashReceiver {
    repository: RawItemRepository,
    client: Client,
    only_leagues: Vec<String>,
}

impl StashReceiver {
    pub fn new(
        repository: RawItemRepository,
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
        let mut t = self.repository.begin().await?;
        let res = self.repository.get_stash_id(&mut t).await?;
        trace!("latest stash id from repo: {:?}", res.latest_stash_id);
        let mut k = self
            .client
            .get_latest_stash(Some(&res.latest_stash_id))
            .await?;
        trace!("received stash with next id: {}", k.next_change_id);
        if k.stashes.is_empty() {
            return Ok(());
        }

        if self.only_leagues.len() > 0 {
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
            let acc = d.account_name.as_ref().unwrap();
            let stash = d.stash.as_ref().unwrap();

            if d.items.len() == 0 {
                self.repository.delete_raw_item(&mut t, acc, stash).await?;
                continue;
            }

            for item in d.items {
                let id = item.id.clone();
                self.repository
                    .insert_raw_item(&mut t, id.as_ref().unwrap(), acc, stash, item)
                    .await?;
            }
        }
        self.repository
            .set_stash_id(&mut t, &k.next_change_id)
            .await?;
        t.commit().await?;
        info!(id = %k.next_change_id, "successfully received and inserted");
        Ok(())
    }

    #[instrument(err, skip(self))]
    pub async fn ensure_stash_id(&mut self, id: String) -> Result<(), anyhow::Error> {
        let mut t = self.repository.begin().await?;
        let r = self.repository.get_stash_id(&mut t).await;
        if r.is_err() {
            self.repository.set_stash_id(&mut t, &id).await?;
        }
        t.commit().await?;
        Ok(())
    }
}
