use domain::build_calculation::{
    comparison::Comparator, typed_item::TypedItem, BuildInfo, ItemWithConfig,
};
use metrics::histogram;
use std::time::Duration;
use tokio::time::{self, Instant};
use tokio_util::sync::CancellationToken;
use tracing::{debug, info, instrument, span, trace, Instrument};

use crate::storage::{
    postgresql::{
        builds::{BuildData, BuildRepository},
        items::ItemRepository,
    },
    ItemRepositoryTrait, SearchItemsByModsTrait,
};

#[instrument(skip_all, ret, err)]
pub async fn process_builds<T: ItemRepositoryTrait>(
    cancel: CancellationToken,
    mut items_repo: T,
    mut build_repo: BuildRepository,
) -> anyhow::Result<()> {
    let mut interval = time::interval(Duration::from_millis(500));
    let cancelled = cancel.cancelled();
    tokio::pin!(cancelled);
    loop {
        tokio::select! {
            _ = &mut cancelled => { break }
            _ = interval.tick() => {},
        };

        let mut build = match build_repo.get_build_for_processing().await? {
            Some(b) => {
                debug!(
                    message = "got build to process",
                    build_id = b.id.to_string()
                );
                b
            }
            None => {
                trace!(message = "no builds to process, sleeping");
                continue;
            }
        };

        let start = Instant::now();

        process_single_build(&mut items_repo, &mut build.data).await?;
        build.processed = true;

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

#[instrument(skip_all, ret, err)]
pub async fn process_single_build<T: SearchItemsByModsTrait>(
    items_repo: &mut T,
    build: &mut BuildInfo,
) -> anyhow::Result<()> {
    build.found.boots = find_similar(items_repo, &build.provided.boots).await?;
    build.found.helmet = find_similar(items_repo, &build.provided.helmet).await?;
    build.found.body = find_similar(items_repo, &build.provided.body).await?;
    build.found.gloves = find_similar(items_repo, &build.provided.gloves).await?;
    build.found.weapon1 = find_similar(items_repo, &build.provided.weapon1).await?;
    build.found.weapon2 = find_similar(items_repo, &build.provided.weapon2).await?;
    build.found.ring1 = find_similar(items_repo, &build.provided.ring1).await?;
    build.found.ring2 = find_similar(items_repo, &build.provided.ring2).await?;
    build.found.belt = find_similar(items_repo, &build.provided.belt).await?;
    build.found.amulet = find_similar(items_repo, &build.provided.amulet).await?;

    let mut flasks = vec![];
    for it in &build.provided.flasks {
        if let Some(found) = find_similar(items_repo, it).await? {
            flasks.push(found);
        }
    }
    if !flasks.is_empty() {
        build.found.flasks = Some(flasks);
    }

    let mut gems = vec![];
    for it in &build.provided.gems {
        if let Some(found) = find_similar(items_repo, it).await? {
            gems.push(found);
        }
    }
    if !gems.is_empty() {
        build.found.gems = Some(gems);
    }

    let mut jewels = vec![];
    for it in &build.provided.jewels {
        if let Some(found) = find_similar(items_repo, it).await? {
            jewels.push(found);
        }
    }
    if !jewels.is_empty() {
        build.found.jewels = Some(jewels);
    }

    Ok(())
}

async fn find_similar<T: SearchItemsByModsTrait>(
    item_searcher: &mut T,
    item: &ItemWithConfig,
) -> anyhow::Result<Option<TypedItem>> {
    let mods_for_search = Comparator::extract_mods_for_search(&item.item);
    let found_items = item_searcher
        .search_items_by_attrs(
            None,
            Some(item.item.category.clone()),
            Some(item.item.subcategory.clone()),
            Some(mods_for_search),
        )
        .await?;
    Ok(found_items.first().cloned())
}
