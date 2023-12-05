use crate::domain::item::Item as DomainItem;
use anyhow::anyhow;
use mods::BASE_TYPES;
use mods::{Category, Class, Hybrid, Influence, ItemLvl, League, Mod, ModType};
use public_stash::models::{Extended, Item as ApiItem};
use sqlx::types::Json;
use sqlx::types::Uuid;
use std::convert::TryInto;
use std::str::FromStr;

#[derive(Debug)]
pub struct RawItem {
    pub id: String,
    pub account_name: String,
    pub stash: String,
    pub item: Json<ApiItem>,
}

impl RawItem {
    pub fn new(id: &str, acc: &str, stash: &str, item: ApiItem) -> Self {
        RawItem {
            id: id.to_owned(),
            account_name: acc.to_owned(),
            stash: stash.to_owned(),
            item: Json(item),
        }
    }

    pub fn as_csvline(&self, del: &str, end: &str) -> String {
        let s = serde_json::to_string(&self.item).expect("can't serialize to json");
        let s = vec![
            self.id.as_str(),
            self.account_name.as_str(),
            self.stash.as_str(),
            s.as_str(),
        ].join(del) + end;
        // println!("{}", s);
        s
    }
}

fn push_mods(mods: &mut Vec<Mod>, source: Option<Vec<String>>, type_: ModType) {
    source.into_iter().for_each(|e| {
        e.iter()
            .for_each(|m| mods.push(Mod::from_str_type(m, type_)));
    });
}

impl TryInto<DomainItem> for RawItem {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<DomainItem, Self::Error> {
        let ApiItem {
            id,
            league,
            item_level,
            identified,
            name,
            extended,
            base_type,
            type_line,
            corrupted,
            influences,
            fractured,
            synthesised,
            utility_mods,
            enchant_mods,
            scourge_mods,
            implicit_mods,
            explicit_mods,
            crafted_mods,
            fractured_mods,
            veiled_mods,
            icon,
            ..
        } = self.item.0;

        let id = id.unwrap_or(Uuid::new_v4().to_string());
        let league: League = league.into();
        let item_lvl: ItemLvl = item_level.into();
        let extended = extended.unwrap_or(Extended {
            category: "amulet".to_string(),
            subcategories: None,
            prefixes: None,
            suffixes: None,
        });
        let category: Category = Category::from_str(&extended.category)?;
        let corrupted = corrupted.unwrap_or(false);
        let mut infs = vec![];
        influences.into_iter().for_each(|e| {
            if e.shaper.is_some() && e.shaper.unwrap() {
                infs.push(Influence::Shaper);
            }
            if e.elder.is_some() && e.elder.unwrap() {
                infs.push(Influence::Elder);
            }
            if e.warlord.is_some() && e.warlord.unwrap() {
                infs.push(Influence::Warlord);
            }
            if e.hunter.is_some() && e.hunter.unwrap() {
                infs.push(Influence::Hunter);
            }
            if e.redeemer.is_some() && e.redeemer.unwrap() {
                infs.push(Influence::Redeemer);
            }
            if e.crusader.is_some() && e.crusader.unwrap() {
                infs.push(Influence::Crusader);
            }
        });
        let influences = infs;
        let fractured = fractured.unwrap_or(false);
        let synthesised = synthesised.unwrap_or(false);
        let mut mods = vec![];
        push_mods(&mut mods, utility_mods, ModType::Utility);
        push_mods(&mut mods, enchant_mods, ModType::Enchant);
        push_mods(&mut mods, scourge_mods, ModType::Scourge);
        push_mods(&mut mods, implicit_mods, ModType::Implicit);
        push_mods(&mut mods, explicit_mods, ModType::Explicit);
        push_mods(&mut mods, crafted_mods, ModType::Crafted);
        push_mods(&mut mods, fractured_mods, ModType::Fractured);
        push_mods(&mut mods, veiled_mods, ModType::Veiled);

        let itemclass = base_type.clone();
        if !BASE_TYPES.contains(&itemclass.as_ref()) {
            return Err(anyhow!("incorrect itemclass: {}", itemclass));
        }
        let class = Class::from_itemclass(&itemclass)?;
        let image_link = icon;

        Ok(DomainItem {
            id,
            league,
            item_lvl,
            identified,
            name,
            category,
            base_type,
            type_line,
            corrupted,
            influences,
            fractured,
            synthesised,
            mods,
            class,
            image_link,
            subcategories: vec![],
            hybrid: Hybrid::default(),
        })
    }
}
