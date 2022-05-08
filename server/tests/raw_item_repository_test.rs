mod common;

use anyhow::Result;
use common::ContainerDrop;
use poe_system::interfaces::public_stash_retriever::Item;
use poe_system::{
    infrastructure::repositories::postgres::{
        raw_item::RawItem, raw_item_repository::RawItemRepository,
    },
    interfaces::public_stash_retriever::PublicStashChange,
};
use sqlx::PgPool;
use testcontainers::{
    clients::Cli,
    images::generic::{GenericImage, Stream, WaitFor},
    Container, Docker, Image, RunArgs,
};

#[tokio::test]
async fn raw_item_repository_test() -> Result<()> {
    // env::set_var("RUST_LOG", "webdav_ss=debug,webdav_handler=debug");

    let args = RunArgs::default().with_mapped_port((2345, 5432));
    let image = GenericImage::new("postgres:14-alpine")
        .with_wait_for(WaitFor::LogMessage {
            message: "database system is ready to accept connections".into(),
            stream: Stream::StdOut,
        })
        .with_env_var("POSTGRES_USER", "admin")
        .with_env_var("POSTGRES_PASSWORD", "admin");

    let docker = Cli::default();
    let cont = docker.run_with_args(image, args);
    let _cont = ContainerDrop { container: cont };

    let pool = PgPool::connect("postgres://admin:admin@localhost:2345/admin").await?;
    let repo = RawItemRepository::new(pool).await;

    // insert_raw_items(&repo).await?;

    Ok(())
}

const EXAMPLE_STASH_CHANGE: &'static str = include_str!("example-stash.json");

async fn insert_raw_items(repo: &RawItemRepository) -> Result<()> {
    let mut tr = repo.begin().await?;

    let stash: PublicStashChange = serde_json::from_str(EXAMPLE_STASH_CHANGE)?;
    for i in stash.items {
        repo.insert_raw_item(
            &mut tr,
            i.id.as_ref().unwrap(),
            stash.account_name.as_ref().unwrap(),
            stash.stash.as_ref().unwrap(),
            i.clone(),
        )
        .await?;
    }

    tr.commit().await?;
    Ok(())
}
