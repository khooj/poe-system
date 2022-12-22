mod common;
mod server;

use anyhow::Result;
use common::ContainerDrop;
use poe_system::application::build_calculating::BuildCalculating;
use poe_system::infrastructure::repositories::postgres::build_repository::BuildRepository;
use poe_system::infrastructure::repositories::postgres::task_repository::TaskRepository;
use server::Server;
use sqlx::PgPool;
use testcontainers::{clients::Cli, images::postgres::Postgres, Docker};
use tracing::debug;

struct Repos {
    tasks: TaskRepository,
    builds: BuildRepository,
}

impl Repos {
    fn build_calculating(&self, _server: &Server) -> BuildCalculating {
        BuildCalculating::new(self.tasks.clone(), self.builds.clone())
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
    let build_repo = BuildRepository::new(pool.clone());
    let tasks_repo = TaskRepository::new(pool);

    let repos = Repos {
        tasks: tasks_repo,
        builds: build_repo,
    };

    check_build_calculating(&repos).await?;

    Ok(())
}

const POB1: &str = include_str!("pob.txt");
const POB3: &str = include_str!("pob3.txt");

async fn check_build_calculating(repos: &Repos) -> Result<()> {
    let server = server::http(|req| async move {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.uri(), "/asd");

        // not used
        http::Response::builder().body(POB1.into()).unwrap()
    });

    let build_calc = repos.build_calculating(&server);
    println!("adding build for calc");

    let id = build_calc
        .add_build_for_calculating(POB3, "", "Standard")
        .await?;
    build_calc.calculate_next_build().await?;
    let build = build_calc.get_calculated_build(&id).await?;
    let build = build.unwrap();
    println!("id: {}", id);
    let helmet = build.found_items.0.helmet;
    debug!(
        "required {:?}\nfound {:?}",
        build.required_items.0.helmet, helmet
    );

    Ok(())
}
