pub mod public_stash_timer;
pub mod stash_receiver;

use crate::raw_items::infrastructure::repositories::mongo::items_repository::DbItem;
use actix::Message;

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
