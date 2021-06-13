use crate::schema::build_info;

#[derive(Insertable)]
#[table_name = "build_info"]
pub struct NewBuild<'a> {
    pub id: String,
    pub pob_url: &'a str,
    pub itemset: &'a str,
}