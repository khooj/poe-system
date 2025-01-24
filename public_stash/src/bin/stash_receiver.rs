use public_stash::client::{Client, Error as StashError};
use storage::{DynItemRepository, LatestStashId};
use thiserror::Error;
use tracing::{error, info, instrument, trace};

#[derive(Error, Debug)]
pub enum StashReceiverError {
    #[error("client error")]
    ClientError(#[from] StashError),
    #[error("skipping this iteration")]
    Skip,
}

pub struct StashReceiver {
    repository: DynItemRepository,
    client: Client,
    only_leagues: Vec<String>,
}

impl StashReceiver {
    pub fn new(
        repository: DynItemRepository,
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
        let res = self.repository.get_stash_id().await?;
        trace!("latest stash id from repo: {}", res.id);
        let mut k = self.client.get_latest_stash(Some(&res.id)).await?;
        trace!("received stash with next id: {}", k.next_change_id);
        if k.stashes.is_empty() {
            return Ok(());
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
            let acc = d.account_name.as_ref().unwrap();
            let stash = d.stash.as_ref().unwrap();

            if d.items.is_empty() {
                self.repository.delete_item(acc, stash).await?;
                continue;
            }

            // doesnt work
            // let items = d
            //     .items
            //     .into_iter()
            //     .filter_map(|m| Item::try_from(m).ok())
            //     .collect();
            // TODO: logic for filtering items
            self.repository.insert_items(vec![]).await?;
            // for item in d.items {
            //     // we cant be sure that every item have unique id
            //     // so we generate it themselves
            //     let id = Uuid::new_v4().to_string();
            //     self.repository
            //         .insert_item(&mut t, RawItem::new(&id, acc, stash, item))
            //         .await?;
            // }
        }
        self.repository
            .set_stash_id(LatestStashId {
                id: k.next_change_id.clone(),
            })
            .await?;
        info!(id = %k.next_change_id, "successfully received and inserted");
        Ok(())
    }
}

fn main() {}
