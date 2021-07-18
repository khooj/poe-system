use super::models::{BuildMatch, PobBuild, PobFile};
use super::TypedConnectionPool;
use crate::domain::PastebinBuild;
use diesel::prelude::*;
use thiserror::Error;
use tracing::{event, instrument, Level};
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
    #[instrument(err, skip(self, pob), fields(id = ?pob.id, token = pob.url_token()))]
    pub fn save_new_pob_file(
        &self,
        pob: PobFile,
    ) -> Result<String, BuildsRepositoryError> {
        use crate::schema::pob_file::dsl::*;

        let conn = self.conn.get()?;

        match pob_file
            .select(id)
            .filter(url_token.eq(pob.url_token()))
            .first::<String>(&conn)
        {
            ok @ Ok(_) => Ok(ok?),
            Err(e) => {
                event!(Level::DEBUG, "got error while selecting pob: {}", e);
                diesel::insert_into(pob_file).values(&pob).execute(&conn)?;
                Ok(pob.id)
            }
        }
    }

    #[instrument(err, skip(self))]
    pub fn save_new_build(
        &self,
        mut build: PobBuild,
    ) -> Result<String, BuildsRepositoryError> {
        use crate::schema::build_info::dsl::*;

        let conn = self.conn.get()?;

        let builds = build_info
            .select(id)
            .filter(
                pob_file_id
                    .eq(&build.pob_file_id)
                    .and(itemset.eq(&build.itemset)),
            )
            .load::<String>(&conn)?;

        if builds.len() > 0 {
            return Ok(builds[0].clone());
        }

        build.id = Uuid::new_v4().to_hyphenated().to_string();

        diesel::insert_into(build_info)
            .values(&build)
            .execute(&conn)?;

        Ok(build.id)
    }

    #[instrument(err, skip(self))]
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

    #[instrument(err, skip(self))]
    pub fn get_build_by_url(
        &self,
        url: &PastebinBuild,
    ) -> Result<Vec<PobBuild>, BuildsRepositoryError> {
        use crate::schema::build_info::dsl::*;

        let conn = self.conn.get()?;

        Ok(build_info
            .filter(pob_file_id.eq(url.as_ref()))
            .load::<PobBuild>(&conn)?)
    }

    #[instrument(err, skip(self))]
    pub fn update_build(&self, build: &PobBuild) -> Result<(), BuildsRepositoryError> {
        use crate::schema::build_info::dsl::*;

        let conn = self.conn.get()?;

        diesel::update(build)
            .set((
                pob_file_id.eq(&build.pob_file_id),
                itemset.eq(&build.itemset),
            ))
            .execute(&conn)?;

        Ok(())
    }

    #[instrument(err, skip(self))]
    pub fn new_build_match(&self, mtch: &BuildMatch) -> Result<(), BuildsRepositoryError> {
        use crate::schema::builds_match::dsl::*;

        let conn = self.conn.get()?;

        diesel::insert_into(builds_match)
            .values(mtch)
            .execute(&conn)?;

        Ok(())
    }

    #[instrument(err, skip(self))]
    pub fn get_items_id_for_build(&self, id_: &str) -> Result<Vec<(i32, String)>, BuildsRepositoryError> {
        use crate::schema::builds_match::dsl::*;

        let conn = self.conn.get()?;

        Ok(builds_match
            .filter(id.eq(id_))
            .select((idx, item_id))
            .get_results::<(i32, String)>(&conn)?)
    }

    #[instrument(err, skip(self))]
    pub fn get_pob_file(&self, id_: &str) -> Result<PobFile, BuildsRepositoryError> {
        use crate::schema::pob_file::dsl::*;

        let conn = self.conn.get()?;
        Ok(pob_file.filter(id.eq(id_)).first::<PobFile>(&conn)?)
    }

    #[instrument(err, skip(self))]
    pub fn get_pob_file_id_by_url(
        &self,
        token: &PastebinBuild,
    ) -> Result<String, BuildsRepositoryError> {
        use crate::schema::pob_file::dsl::*;

        let conn = self.conn.get()?;
        Ok(pob_file
            .select(id)
            .filter(url_token.eq(token.as_ref()))
            .first::<String>(&conn)?)
    }
}
