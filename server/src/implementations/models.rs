use crate::schema::{build_info, builds_match};

#[derive(Insertable)]
#[table_name = "build_info"]
pub struct NewBuild<'a> {
    pub id: String,
    pub pob_url: &'a str,
    pub itemset: &'a str,
}

#[derive(Queryable, Identifiable, AsChangeset, Clone)]
#[table_name = "build_info"]
pub struct PobBuild {
    pub id: String,
    pub pob_url: String,
    pub itemset: String,
}

#[derive(Insertable, AsChangeset)]
#[table_name = "builds_match"]
pub struct NewBuildMatch<'a> {
    pub id: String,
    pub idx: i32,
    pub score: i32,
    pub item_id: &'a str,
}
