mod common;

use common::prepare_test;

#[test]
fn check_build_calculating() -> Result<(), anyhow::Error> {
    let _app = prepare_test()?;
    Ok(())
}
