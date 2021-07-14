use super::models::NewBuildMatch;
use super::TypedConnectionPool;
use crate::domain::PastebinBuild;
use crate::implementations::models::{NewBuild, PobBuild};
use diesel::prelude::*;
use thiserror::Error;
use uuid::Uuid;

#[derive(Error, Debug)]
pub enum BuildsRepositoryError {
    #[error("cant load from db")]
    Repository(#[from] diesel::result::Error),
    #[error("pool empty")]
    PoolError(#[from] r2d2::Error),
    #[error("empty response")]
    Empty,
}

#[derive(Clone)]
pub struct DieselBuildsRepository {
    pub conn: TypedConnectionPool,
}

impl DieselBuildsRepository {
    pub fn save_new_build(
        &self,
        pob: &PastebinBuild,
        item_set: &str,
    ) -> Result<String, BuildsRepositoryError> {
        use crate::schema::build_info::dsl::*;

        let conn = self.conn.get()?;

        let builds = build_info
            .select(id)
            .filter(pob_url.eq(pob.as_ref()).and(itemset.eq(item_set)))
            .load::<String>(&conn)?;

        if builds.len() > 0 {
            return Ok(builds[0].clone());
        }

        let new_build = NewBuild {
            // TODO: change to_string()
            id: Uuid::new_v4().to_hyphenated().to_string(),
            pob_url: pob,
            itemset: item_set,
        };

        diesel::insert_into(build_info)
            .values(&new_build)
            .execute(&conn)?;

        Ok(new_build.id.to_owned())
    }

    pub fn get_build(&self, id_: &str) -> Result<PobBuild, BuildsRepositoryError> {
        use crate::schema::build_info::dsl::*;

        let conn = self.conn.get()?;

        let builds = build_info
            .filter(id.eq(id_))
            .limit(1)
            .load::<PobBuild>(&conn)?;

        if builds.len() > 0 {
            Ok(builds[0].clone())
        } else {
            Err(BuildsRepositoryError::Empty)
        }
    }

    pub fn get_build_by_url(
        &self,
        url: &PastebinBuild,
    ) -> Result<Vec<PobBuild>, BuildsRepositoryError> {
        use crate::schema::build_info::dsl::*;

        let conn = self.conn.get()?;

        Ok(build_info
            .filter(pob_url.eq(url.as_ref()))
            .load::<PobBuild>(&conn)?)
    }

    pub fn update_build(&self, build: &PobBuild) -> Result<(), BuildsRepositoryError> {
        use crate::schema::build_info::dsl::*;

        let conn = self.conn.get()?;

        diesel::update(build)
            .set((
                pob_url.eq(build.pob_url.as_ref()),
                itemset.eq(&build.itemset),
            ))
            .execute(&conn)?;

        Ok(())
    }

    pub fn new_build_match(&self, mtch: &NewBuildMatch) -> Result<(), BuildsRepositoryError> {
        use crate::schema::builds_match::dsl::*;

        let conn = self.conn.get()?;

        diesel::insert_into(builds_match)
            .values(mtch)
            .execute(&conn)?;

        Ok(())
    }

    pub fn get_items_id_for_build(&self, id_: &str) -> Result<Vec<String>, BuildsRepositoryError> {
        use crate::schema::builds_match::dsl::*;

        let conn = self.conn.get()?;

        Ok(builds_match
            .filter(id.eq(id_))
            .select(item_id)
            .get_results::<String>(&conn)?)
    }
}
