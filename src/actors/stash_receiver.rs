use actix::prelude::*;

#[derive(Message)]
#[rtype(result = "()")]
pub struct StashReceiverMsg;

pub struct StashReceiverActor {}

impl Actor for StashReceiverActor {
    type Context = Context<Self>;
}

impl Handler<StashReceiverMsg> for StashReceiverActor {
    type Result = ();

    fn handle(&mut self, msg: StashReceiverMsg, ctx: &mut Self::Context) -> Self::Result {
        unimplemented!();
    }
}
