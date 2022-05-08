mod common;
mod server;

use anyhow::Result;
use common::ContainerDrop;
use poe_system::application::build_calculating::BuildCalculating;
use poe_system::infrastructure::repositories::postgres::build_repository::BuildRepository;
use poe_system::infrastructure::repositories::postgres::task_repository::TaskRepository;
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
    images::postgres::{Postgres, PostgresArgs},
    Container, Docker, Image, RunArgs,
};

#[tokio::test]
async fn build_calculating_test() -> Result<()> {
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
    let items_repo = RawItemRepository::new(pool.clone()).await;
    let build_repo = BuildRepository::new(pool.clone());
    let tasks_repo = TaskRepository::new(pool);
    let server =
        server::http(|req| async move { http::Response::builder().body("asd".into()).unwrap() });

    let build_calculating = BuildCalculating::new_with_host(
        items_repo,
        tasks_repo,
        build_repo,
        server.addr().to_string().as_str(),
    );

    Ok(())
}
