use std::time::Duration;

use application::{
    stash_receiver::StashReceiver,
    storage::postgresql::{
        builds::{BuildData, BuildRepository},
        items::ItemRepository,
    },
};
use domain::build_calculation::mod_config::ModConfig;
use pob::build_import_pob::import_build_from_pob_first_itemset;
use public_stash::models::PublicStashData;
use sqlx::{postgres::PgPoolOptions, types::chrono::Utc};
use uuid::Uuid;

const POB_FILE: &str = include_str!("pob.xml");

pub struct TestContext {
    pub item_repo: ItemRepository,
    pub build_repo: BuildRepository,
    pool: sqlx::Pool<sqlx::Postgres>,
    guard: DbGuard,
}

pub async fn setup_db() -> anyhow::Result<TestContext> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://khooj@localhost/khooj")
        .await?;

    let guard = DbGuard { pool: pool.clone() };

    sqlx::query("create database testing")
        .execute(&pool)
        .await?;

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://khooj@localhost/testing")
        .await?;

    let item_repo = ItemRepository::new(pool.clone()).await?;
    import_items(item_repo.clone(), "../slice1").await?;
    let mut build_repo = BuildRepository::new(pool.clone()).await?;

    let pob = pob::Pob::new(POB_FILE);
    let mut data = import_build_from_pob_first_itemset(&pob)?;
    let first_stat_id = data
        .provided
        .boots
        .item
        .info
        .mod_ids()
        .first()
        .map(|s| s.to_string())
        .unwrap();
    assert!(data
        .provided
        .boots
        .item
        .info
        .add_or_update_config(&first_stat_id, ModConfig::Exist));

    build_repo
        .upsert_build(&BuildData {
            id: Uuid::new_v4(),
            processed: false,
            inserted_at: Utc::now(),
            updated_at: Utc::now(),
            data,
        })
        .await?;

    Ok(TestContext {
        item_repo,
        build_repo,
        pool,
        guard,
    })
}

struct DbGuard {
    pool: sqlx::Pool<sqlx::Postgres>,
}

impl Drop for DbGuard {
    fn drop(&mut self) {
        let pool = self.pool.clone();
        let handle = tokio::spawn(async move {
            let ret = sqlx::query("drop database testing").execute(&pool).await;
            println!("drop database result: {:?}", ret);
            pool.close().await;
        });
        let ret = futures::executor::block_on(handle);
        println!("ret: {:?}", ret);
    }
}

async fn import_items<T: AsRef<str>>(item_repo: ItemRepository, dir: T) -> anyhow::Result<()> {
    let stashes = utils::stream_stashes::open_stashes(dir.as_ref());
    let mut receiver = StashReceiver::new(item_repo, vec![]);

    for (_, content) in stashes {
        let data: PublicStashData = serde_json::from_str(&content)?;
        receiver.receive(data).await?;
    }

    Ok(())
}
