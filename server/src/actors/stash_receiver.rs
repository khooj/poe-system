use crate::implementations::{public_stash_retriever::Client, ItemsRepository};
use crate::ports::outbound::{public_stash_retriever::Error, repository::RepositoryError};
use actix::prelude::*;
use thiserror::Error;
use tracing::{error, event, info, instrument, Level};

#[derive(Error, Debug)]
pub enum ActorError {
    #[error("repo error")]
    RepoError(#[from] RepositoryError),
    #[error("client error")]
    ClientError(#[from] Error),
    #[error("skipping this iteration")]
    Skip,
}

pub struct StashReceiverActor {
    repository: ItemsRepository,
    client: Client,
}

impl StashReceiverActor {
    pub fn new(repository: ItemsRepository, client: Client) -> StashReceiverActor {
        StashReceiverActor { repository, client }
    }
}

impl Actor for StashReceiverActor {
    type Context = SyncContext<Self>;
}

#[derive(Message)]
#[rtype(result = "Result<(), anyhow::Error>")]
pub struct StartReceiveMsg;

impl Handler<StartReceiveMsg> for StashReceiverActor {
    type Result = Result<(), anyhow::Error>;

    #[instrument(err, skip(self))]
    fn handle(&mut self, _: StartReceiveMsg, _: &mut Self::Context) -> Self::Result {
        let res = self.repository.get_stash_id()?;
        info!("latest stash id from repo: {:?}", res.latest_stash_id);
        let k = self
            .client
            .get_latest_stash(res.latest_stash_id.as_deref())?;
        info!("received stash with next id: {}", k.next_change_id);
        self.repository.insert_raw_item(&k)?;
        event!(Level::INFO, "successfully inserted");
        Ok(())
    }
}
