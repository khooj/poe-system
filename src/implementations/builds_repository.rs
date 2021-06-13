use crate::implementations::models::NewBuild;
use diesel::prelude::*;
use thiserror::Error;
use uuid::Uuid;

#[derive(Error, Debug)]
pub enum BuildsRepositoryError {
    #[error("cant load from db")]
    RepositoryError(#[from] diesel::result::Error),
}

pub struct DieselBuildsRepository {
    conn: SqliteConnection,
}

impl DieselBuildsRepository {
    pub fn save_new_build(
        &self,
        pob: &str,
        item_set: &str,
    ) -> Result<String, BuildsRepositoryError> {
        use crate::schema::build_info::dsl::*;

        let builds = build_info
            .select(id)
            .filter(pob_url.eq(pob).and(itemset.eq(item_set)))
            .load::<String>(&self.conn)?;

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
            .execute(&self.conn)?;

        Ok(new_build.id.to_owned())
    }
}
