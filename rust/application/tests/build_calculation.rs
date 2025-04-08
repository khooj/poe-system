mod common;

#[cfg(feature = "integration_tests")]
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn check_process_build() -> anyhow::Result<()> {
    use application::build_calculation::process_single_build;
    use domain::build_calculation::FillRules;
    use tokio::time::Instant;

    let mut ctx = common::setup().await?;
    ctx.pob_data
        .provided
        .fill_configs_by_rule(FillRules::AllExist);

    let start = Instant::now();
    process_single_build(&mut ctx.item_repo, &mut ctx.pob_data).await?;

    println!("calc time: {}ms", start.elapsed().as_millis());
    println!("provided build: {:?}", ctx.pob_data.provided.boots);
    println!("found build: {:?}", ctx.pob_data.found.boots);

    Ok(())
}
