mod common;

use common::prepare_test;

use poe_system::actors::build_calculator::{BuildCalculatorActor, StartBuildCalculatingMsg};

#[test]
fn check_build_calculating() -> Result<(), anyhow::Error> {
    let app = prepare_test()?;
    Ok(())
}
