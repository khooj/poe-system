use crate::storage::{
    postgresql::items::ItemRepository, ItemInsertTrait, LatestStashId, StashRepositoryTrait,
};
use domain::{build_calculation::stored_item::StoredItem, item::Item};
use public_stash::{client::Error as StashError, models::PublicStashData};
use tracing::{info, instrument, trace};

pub type PgStashReceiver = StashReceiver<ItemRepository>;

#[derive(thiserror::Error, Debug)]
pub enum StashReceiverError {
    #[error("client error")]
    ClientError(#[from] StashError),
    #[error("skipping this iteration")]
    Skip,
}

pub struct StashReceiver<T> {
    repository: T,
    only_leagues: Vec<String>,
}

impl<T> StashReceiver<T> {
    pub fn new(repository: T, only_leagues: Vec<String>) -> Self {
        StashReceiver {
            repository,
            only_leagues,
        }
    }
}

impl<T: ItemInsertTrait + StashRepositoryTrait> StashReceiver<T> {
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
                self.repository.clear_stash(stash).await?;
                continue;
            }

            let items = d
                .items
                .into_iter()
                .filter_map(|i| Item::try_from(i).ok())
                .filter_map(|i| StoredItem::try_from(i).ok())
                .collect::<Vec<_>>();
            self.repository.insert_items(items.clone(), stash).await?;
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
