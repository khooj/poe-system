use application::{
    build_calculation::process_builds,
    storage::postgresql::{builds::BuildRepository, items::ItemRepository},
};
use serde::Deserialize;
use sqlx::postgres::PgPoolOptions;
use tokio_util::sync::CancellationToken;
use tracing::{debug, info};

#[derive(Deserialize, Debug)]
struct Settings {
    pg: String,
}

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt().init();

    let settings: Settings = config::Config::builder()
        .add_source(
            config::File::with_name("config")
                .format(config::FileFormat::Toml)
                .required(false),
        )
        .add_source(config::Environment::with_prefix("APP"))
        .build()?
        .try_deserialize()?;

    let token = CancellationToken::new();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&settings.pg)
        .await?;
    info!(message = "connected to postgresql");
    let item_repo = ItemRepository::new(pool.clone()).await?;
    let build_repo = BuildRepository::new(pool).await?;
    let token_clone = token.clone();
    let handle =
        tokio::spawn(async move { process_builds(token_clone, item_repo, build_repo).await });

    let ctrlc = tokio::signal::ctrl_c();
    info!(message = "started");
    ctrlc.await?;
    token.cancel();
    handle.await??;

    Ok(())
}
