pub mod comparison;
pub mod import_pob;
pub mod mod_config;

use std::time::Duration;

use comparison::Comparator;
use metrics::histogram;
use mod_config::ModConfig;
use serde::{Deserialize, Serialize};
use tokio::time::Instant;
use tokio_util::sync::CancellationToken;

use crate::{
    storage::{
        postgresql::{
            builds::{BuildData, BuildRepository},
            items::ItemRepository,
        },
        ItemRepositoryTrait,
    },
    typed_item::TypedItem,
};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct BuildInfo {
    pub provided: BuildItemsWithConfig,
    pub found: FoundBuildItems,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct BuildItemsWithConfig {
    pub helmet: ItemWithConfig,
    pub body: ItemWithConfig,
    pub boots: ItemWithConfig,
    pub gloves: ItemWithConfig,
    pub weapon1: ItemWithConfig,
    pub weapon2: ItemWithConfig,
    pub ring1: ItemWithConfig,
    pub ring2: ItemWithConfig,
    pub belt: ItemWithConfig,
    pub flasks: Vec<ItemWithConfig>,
    pub gems: Vec<ItemWithConfig>,
    pub jewels: Vec<ItemWithConfig>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ItemWithConfig {
    pub config: Vec<ModConfig>,
    pub item: TypedItem,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct FoundBuildItems {
    pub helmet: Option<TypedItem>,
    pub body: Option<TypedItem>,
    pub boots: Option<TypedItem>,
    pub gloves: Option<TypedItem>,
    pub weapon1: Option<TypedItem>,
    pub weapon2: Option<TypedItem>,
    pub ring1: Option<TypedItem>,
    pub ring2: Option<TypedItem>,
    pub belt: Option<TypedItem>,
    pub flasks: Option<Vec<TypedItem>>,
    pub gems: Option<Vec<TypedItem>>,
    pub jewels: Option<Vec<TypedItem>>,
}

pub async fn process_builds(
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

        process_single_build(&mut items_repo, &mut build).await?;

        build_repo.upsert_build(build).await?;

        let delta = start.elapsed();
        histogram!("build_calculator.process_build.time").record(delta);
    }
    Ok(())
}

pub async fn process_single_build(
    items_repo: &mut ItemRepository,
    build: &mut BuildData,
) -> anyhow::Result<()> {
    build.data.found.boots = find_similar(items_repo, &build.data.provided.boots).await?;
    build.processed = true;
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
