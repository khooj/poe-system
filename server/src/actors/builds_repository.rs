use crate::implementations::{builds_repository::{BuildsRepositoryError, DieselBuildsRepository}, models::NewBuildMatch as NewBuildMatchRepo};
use crate::implementations::models::*;
use actix::prelude::*;

use crate::define_repo_method;

pub struct BuildsRepositoryActor {
    pub repo: DieselBuildsRepository,
}

impl Actor for BuildsRepositoryActor {
    type Context = Context<Self>;
}

#[derive(Message)]
#[rtype(result = "Result<String, BuildsRepositoryError>")]
pub struct SaveNewBuild {
    pub pob: String,
    pub itemset: String,
}

define_repo_method! {
    BuildsRepositoryActor,
    SaveNewBuild,
    Result<String, BuildsRepositoryError>,
    save_new_build, pob, itemset
}

#[derive(Message)]
#[rtype(result = "Result<PobBuild, BuildsRepositoryError>")]
pub struct GetBuild {
    pub id: String,
}

define_repo_method! {
    BuildsRepositoryActor,
    GetBuild,
    Result<PobBuild, BuildsRepositoryError>,
    get_build, id
}

#[derive(Message)]
#[rtype(result = "Result<(), BuildsRepositoryError>")]
pub struct UpdateBuild {
    pub build: PobBuild,
}

define_repo_method! {
    BuildsRepositoryActor,
    UpdateBuild,
    Result<(), BuildsRepositoryError>,
    update_build, build
}

#[derive(Message)]
#[rtype(result = "Result<(), BuildsRepositoryError>")]
pub struct NewBuildMatch {
    pub mtch: NewBuildMatchRepo,
}

define_repo_method! {
    BuildsRepositoryActor,
    NewBuildMatch,
    Result<(), BuildsRepositoryError>,
    new_build_match, mtch
}