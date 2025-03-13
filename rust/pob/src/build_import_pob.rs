use crate::{ItemSet, Pob};

use domain::{
    build_calculation::{
        typed_item::{TypedItem, TypedItemError},
        BuildInfo, BuildItemsWithConfig, ItemWithConfig,
    },
    item::Item,
    types::{Category, Subcategory},
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
        match it.subcategories {
            Subcategory::Helmets => fill(&mut builditems.helmet, it)?,
            Subcategory::BodyArmour => fill(&mut builditems.body, it)?,
            Subcategory::Ring => {
                if builditems.ring1 == ItemWithConfig::default() {
                    fill(&mut builditems.ring1, it)?
                } else {
                    fill(&mut builditems.ring2, it)?
                }
            }
            Subcategory::Belt => fill(&mut builditems.belt, it)?,
            Subcategory::Gloves => fill(&mut builditems.gloves, it)?,
            Subcategory::Boots => fill(&mut builditems.boots, it)?,
            Subcategory::Shield => fill(&mut builditems.weapon2, it)?,
            Subcategory::Weapon => {
                if builditems.weapon1 == ItemWithConfig::default() {
                    fill(&mut builditems.weapon1, it)?
                } else {
                    fill(&mut builditems.weapon2, it)?
                }
            }
            Subcategory::Amulet => fill(&mut builditems.amulet, it)?,
            _ => {}
        }

        match it.category {
            Category::Flasks => {
                let mut ic = ItemWithConfig::default();
                fill(&mut ic, it)?;
                builditems.flasks.push(ic);
            }
            Category::Gems => {
                let mut ic = ItemWithConfig::default();
                fill(&mut ic, it)?;
                builditems.gems.push(ic);
            }
            Category::Jewels => {
                let mut ic = ItemWithConfig::default();
                fill(&mut ic, it)?;
                builditems.jewels.push(ic);
            }
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
