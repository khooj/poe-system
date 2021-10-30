pub mod stash_receiver;

use actix::Message;
use crate::implementations::DbItem;

#[derive(Message)]
#[rtype(result = "()")]
pub struct NewItem {
    pub item: DbItem,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct DeleteItem {
    pub id: String,
}