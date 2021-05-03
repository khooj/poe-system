use crate::implementations::{
    item_repository::DieselItemRepository, public_stash_retriever::Client,
};
use crate::ports::outbound::{
    public_stash_retriever::{Error, PublicStashData, Retriever},
    repository::{ItemRepository, RepositoryError},
};
use actix::prelude::*;
use log::error;
use std::sync::Arc;
use std::sync::Mutex;
use thiserror::Error;
use tokio::task;

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
    client: Arc<Mutex<Client>>,
}

impl StashReceiverActor {
    pub fn new(
        repo: Arc<Mutex<DieselItemRepository>>,
        client: Arc<Mutex<Client>>,
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

    fn handle(&mut self, msg: StartReceiveMsg, ctx: &mut Self::Context) -> Self::Result {
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

    fn handle(&mut self, msg: GetStashMsg, ctx: &mut Self::Context) -> Self::Result {
        println!("started 2");
        let cl = Arc::clone(&self.client);
        Box::pin(
            async move {
                let id = msg.0;
                match cl.lock().unwrap().get_latest_stash(id.as_deref()).await {
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

    fn handle(&mut self, msg: ReceivedStash, ctx: &mut Self::Context) -> Self::Result {
        println!("started 3");
        match self.repository.lock().unwrap().insert_raw_item(msg.0) {
            Err(e) => {
                error!("cant insert items: {}", e);
            }
            _ => {}
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

    #[actix::test]
    async fn run_actor() -> Result<(), anyhow::Error> {
        let conn = diesel::SqliteConnection::establish(":memory:")?;
        embedded_migrations::run(&conn)?;

        let repo = Arc::new(Mutex::new(DieselItemRepository::new(conn)?));
        let client = Arc::new(Mutex::new(Client::new(
            "OAuth poe-system/0.0.1 (contact: bladoff@gmail.com)".to_owned(),
        )));
        let actor = StashReceiverActor {
            client: Arc::clone(&client),
            repository: Arc::clone(&repo),
        };

        let actor = actor.start();

        actor.try_send(StartReceiveMsg)?;
        tokio::time::sleep(std::time::Duration::from_secs(5)).await;

        let stash_id = repo.lock().unwrap().get_stash_id()?;
        assert_ne!(stash_id.latest_stash_id, None);
        Ok(())
    }
}
