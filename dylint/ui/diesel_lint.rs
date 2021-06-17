#[macro_use]
extern crate diesel;

mod schema;

use diesel::prelude::*;
use diesel::{Connection, Identifiable, SqliteConnection};

use schema::build_info;

#[derive(Queryable, Identifiable)]
#[table_name = "build_info"]
struct CustomStruct {
    id: String,
    // pob_url: String,
}

fn main() {
    use schema::build_info::dsl::*;
    let conn = match SqliteConnection::establish(":memory:") {
        Ok(k) => k,
        Err(e) => panic!("{}", e),
    };

    let _ = match build_info.select((id, pob_url)).load::<CustomStruct>(&conn) {
        Ok(k) => k,
        Err(e) => panic!("{}", e),
    };
}
