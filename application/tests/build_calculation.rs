use std::time::Duration;

use application::{build_calculation::process_builds, storage::postgresql::{builds::BuildRepository, items::ItemRepository}};
use tokio::{spawn, task::JoinHandle};
use tokio_util::sync::CancellationToken;

mod common;

#[cfg(integration_tests)]
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn check_process_build() -> anyhow::Result<()> {
    let mut ctx = common::setup_db().await?;

    let token = CancellationToken::new();
    let handle = spawn_process_builds(token.clone(), ctx.item_repo.clone(), ctx.build_repo.clone()).await;
    tokio::time::sleep(Duration::from_secs(5)).await;
    token.cancel();
    handle.await??;

    let builds = ctx.build_repo.get_builds().await?;
    assert!(!builds.is_empty());
    println!("build: {:?}", builds[0]);

    Ok(())
}

async fn spawn_process_builds(token: CancellationToken, item: ItemRepository, build: BuildRepository) -> JoinHandle<anyhow::Result<()>> {
    tokio::spawn(async move { process_builds(token, item, build).await })
}

