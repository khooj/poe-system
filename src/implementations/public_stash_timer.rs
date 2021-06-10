use actix::prelude::*;

use crate::actors::stash_receiver::{StartReceiveMsg, StashReceiverActor};
use crate::implementations::item_repository::DieselItemRepository;
use crate::implementations::public_stash_retriever::Client;

use std::sync::{Arc, Mutex};
use tokio::sync::Mutex as AsyncMutex;

pub struct PublicStashTimer {
    pub interval: std::time::Duration,
    pub repo: Arc<Mutex<DieselItemRepository>>,
    pub client: Arc<AsyncMutex<Client>>,
    pub actor: Addr<StashReceiverActor>,
}

impl Actor for PublicStashTimer {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        ctx.run_interval(self.interval, move |act, _ctx| {
            let _ = act.actor.do_send(StartReceiveMsg);
        });
    }
}
