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

pub struct BaseTypesData {
    base_type_to_idx: HashMap<String, usize>,
    types: Vec<Vec<String>>,
}

impl BaseTypesData {
    pub fn new(filename: &str) -> Self {
        let items = initialize_base_items(filename);
        let mut types: HashMap<String, Vec<String>> = HashMap::new();
        for item in items {
            let v = types.entry(item.item_class).or_default();
            v.push(item.name.clone());
        }

        let mut base_type_to_idx = HashMap::new();
        let mut types_v = Vec::with_capacity(types.len());
        types_v.resize(types.len(), vec![]);

        for (i, (_, v)) in types.into_iter().enumerate() {
            for item in &v {
                base_type_to_idx.entry(item.clone()).or_insert(i);
            }

            types_v[i] = v;
        }

        BaseTypesData {
            base_type_to_idx,
            types: types_v,
        }
    }

    pub fn get_alternate_types(&self, base_type: &str) -> Vec<&str> {
        let idx = self.base_type_to_idx[base_type];
        self.types
            .get(idx)
            .unwrap()
            .iter()
            .map(|e| e.as_str())
            .collect()
    }
}

lazy_static! {
    pub static ref BASE_ITEMS: BaseTypesData = BaseTypesData::new("base_items.min.json");
}
