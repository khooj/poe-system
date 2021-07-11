use diesel::r2d2::{ConnectionManager, Pool};
use poe_system::implementations::item_repository::DieselItemRepository;
use poe_system::implementations::TypedConnectionPool;
use poe_system::ports::outbound::public_stash_retriever::PublicStashData;
use std::path::PathBuf;
use temp_file::{empty, TempFile};
use tracing_subscriber;

lazy_static::lazy_static! {
    static ref TRACING_EXEC: i32 = {
        tracing_subscriber::fmt::init();
        1
    };
}

// embed_migrations!("migrations");

pub struct TestApp {
    pub pool: TypedConnectionPool,
    pub file: TempFile,
}

pub fn prepare_test() -> Result<TestApp, anyhow::Error> {
    let _ = TRACING_EXEC;
    let f = empty();

    let pool = Pool::new(ConnectionManager::new(f.path().to_str().unwrap()))?;

    {
        let conn = pool.get()?;
        // embedded_migrations::run(&conn)?;
    }

    Ok(TestApp { pool, file: f })
}
