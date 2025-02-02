use pob::{ItemSet, Pob};

use crate::typed_item::{TypedItem, TypedItemError};

use super::{BuildInfo, BuildItemsWithConfig, ItemWithConfig};
use domain::{Category, Subcategory};

#[derive(Debug, thiserror::Error)]
pub enum ImportPobError {
    #[error("pob error")]
    Pob(#[from] pob::PobError),
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

fn import(itemset: ItemSet) -> Result<BuildInfo, ImportPobError> {
    let mut builditems = BuildItemsWithConfig::default();
    for it in itemset.items() {
        match it.category {
            Category::Armour => {
                if contains_subcategory(&it.subcategories, Subcategory::Boots) {
                    builditems.boots = ItemWithConfig {
                        item: TypedItem::try_from(it.clone())?,
                        ..Default::default()
                    };
                }
            }
            _ => {}
        }
    }

    Ok(BuildInfo {
        provided: builditems,
        ..Default::default()
    })
}

fn contains_subcategory(subs: &[Subcategory], subc: Subcategory) -> bool {
    subs.iter().any(|s| *s == subc)
}

#[cfg(test)]
mod tests {
    use crate::typed_item::ItemInfo;

    use super::import_build_from_pob_first_itemset;

    const POB: &str = include_str!("pob.xml");

    #[test]
    fn check_import_items() -> anyhow::Result<()> {
        let pob = pob::Pob::new(POB);
        let buildinfo = import_build_from_pob_first_itemset(&pob)?;
        assert_eq!(buildinfo.provided.boots.item.info, ItemInfo::Armor { basetype: "test".to_string(), quality: 0, mods: vec![], properties: vec![] });
        Ok(())
    }
}
