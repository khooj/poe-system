use domain::build_calculation::BuildInfo;
use pob::build_import_pob::import_build_from_pob_first_itemset;

const POB_FILE: &str = include_str!("pob.xml");

pub struct TestContext {
    pub pob_data: BuildInfo,
}

pub async fn setup() -> anyhow::Result<TestContext> {
    let pob = pob::Pob::new(POB_FILE);
    let data = import_build_from_pob_first_itemset(&pob)?;

    Ok(TestContext { pob_data: data })
}
