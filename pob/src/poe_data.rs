use anyhow::{anyhow, Result};
use lazy_static::lazy_static;
use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;
use std::convert::TryInto;
use std::fs::File;
use std::io::BufReader;
use std::iter::FromIterator;
use std::path::PathBuf;

#[derive(Deserialize)]
pub struct ModInfo {
    pub string: String,
}

#[derive(Deserialize)]
pub struct Mod {
    pub english: Vec<ModInfo>,
    pub ids: Vec<String>,
}

#[derive(Deserialize)]
pub struct BaseItem {
    pub item_class: String,
    pub name: String,
    pub tags: Vec<String>,
}

fn initialize_data<T>(filename: &str) -> Result<T>
where
    T: for<'de> Deserialize<'de>,
{
    let data_dir = std::env::var("REPOE_DATA_DIR")?;
    let data_dir = PathBuf::from(data_dir);
    let jsonfile = data_dir.join(filename);

    let f = File::open(&jsonfile)?;
    let buf = BufReader::new(f);

    Ok(serde_json::from_reader(buf)?)
}

fn initialize_base_items(filename: &str) -> Vec<BaseItem> {
    match initialize_data::<Value>(filename) {
        Ok(k) => k
            .as_object()
            .unwrap()
            .iter()
            .filter_map(|(_, v)| BaseItem::deserialize(v).ok())
            .collect(),
        Err(e) => panic!("{}", e),
    }
}

struct PreparedItemInfo {
    // map origin base_type to alternate base_types with same class
    to_alternate_types_by_itemclass: Vec<String>,
    item_class: String,
}

pub struct BaseTypesData {
    base_type_to_data: HashMap<String, PreparedItemInfo>,
}

impl BaseTypesData {
    pub fn new(filename: &str) -> Self {
        let items = initialize_base_items(filename);
        let mut alternate_types: HashMap<String, Vec<String>> = HashMap::new();
        for item in items {
            let v = alternate_types.entry(item.item_class).or_default();
            v.push(item.name.clone());
        }

        let mut base_type_to_data = HashMap::new();

        for (i, (k, v)) in alternate_types.into_iter().enumerate() {
            for item in &v {
                base_type_to_data
                    .entry(item.clone())
                    .or_insert(PreparedItemInfo {
                        to_alternate_types_by_itemclass: v.clone(),
                        item_class: k.clone(),
                    });
            }
        }

        BaseTypesData { base_type_to_data }
    }

    pub fn get_alternate_types(&self, base_type: &str) -> Option<Vec<&str>> {
        match self.base_type_to_data.get(base_type) {
            Some(k) => Some(
                k.to_alternate_types_by_itemclass
                    .iter()
                    .map(|e| e.as_str())
                    .collect(),
            ),
            None => None,
        }
    }

    pub fn get_item_class(&self, base_type: &str) -> Option<&str> {
        match self.base_type_to_data.get(base_type) {
            Some(k) => Some(&k.item_class),
            None => None,
        }
    }

    pub fn get_all_itemclasses(&self) -> Vec<&str> {
        self.base_type_to_data.keys().map(|e| e.as_str()).collect()
    }
}

lazy_static! {
    pub static ref BASE_ITEMS: BaseTypesData = BaseTypesData::new("base_items.min.json");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_base_items_const() -> Result<()> {
        dotenv::dotenv().ok();

        let types = BASE_ITEMS
            .get_alternate_types("Champion Kite Shield")
            .ok_or(anyhow!("err"))?;
        assert!(types.contains(&"Plank Kite Shield"));
        let types = BASE_ITEMS
            .get_alternate_types("The Porcupine")
            .ok_or(anyhow!("err"))?;
        assert!(types.contains(&"The Doctor"));

        let item_class = BASE_ITEMS
            .get_item_class("Champion Kite Shield")
            .ok_or(anyhow!("err"))?;
        assert_eq!(item_class, "Shield");
        Ok(())
    }
}
