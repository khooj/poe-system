use crate::implementations::public_stash_retriever::Client;
use crate::ports::outbound::{
    public_stash_retriever::{Error, PublicStashData, Retriever},
    repository::RepositoryError,
};
use actix::prelude::*;
use if_chain::if_chain;
use log::{error, info};
use std::sync::Arc;
use thiserror::Error;
use tokio::sync::Mutex as AsyncMutex;

use super::item_repository::{GetStashId, InsertRawItem, ItemsRepositoryActor};

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
    repository: Addr<ItemsRepositoryActor>,
    client: Arc<AsyncMutex<Client>>,
}

impl StashReceiverActor {
    pub fn new(
        repo: Addr<ItemsRepositoryActor>,
        client: Arc<AsyncMutex<Client>>,
    ) -> StashReceiverActor {
        StashReceiverActor {
            repository: repo,
            client,
        }
    }
}

impl Actor for StashReceiverActor {
    type Context = Context<Self>;
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct StartReceiveMsg;

impl Handler<StartReceiveMsg> for StashReceiverActor {
    type Result = ResponseActFuture<Self, ()>;

    fn handle(&mut self, _: StartReceiveMsg, _ctx: &mut Self::Context) -> Self::Result {
        let r = self.repository.clone();
        let c = Arc::clone(&self.client);
        Box::pin(
            async move {
                if c.try_lock().is_err() {
                    info!("locked, skipping iteration");
                    return Err(RepositoryError::Skipped);
                }
                match r.send(GetStashId {}).await {
                    Ok(k) => k,
                    Err(_) => Err(RepositoryError::Skipped),
                }
            }
            .into_actor(self)
            .map(|res, _act, ctx| {
                if_chain! {
                    if let Ok(stash_id) = res;
                    then {
                        ctx.notify(GetStashMsg(stash_id.latest_stash_id));
                    }
                };
            }),
        )
    }
}

#[derive(Message)]
#[rtype(result = "()")]
struct GetStashMsg(Option<String>);

impl Handler<GetStashMsg> for StashReceiverActor {
    type Result = ResponseActFuture<Self, ()>;

    fn handle(&mut self, msg: GetStashMsg, _: &mut Self::Context) -> Self::Result {
        let cl = Arc::clone(&self.client);
        info!("latest stash id from repo: {:?}", msg.0);
        Box::pin(
            async move {
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
#[rtype(result = "()")]
struct ReceivedStash(PublicStashData);

impl Handler<ReceivedStash> for StashReceiverActor {
    type Result = ResponseActFuture<Self, ()>;

    fn handle(&mut self, msg: ReceivedStash, _: &mut Self::Context) -> Self::Result {
        let r = self.repository.clone();
        Box::pin(
            async move {
                info!("received stash with next id: {}", msg.0.next_change_id);
                r.send(InsertRawItem { data: msg.0 }).await
            }
            .into_actor(self)
            .map(|res, _act, _ctx| {
                match res {
                    Err(e) => {
                        error!("cant insert items: {}", e);
                    }
                    _ => {
                        info!("successfully inserted");
                    }
                };
            }),
        )
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
