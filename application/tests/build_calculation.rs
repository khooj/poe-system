use std::time::Duration;

use application::{
    build_calculation::process_builds,
    storage::postgresql::{builds::BuildRepository, items::ItemRepository},
};
use tokio::{spawn, task::JoinHandle};
use tokio_util::sync::CancellationToken;

mod common;

#[cfg(feature = "integration_tests")]
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn check_process_build() -> anyhow::Result<()> {
    use application::build_calculation::process_single_build;
    use tokio::time::Instant;

    let mut ctx = common::setup_db().await?;

    let build = ctx.build_repo.get_build_for_processing().await?;
    let mut build = build.unwrap();

    let start = Instant::now();
    process_single_build(&mut ctx.item_repo, &mut build).await?;

    println!("calc time: {}ms", start.elapsed().as_millis());
    println!("build: {:?}", build.data.provided.boots);
    println!("build: {:?}", build.data.found.boots);
    println!("build: {:?}", build.processed);

    assert!(build.processed);

    Ok(())
}
