mod item_repository_dbg;

use domain::build_calculation::BuildInfo;
use item_repository_dbg::ItemRepositoryDbg;
use pob::build_import_pob::import_build_from_pob_first_itemset;

const POB_FILE: &str = include_str!("pob.xml");

pub struct TestContext {
    pub item_repo: ItemRepositoryDbg,
    pub pob_data: BuildInfo,
}

pub async fn setup() -> anyhow::Result<TestContext> {
    let item_repo = ItemRepositoryDbg::import_items("../slice1")?;

    let pob = pob::Pob::new(POB_FILE);
    let data = import_build_from_pob_first_itemset(&pob)?;

    Ok(TestContext {
        item_repo,
        pob_data: data,
    })
}
