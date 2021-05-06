use diesel::prelude::*;
use crate::domain::item::*;
use crate::schema::*;

type DB = diesel::sqlite::Sqlite;

impl Queryable<items::SqlType, DB> for Item {
    type Row = ()

    fn build(row: Self::Row) -> Self {
        
    }
}