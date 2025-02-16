use std::time::Duration;

use application::storage::postgresql::builds::BuildRepository;
use serde::Deserialize;
use sqlx::postgres::PgPoolOptions;
use tokio_util::sync::CancellationToken;

#[derive(Deserialize, Debug)]
struct Settings {
    pg: String,
    timeout: Duration,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let settings: Settings = config::Config::builder()
        .add_source(
            config::File::with_name("config")
                .format(config::FileFormat::Toml)
                .required(false),
        )
        .add_source(config::Environment::with_prefix("APP"))
        .build()?
        .try_deserialize()?;

    let ctrlc = tokio::signal::ctrl_c();
    let token = CancellationToken::new();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&settings.pg)
        .await?;
    let mut build_repo = BuildRepository::new(pool).await?;
    let token_clone = token.clone();
    let handle = tokio::spawn(async move {
        loop {
            tokio::select! {
                _ = token_clone.cancelled() => break,
                else => {}
            };

            tokio::time::sleep(Duration::from_millis(100)).await;

            if let Err(e) = build_repo.unlock_failed_builds(settings.timeout).await {
                eprintln!("{}", e);
            }
        }
    });

    ctrlc.await?;
    token.cancel();
    handle.await?;

    Ok(())
}
