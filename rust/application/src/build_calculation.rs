use domain::build_calculation::{comparison::Comparator, typed_item::TypedItem, ItemWithConfig};
use metrics::histogram;
use std::time::Duration;
use tokio::time::Instant;
use tokio_util::sync::CancellationToken;

use crate::storage::postgresql::{
    builds::{BuildData, BuildRepository},
    items::ItemRepository,
};

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

        let mut build = match build_repo.get_build_for_processing().await? {
            Some(b) => b,
            None => {
                tokio::time::sleep(Duration::from_millis(500)).await;
                continue;
            }
        };

        let start = Instant::now();

        process_single_build(&mut items_repo, &mut build).await?;

        // TODO: probably better to do this 2 operations in single transaction
        // because of possible race condition
        // (upsert build -> unlock failed build -> getting build for calculation -> unlock build)
        // i think the worst thing that can happen is "blink" items on client side
        build_repo.upsert_build(&build).await?;
        build_repo.unlock_build(&build.id).await?;

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
    build.data.found.helmet = find_similar(items_repo, &build.data.provided.helmet).await?;
    build.data.found.body = find_similar(items_repo, &build.data.provided.body).await?;
    build.data.found.gloves = find_similar(items_repo, &build.data.provided.gloves).await?;
    build.data.found.weapon1 = find_similar(items_repo, &build.data.provided.weapon1).await?;
    build.data.found.weapon2 = find_similar(items_repo, &build.data.provided.weapon2).await?;
    build.data.found.ring1 = find_similar(items_repo, &build.data.provided.ring1).await?;
    build.data.found.ring2 = find_similar(items_repo, &build.data.provided.ring2).await?;
    build.data.found.belt = find_similar(items_repo, &build.data.provided.belt).await?;

    let mut flasks = vec![];
    for it in &build.data.provided.flasks {
        if let Some(found) = find_similar(items_repo, it).await? {
            flasks.push(found);
        }
    }
    if !flasks.is_empty() {
        build.data.found.flasks = Some(flasks);
    }

    let mut gems = vec![];
    for it in &build.data.provided.gems {
        if let Some(found) = find_similar(items_repo, it).await? {
            gems.push(found);
        }
    }
    if !gems.is_empty() {
        build.data.found.gems = Some(gems);
    }

    let mut jewels = vec![];
    for it in &build.data.provided.jewels {
        if let Some(found) = find_similar(items_repo, it).await? {
            jewels.push(found);
        }
    }
    if !jewels.is_empty() {
        build.data.found.jewels = Some(jewels);
    }

    build.processed = true;
    Ok(())
}

async fn find_similar(
    items_repo: &mut ItemRepository,
    item: &ItemWithConfig,
) -> anyhow::Result<Option<TypedItem>> {
    let mods_for_search = Comparator::extract_mods_for_search(&item.item);
    let found_items = items_repo
        .search_items_by_attrs(
            None,
            Some(item.item.category.clone()),
            Some(item.item.subcategory.clone()),
            Some(mods_for_search),
        )
        .await?;
    Ok(found_items.first().cloned())
}
