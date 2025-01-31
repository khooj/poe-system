use application::{
    build_calculation::{comparison::Comparator, ItemWithConfig},
    storage::{
        postgresql::{builds::BuildRepository, items::ItemRepository},
        ItemRepositoryTrait,
    },
    typed_item::TypedItem,
};
use metrics::histogram;
use serde::Deserialize;
use sqlx::postgres::PgPoolOptions;
use std::time::Duration;
use tokio::time::Instant;
use tokio_util::sync::CancellationToken;

async fn process_builds(
    cancel: CancellationToken,
    mut items_repo: ItemRepository,
    mut build_repo: BuildRepository,
) -> anyhow::Result<()> {
    loop {
        tokio::select! {
            _ = cancel.cancelled() => break,
            else => {}
        };

        tokio::time::sleep(Duration::from_millis(500)).await;

        let mut build = match build_repo.get_build_for_processing().await? {
            Some(b) => b,
            None => continue,
        };

        let start = Instant::now();

        build.data.found.helmet =
            find_similar(&mut items_repo, &build.data.provided.helmet).await?;
        build.processed = true;
        build_repo.upsert_build(build).await?;

        let delta = start.elapsed();
        histogram!("build_calculator.process_build.time").record(delta);
    }
    Ok(())
}

async fn find_similar(
    items_repo: &mut ItemRepository,
    item: &ItemWithConfig,
) -> anyhow::Result<Option<TypedItem>> {
    let mods_for_search = Comparator::extract_mods_for_search(&item.config, &item.item);
    let found_items = items_repo.search_items_by_mods(mods_for_search).await?;
    Ok(found_items.first().cloned())
}

#[derive(Deserialize, Debug)]
struct Settings {
    pg: String,
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
    let item_repo = ItemRepository::new(pool.clone()).await?;
    let build_repo = BuildRepository::new(pool).await?;
    let token_clone = token.clone();
    let handle =
        tokio::spawn(async move { process_builds(token_clone, item_repo, build_repo).await });

    ctrlc.await?;
    token.cancel();
    handle.await??;

    Ok(())
}
