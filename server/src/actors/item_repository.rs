use crate::define_repo_method;
use crate::domain::item::Item;
use crate::implementations::ItemsRepository;
use crate::ports::outbound::public_stash_retriever::PublicStashData;
use crate::ports::outbound::repository::{LatestStashId, RepositoryError};
use actix::prelude::*;

pub struct ItemsRepositoryActor {
    pub repo: ItemsRepository,
}

impl Actor for ItemsRepositoryActor {
    type Context = SyncContext<Self>;
}

#[derive(Message)]
#[rtype(result = "Result<Vec<Item>, RepositoryError>")]
pub struct GetItemsByBasetype {
    pub base_type: String,
}

define_repo_method! {
    ItemsRepositoryActor,
    GetItemsByBasetype,
    Result<Vec<Item>, RepositoryError>,
    get_items_by_basetype, base_type
}

#[derive(Message)]
#[rtype(result = "Result<LatestStashId, RepositoryError>")]
pub struct GetStashId {}

define_repo_method! {
    ItemsRepositoryActor,
    GetStashId,
    Result<LatestStashId, RepositoryError>,
    get_stash_id,
}

#[derive(Message)]
#[rtype(result = "Result<(), RepositoryError>")]
pub struct SetStashId {
    pub id: String,
}

define_repo_method! {
    ItemsRepositoryActor,
    SetStashId,
    Result<(), RepositoryError>,
    set_stash_id, id
}

#[derive(Message)]
#[rtype(result = "Result<(), RepositoryError>")]
pub struct InsertRawItem {
    pub data: PublicStashData,
}

define_repo_method! {
    ItemsRepositoryActor,
    InsertRawItem,
    Result<(), RepositoryError>,
    insert_raw_item, data
}
