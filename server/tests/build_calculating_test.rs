mod common;
mod server;

use anyhow::Result;
use common::{insert_raw_items, ContainerDrop};
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
use server::Server;
use sqlx::PgPool;
use testcontainers::{
    clients::Cli,
    images::generic::{GenericImage, Stream, WaitFor},
    images::postgres::{Postgres, PostgresArgs},
    Container, Docker, Image, RunArgs,
};
use tracing::debug;

struct Repos {
    items: RawItemRepository,
    tasks: TaskRepository,
    builds: BuildRepository,
}

impl Repos {
    fn build_calculating(&self, server: &Server) -> Result<BuildCalculating> {
        BuildCalculating::new_with_host(
            self.items.clone(),
            self.tasks.clone(),
            self.builds.clone(),
            &format!("http://{}", server.addr().to_string().as_str()),
        )
    }
}

#[tokio::test]
async fn build_calculating_test() -> Result<()> {
    dotenv::dotenv().ok();
    // tracing_subscriber::fmt()
    //     .with_max_level(tracing::Level::DEBUG)
    //     .init();

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

    insert_raw_items(&items_repo).await?;

    let repos = Repos {
        items: items_repo,
        tasks: tasks_repo,
        builds: build_repo,
    };

    check_build_calculating(&repos).await?;

    Ok(())
}

const POB1: &str = include_str!("pob.txt");

async fn check_build_calculating(repos: &Repos) -> Result<()> {
    let server = server::http(|req| async move {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.uri(), "/asd");

        http::Response::builder().body(POB1.into()).unwrap()
    });

    let build_calc = repos.build_calculating(&server)?;
    println!("adding build for calc");

    let id = build_calc
        .add_build_for_calculating("http://customurl.com/asd", "", "Standard")
        .await?;
    build_calc.calculate_next_build().await?;
    let build = build_calc.get_calculated_build(&id).await?;
    println!("id: {}", id);
    let helmet = build.found_items.0.helmet;
    debug!("required {:?}\nfound {:?}", build.required_items.0.helmet, helmet);
    assert_eq!(&helmet.base_type, "Gladiator Helmet");
    assert_eq!(&helmet.name, "Blood Corona");

    Ok(())
}
