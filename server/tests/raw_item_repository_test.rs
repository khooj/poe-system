mod common;

use anyhow::Result;
use common::{insert_raw_items, ContainerDrop};
use poe_system::{
    infrastructure::poe_data::BASE_ITEMS,
    infrastructure::repositories::postgres::raw_item_repository::RawItemRepository,
};
use sqlx::PgPool;
use testcontainers::{clients::Cli, images::postgres::Postgres, Docker};

#[tokio::test]
async fn raw_item_repository_test() -> Result<()> {
    dotenv::dotenv().ok();
    let image = Postgres::default().with_version(14);

    let docker = Cli::default();
    let cont = docker.run(image);
    let port = cont.get_host_port(5432).unwrap();
    let _cont = ContainerDrop { container: cont };

    let pool = PgPool::connect(&format!(
        "postgres://postgres:postgres@localhost:{}/postgres",
        port
    ))
    .await?;
    sqlx::migrate!().run(&pool).await?;
    let repo = RawItemRepository::new(pool).await;

    insert_raw_items(&repo).await?;
    let alternate_types = BASE_ITEMS.get_alternate_types("Visored Sallet").unwrap();
    check_get_raw_items(&repo, &alternate_types, "Standard", 5).await?;
    check_get_raw_items(&repo, &alternate_types, "Hardcore", 0).await?;

    Ok(())
}

async fn check_get_raw_items(
    repo: &RawItemRepository,
    types: &[&str],
    league: &str,
    len: usize,
) -> Result<()> {
    use tokio_stream::StreamExt;
    let mut s = repo.get_items_cursor(types, league).await;

    let mut items = vec![];
    while let Some(db_item) = s.next().await {
        let db_item = db_item.unwrap();
        items.push(db_item);
    }

    assert_eq!(items.len(), len);

    Ok(())
}
