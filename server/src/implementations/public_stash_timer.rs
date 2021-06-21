use actix::prelude::*;

use crate::actors::stash_receiver::{StartReceiveMsg, StashReceiverActor};

pub struct PublicStashTimer {
    pub interval: std::time::Duration,
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
