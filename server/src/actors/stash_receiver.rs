use crate::implementations::{
    item_repository::DieselItemRepository, public_stash_retriever::Client,
};
use crate::ports::outbound::{
    public_stash_retriever::{Error, PublicStashData, Retriever},
    repository::RepositoryError,
};
use actix::prelude::*;
use log::{error, info};
use std::sync::{Arc, Mutex};
use thiserror::Error;
use tokio::sync::Mutex as AsyncMutex;

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
    repository: Arc<Mutex<DieselItemRepository>>,
    client: Arc<AsyncMutex<Client>>,
}

impl StashReceiverActor {
    pub fn new(
        repo: Arc<Mutex<DieselItemRepository>>,
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
    type Result = ();

    fn handle(&mut self, _: StartReceiveMsg, ctx: &mut Self::Context) -> Self::Result {
        if self.client.try_lock().is_err() {
            info!("locked, skipping iteration");
            return;
        }
        let stash_id = match self.repository.lock().unwrap().get_stash_id() {
            Ok(s) => s,
            Err(e) => {
                error!("cant get stash id: {}", e);
                return ();
            }
        };

        ctx.notify(GetStashMsg(stash_id.latest_stash_id));
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
    type Result = ();

    fn handle(&mut self, msg: ReceivedStash, _: &mut Self::Context) -> Self::Result {
        info!("received stash with next id: {}", msg.0.next_change_id);
        match self.repository.lock().unwrap().insert_raw_item(msg.0) {
            Err(e) => {
                error!("cant insert items: {}", e);
            }
            _ => {
                info!("successfully inserted");
            }
        };
    }
}

#[cfg(test)]
mod test {
    use diesel::Connection;

    use super::*;
    use crate::implementations::{
        item_repository::DieselItemRepository, public_stash_retriever::Client,
    };

    embed_migrations!("migrations");

    // TODO: mock db or client to remove ugly wait
    // #[actix::test]
    async fn run_actor() -> Result<(), anyhow::Error> {
        let conn = diesel::SqliteConnection::establish(":memory:")?;
        embedded_migrations::run(&conn)?;

        let repo = Arc::new(Mutex::new(DieselItemRepository::new(conn)?));
        let client = Arc::new(AsyncMutex::new(Client::new(
            "OAuth poe-system/0.0.1 (contact: bladoff@gmail.com)".to_owned(),
        )));
        let actor = StashReceiverActor {
            client: Arc::clone(&client),
            repository: Arc::clone(&repo),
        };

        let actor = actor.start();

        actor.try_send(StartReceiveMsg)?;
        tokio::time::sleep(std::time::Duration::from_secs(10)).await;

        let stash_id = repo.lock().unwrap().get_stash_id()?;
        assert_ne!(stash_id.latest_stash_id, None);
        Ok(())
    }
}
