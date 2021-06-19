use actix::prelude::*;
use crate::{define_repo_method, implementations::item_repository::{DieselItemRepository, RawItem}};
use crate::ports::outbound::repository::{RepositoryError, LatestStashId};
use crate::domain::item::Item;

pub struct ItemsRepositoryActor {
    pub repo: DieselItemRepository,
}

impl Actor for ItemsRepositoryActor {
    type Context = Context<Self>;
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
pub struct GetStashId {
}

define_repo_method! {
    ItemsRepositoryActor,
    GetStashId,
    Result<LatestStashId, RepositoryError>,
    get_stash_id,
}