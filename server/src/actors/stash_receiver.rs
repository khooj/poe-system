use crate::implementations::{public_stash_retriever::Client, ItemsRepository};
use crate::ports::outbound::{
    public_stash_retriever::{Error, PublicStashData, Retriever},
    repository::RepositoryError,
};
use actix::prelude::*;
use std::sync::Arc;
use thiserror::Error;
use tokio::sync::Mutex as AsyncMutex;
use tracing::{error, event, info, instrument, span, Instrument, Level};

#[derive(Error, Debug)]
pub enum ActorError {
    #[error("repo error")]
    RepoError(#[from] RepositoryError),
    #[error("client error")]
    ClientError(#[from] Error),
    #[error("skipping this iteration")]
    Skip,
}

// TODO: should work on different thread
// but cant do it now because of async client
pub struct StashReceiverActor {
    repository: ItemsRepository,
    client: Arc<AsyncMutex<Client>>,
}

impl StashReceiverActor {
    pub fn new(repository: ItemsRepository, client: Arc<AsyncMutex<Client>>) -> StashReceiverActor {
        StashReceiverActor { repository, client }
    }
}

impl Actor for StashReceiverActor {
    type Context = Context<Self>;
}

#[derive(Message)]
#[rtype(result = "Result<(), anyhow::Error>")]
pub struct StartReceiveMsg;

impl Handler<StartReceiveMsg> for StashReceiverActor {
    type Result = Result<(), anyhow::Error>;

    #[instrument(err, skip(self, ctx))]
    fn handle(&mut self, _: StartReceiveMsg, ctx: &mut Self::Context) -> Self::Result {
        if self.client.try_lock().is_err() {
            event!(Level::INFO, "locked, skipping iteration");
            return Ok(());
        }
        let res = self.repository.get_stash_id()?;
        ctx.notify(GetStashMsg(res.latest_stash_id));
        Ok(())
    }
}

#[derive(Message)]
#[rtype(result = "()")]
struct GetStashMsg(Option<String>);

impl Handler<GetStashMsg> for StashReceiverActor {
    type Result = ResponseActFuture<Self, ()>;

    fn handle(&mut self, msg: GetStashMsg, _: &mut Self::Context) -> Self::Result {
        let cl = Arc::clone(&self.client);
        let span = span!(Level::INFO, "StashReceiverActor(GetStashMsg)");
        Box::pin(
            async move {
                info!("latest stash id from repo: {:?}", msg.0);
                let id = msg.0;
                let mut lock = cl.lock().await;
                match lock.get_latest_stash(id.as_deref()).await {
                    Err(e) => {
                        error!("cant get latest stash: {}", e);
                        return Err(ActorError::Skip);
                    }
                    Ok(k) => Ok(k),
                }
            }
            .instrument(span)
            .into_actor(self)
            .map(|res, _act, ctx| {
                if res.is_err() {
                    return;
                }
                ctx.notify(ReceivedStash(res.unwrap()));
            }),
        )
    }
}

#[derive(Message)]
#[rtype(result = "Result<(), anyhow::Error>")]
struct ReceivedStash(PublicStashData);

impl Handler<ReceivedStash> for StashReceiverActor {
    type Result = Result<(), anyhow::Error>;

    #[instrument(err, skip(self, msg))]
    fn handle(&mut self, msg: ReceivedStash, _: &mut Self::Context) -> Self::Result {
        info!("received stash with next id: {}", msg.0.next_change_id);
        self.repository.insert_raw_item(&msg.0)?;
        event!(Level::INFO, "successfully inserted");
        Ok(())
    }
}

#[cfg(test)]
mod test {
    // use diesel::Connection;

    // use super::*;
    // use crate::implementations::{
    //     item_repository::DieselItemRepository, public_stash_retriever::Client,
    // };

    // TODO: mock db or client to remove ugly wait
    // #[actix::test]
    // async fn run_actor() -> Result<(), anyhow::Error> {
    //     let conn = diesel::SqliteConnection::establish(":memory:")?;
    //     // embedded_migrations::run(&conn)?;

    //     let repo = DieselItemRepository::new(conn)?;
    //     let client = Arc::new(AsyncMutex::new(Client::new(
    //         "OAuth poe-system/0.0.1 (contact: bladoff@gmail.com)".to_owned(),
    //     )));
    //     let repo = ItemsRepositoryActor { repo }.start();
    //     let actor = StashReceiverActor {
    //         client: Arc::clone(&client),
    //         repository: repo.clone(),
    //     };

    //     let actor = actor.start();

    //     actor.try_send(StartReceiveMsg)?;
    //     tokio::time::sleep(std::time::Duration::from_secs(10)).await;

    //     let stash_id = repo.send(GetStashId {}).await??;
    //     assert_ne!(stash_id.latest_stash_id, None);
    //     Ok(())
    // }
}
