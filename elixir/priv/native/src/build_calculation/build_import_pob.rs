use domain::item::{
    types::{Category, Subcategory},
    Item as DomainItem,
};
use pob::{ItemSet, Pob, PobError, SkillSet};

use super::{
    builds::BuildItems,
    item::{Item, ItemError},
};

#[derive(Debug, thiserror::Error)]
pub enum ImportPobError {
    #[error("pob error: {0}")]
    Pob(#[from] PobError),
    #[error("domain item convert to app item error: {0}")]
    Convert(#[from] ItemError),
}

pub fn import_build_from_pob<T: AsRef<str>>(
    pob: &Pob,
    itemset: T,
    skillset: T,
) -> Result<BuildItems, ImportPobError> {
    let doc = pob.as_document()?;
    let itemset = doc.get_itemset(itemset.as_ref())?;
    let skillset = doc
        .get_skillsets()
        .iter()
        .find(|s| s.title() == skillset.as_ref())
        .unwrap()
        .clone();
    import(itemset, skillset)
}

pub fn import_build_from_pob_first_itemset(pob: &Pob) -> Result<BuildItems, ImportPobError> {
    let doc = pob.as_document()?;
    let itemset = doc.get_first_itemset()?;
    let skillset = doc.get_skillsets().first().unwrap().clone();
    import(itemset, skillset)
}

fn fill(prov_item: &mut Option<Item>, it: &DomainItem) -> Result<(), ImportPobError> {
    *prov_item = Some(Item::try_from(it.clone())?);
    Ok(())
}

fn import(itemset: ItemSet, skillset: SkillSet) -> Result<BuildItems, ImportPobError> {
    let mut builditems = BuildItems::default();
    for it in itemset.items() {
        match it.subcategories {
            Subcategory::Helmets => fill(&mut builditems.helmet, it)?,
            Subcategory::BodyArmour => fill(&mut builditems.body, it)?,
            Subcategory::Ring => {
                if builditems.ring1.is_none() {
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
                if builditems.weapon1.is_none() {
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
                let mut ic = None;
                fill(&mut ic, it)?;
                if builditems.flasks.is_none() {
                    builditems.flasks = Some(vec![]);
                }
                builditems.flasks.as_mut().unwrap().push(ic.unwrap());
            }
            Category::Jewels => {
                let mut ic = None;
                fill(&mut ic, it)?;
                if builditems.jewels.is_none() {
                    builditems.jewels = Some(vec![]);
                }
                builditems.jewels.as_mut().unwrap().push(ic.unwrap());
            }
            _ => {}
        }
    }

    builditems.gems = skillset
        .gems()
        .into_iter()
        .map(|it| {
            let mut ic = None;
            fill(&mut ic, &it).unwrap();
            ic
        })
        .collect();

    Ok(builditems)
}

#[cfg(test)]
mod tests {
    use crate::build_calculation::item::ItemInfo;

    use super::import_build_from_pob_first_itemset;
    use pob::Pob;
    // use ::build_calculation::required_item::ItemInfo;

    const POB: &str = include_str!("pob.xml");

    #[test]
    fn check_import_items() -> anyhow::Result<()> {
        let pob = Pob::new(POB);
        let first_itemset = pob.as_document()?.get_first_itemset()?;
        let weapon = first_itemset
            .items()
            .iter()
            .find(|it| it.name == "Cataclysm Mark")
            .unwrap();
        let buildinfo = import_build_from_pob_first_itemset(&pob)?;
        assert!(!buildinfo.weapon1.is_none());

        match buildinfo.weapon1.as_ref().unwrap().info {
            ItemInfo::Weapon { quality, .. } => {
                assert_eq!(quality, 20);
            }
            _ => {
                panic!("wrong item type")
            }
        }

        println!("{:?}", weapon);
        println!("{:?}", buildinfo.weapon1);
        Ok(())
    }
}
