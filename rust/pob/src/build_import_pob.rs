use crate::{ItemSet, Pob};

use domain::{
    build_calculation::{
        typed_item::{TypedItem, TypedItemError},
        BuildInfo, BuildItemsWithConfig, ItemWithConfig,
    },
    item::Item,
    types::Subcategory,
};

#[derive(Debug, thiserror::Error)]
pub enum ImportPobError {
    #[error("pob error")]
    Pob(#[from] crate::PobError),
    #[error("domain item convert to typed error")]
    Convert(#[from] TypedItemError),
}

pub fn import_build_from_pob<T: AsRef<str>>(
    pob: &Pob,
    itemset: T,
) -> Result<BuildInfo, ImportPobError> {
    let doc = pob.as_document()?;
    let itemset = doc.get_itemset(itemset.as_ref())?;
    import(itemset)
}

pub fn import_build_from_pob_first_itemset(pob: &Pob) -> Result<BuildInfo, ImportPobError> {
    let doc = pob.as_document()?;
    let itemset = doc.get_first_itemset()?;
    import(itemset)
}

fn fill(prov_item: &mut ItemWithConfig, it: &Item) -> Result<(), ImportPobError> {
    *prov_item = ItemWithConfig {
        item: TypedItem::try_from(it.clone())?,
        ..Default::default()
    };
    Ok(())
}

fn import(itemset: ItemSet) -> Result<BuildInfo, ImportPobError> {
    let mut builditems = BuildItemsWithConfig::default();
    for it in itemset.items() {
        #[allow(clippy::single_match)]
        match it.subcategories {
            Subcategory::Boots => fill(&mut builditems.boots, it)?,
            _ => {}
        }
    }

    Ok(BuildInfo {
        provided: builditems,
        ..Default::default()
    })
}

#[cfg(test)]
mod tests {
    use super::import_build_from_pob_first_itemset;
    use crate::Pob;
    use domain::build_calculation::typed_item::ItemInfo;

    const POB: &str = include_str!("pob.xml");

    #[test]
    fn check_import_items() -> anyhow::Result<()> {
        let pob = Pob::new(POB);
        let buildinfo = import_build_from_pob_first_itemset(&pob)?;
        assert_ne!(buildinfo.provided.boots.item.info, ItemInfo::default(),);
        Ok(())
    }
}
