#[cfg(feature = "parsing")]
use crate::parser::{parse_pob_item, ParsedItem};

use base64::{decode_config, URL_SAFE};
use domain::data::BaseItems;
use domain::item::{
    types::{Category, Property, Subcategory, TypeError},
    Item,
};
use flate2::read::ZlibDecoder;
use nom::error::VerboseError;
use roxmltree::{Document, Node};
use thiserror::Error;
use tracing::{error, info};

use std::str::FromStr;
use std::{collections::HashMap, io::Read};

#[derive(Error, Debug)]
pub enum PobError {
    #[error("wrong basetype: {0}")]
    WrongBasetype(String),
    #[error("parse error: {0}")]
    Parse(String),
    #[error("itemset not found: {0}")]
    ItemsetNotFound(i32),
    #[error("itemset name not found: {0}")]
    ItemsetNameNotFound(String),

    #[error("type error: {0}")]
    TypeError(#[from] TypeError),
    #[error("xml error")]
    XmlError(#[from] roxmltree::Error),
    #[error("base64 error")]
    Base64Error(#[from] base64::DecodeError),
    #[error("io")]
    IoError(#[from] std::io::Error),
    #[error("int parse")]
    ParseIntError(#[from] core::num::ParseIntError),
}
#[derive(Clone)]
pub struct Pob {
    original: String,
}

impl<'a> Pob {
    pub fn new<T: AsRef<str>>(data: T) -> Pob {
        Pob {
            original: data.as_ref().to_string(),
        }
    }

    pub fn from_pastebin_data(data: String) -> Result<Pob, PobError> {
        let tmp = decode_config(data, URL_SAFE)?;
        let mut decoder = ZlibDecoder::new(&tmp[..]);
        let mut s = String::new();
        decoder.read_to_string(&mut s)?;
        Ok(Pob { original: s })
    }

    pub fn get_original(&self) -> String {
        self.original.clone()
    }

    pub fn as_document(&'a self) -> Result<PobDocument<'a>, PobError> {
        let doc = Document::parse(&self.original)?;
        Ok(PobDocument { doc })
    }
}

#[derive(Clone, Debug)]
pub struct ItemSet {
    title: String,
    id: i32,
    items: Vec<Item>,
}

impl ItemSet {
    fn try_from(node: &Node, items_map: &HashMap<i32, Item>) -> Result<ItemSet, PobError> {
        let id = node
            .attribute("id")
            .ok_or(PobError::Parse("can't get id from node".into()))?;
        let id = i32::from_str(id)?;
        let title = node.attribute("title").map_or("default", |v| v);
        let mut items = vec![];

        for item in node.descendants() {
            let id = item.attribute("itemId").map_or("-1", |v| v);
            let id = i32::from_str(id)?;
            if id == -1 || id == 0 {
                continue;
            }
            items.push(items_map.get(&id).unwrap_or(&Item::default()).clone());
        }

        Ok(ItemSet {
            id,
            title: String::from(title),
            items,
        })
    }

    pub fn get_nth_item(&self, nth: usize) -> Option<&Item> {
        self.items.get(nth)
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn items(&self) -> &Vec<Item> {
        &self.items
    }
}

#[derive(Clone, Debug)]
pub struct SkillSet {
    title: String,
    id: i32,
    skills: Vec<Item>,
}

impl SkillSet {
    fn try_from(
        node: &Node,
        default_gem_quality: Option<i32>,
        default_gem_level: Option<i32>,
    ) -> Result<SkillSet, PobError> {
        if node.has_tag_name("SkillSet") {
            let title = node.attribute("title").map_or("default", |v| v);
            let id = node
                .attribute("id")
                .ok_or(PobError::Parse("cannot get id from skillset".into()))?;
            let id = i32::from_str(id)?;

            let skills = SkillSet::parse_skills(
                node.descendants()
                    .filter(|&x| x.tag_name().name() == "Gem")
                    .collect(),
                default_gem_quality,
                default_gem_level,
            )?;

            Ok(SkillSet {
                title: title.to_string(),
                id,
                skills,
            })
        } else {
            let skills = SkillSet::parse_skills(
                node.next_siblings()
                    .flat_map(|n| n.descendants().filter(|&x| x.has_tag_name("Gem")))
                    .collect(),
                default_gem_quality,
                default_gem_level,
            )?;
            Ok(SkillSet {
                title: "default".to_string(),
                id: 0,
                skills,
            })
        }
    }

    fn parse_skills(
        nodes: Vec<Node>,
        default_gem_quality: Option<i32>,
        default_gem_level: Option<i32>,
    ) -> Result<Vec<Item>, PobError> {
        Ok(nodes
            .into_iter()
            .filter_map(|n| {
                if !n.has_attribute("gemId") {
                    return None;
                }

                let gem_id = n.attribute("gemId").unwrap();
                let quality = n
                    .attribute("quality")
                    .and_then(|q| i32::from_str(q).ok())
                    .or(default_gem_quality)
                    .unwrap();
                let level = n
                    .attribute("level")
                    .and_then(|q| i32::from_str(q).ok())
                    .or(default_gem_level)
                    .unwrap();
                let info = BaseItems::get_by_id(gem_id).unwrap();
                Some(Item {
                    name: info.name.clone(),
                    base_type: info.name,
                    category: Category::Gems,
                    subcategories: Subcategory::Gem,
                    quality,
                    properties: vec![Property {
                        name: "Level".to_string(),
                        value: Some(format!("{}", level)),
                        augmented: false,
                    }],
                    ..Default::default()
                })
            })
            .collect())
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn gems(&self) -> Vec<Item> {
        self.skills.clone()
    }
}

#[derive(Debug)]
pub struct PobDocument<'a> {
    doc: Document<'a>,
}

#[cfg(feature = "parsing")]
impl<'a> PobDocument<'a> {
    pub fn get_item_sets(&self) -> Vec<ItemSet> {
        let mut itemsets = vec![];
        let mut items: HashMap<i32, ParsedItem> = HashMap::new();
        let mut nodes = self.doc.descendants();
        let items_node = match nodes.find(|&x| x.tag_name().name() == "Items") {
            Some(k) => k,
            None => {
                info!("pob does not have any items");
                return vec![];
            }
        };

        for item in items_node.descendants() {
            if item.tag_name().name() == "Item" {
                let id = item.attribute("id").unwrap();
                let id = i32::from_str(id).unwrap();
                let (_, itm) = parse_pob_item::<VerboseError<&str>>(item.text().unwrap_or(""))
                    .expect("can't parse item");
                items.insert(id, itm);
            }
        }

        let mut items_processed = HashMap::with_capacity(items.len());

        for (ii, parsed_item) in items {
            let item = parsed_item.item;
            items_processed.entry(ii).or_insert(item);
        }

        for set in items_node.descendants() {
            if set.tag_name().name() == "ItemSet" {
                if let Ok(s) = ItemSet::try_from(&set, &items_processed) {
                    itemsets.push(s);
                }
            }
        }

        itemsets
    }

    pub fn get_skillsets(&self) -> Vec<SkillSet> {
        let mut nodes = self.doc.descendants();
        let skills_node = nodes.find(|&x| x.has_tag_name("Skills")).unwrap();
        let default_gem_quality = skills_node.attribute("defaultGemQuality").and_then(|s| {
            if s == "normalMaximum" {
                Some(20)
            } else {
                i32::from_str(s).ok()
            }
        });
        let default_gem_level = skills_node.attribute("defaultGemLevel").and_then(|s| {
            if s == "normalMaximum" {
                Some(20)
            } else {
                i32::from_str(s).ok()
            }
        });
        let child = skills_node.first_element_child().unwrap();
        let skillsets = if child.has_tag_name("SkillSet") {
            child
                .next_siblings()
                .filter_map(|sk| {
                    if sk.is_text() {
                        None
                    } else {
                        Some(
                            SkillSet::try_from(&sk, default_gem_quality, default_gem_level)
                                .unwrap(),
                        )
                    }
                })
                .collect()
        } else {
            vec![SkillSet::try_from(&child, default_gem_quality, default_gem_level).unwrap()]
        };
        skillsets
    }

    pub fn get_first_itemset(&self) -> Result<ItemSet, PobError> {
        let itemsets = self.get_item_sets();

        itemsets
            .into_iter()
            .nth(0)
            .ok_or(PobError::ItemsetNotFound(0))
    }

    pub fn get_itemset(&self, title: &str) -> Result<ItemSet, PobError> {
        if title.is_empty() {
            return self.get_first_itemset();
        }

        let itemsets = self.get_item_sets();

        itemsets
            .into_iter()
            .find(|e| e.title == title)
            .ok_or(PobError::ItemsetNameNotFound(title.into()))
    }
}

impl<'a> PobDocument<'a> {
    pub fn get_itemsets_list(&self) -> Result<Vec<String>, PobError> {
        let mut itemsets = vec![];
        let mut nodes = self.doc.descendants();
        let items_node = match nodes.find(|&x| x.tag_name().name() == "Items") {
            Some(k) => k,
            None => {
                info!("pob does not have any items");
                return Ok(vec![]);
            }
        };

        for set in items_node.descendants() {
            if set.tag_name().name() == "ItemSet" {
                let title = set.attribute("title").map_or("default", |v| v);
                itemsets.push(title.to_string());
            }
        }

        Ok(itemsets)
    }

    pub fn get_skillsets_list(&self) -> Result<Vec<String>, PobError> {
        let mut nodes = self.doc.descendants();
        let skills_node = nodes.find(|&x| x.has_tag_name("Skills")).unwrap();
        let child = skills_node.first_element_child().unwrap();
        let skillsets = if child.has_tag_name("SkillSet") {
            child
                .next_siblings()
                .filter_map(|sk| {
                    if sk.is_text() {
                        None
                    } else {
                        Some(sk.attribute("title").unwrap().to_string())
                    }
                })
                .collect()
        } else {
            vec!["default".to_string()]
        };
        Ok(skillsets)
    }
}

#[cfg(test)]
mod tests {
    const TESTPOB: &str = include_str!("pob.txt");
    const TESTPOB2: &str = include_str!("pob2.txt");
    const TESTPOB3: &str = include_str!("pob3.txt");
    const TESTPOB_GEMS: &str = include_str!("pob_gems.txt");

    use super::Pob;

    #[test]
    fn parse_pob() -> Result<(), anyhow::Error> {
        dotenv::dotenv().ok();
        let pob = Pob::from_pastebin_data(TESTPOB.to_owned())?;
        let doc = pob.as_document()?;
        let _ = doc.get_item_sets();
        let _ = doc.get_skillsets();
        Ok(())
    }

    #[test]
    fn parse_pob2() -> Result<(), anyhow::Error> {
        dotenv::dotenv().ok();
        let pob = Pob::from_pastebin_data(TESTPOB2.to_owned())?;
        let doc = pob.as_document()?;
        // let sets = doc.get_item_sets();
        // println!("sets names: {:?}", sets.iter().map(|e| &e.title).collect::<Vec<&String>>());
        let set = doc.get_first_itemset()?;
        println!("first itemset: {:?}", set);
        Ok(())
    }

    #[test]
    fn parse_pob3() -> Result<(), anyhow::Error> {
        dotenv::dotenv().ok();
        let pob = Pob::from_pastebin_data(TESTPOB3.to_owned())?;
        let doc = pob.as_document()?;
        // let sets = doc.get_item_sets();
        // println!("sets names: {:?}", sets.iter().map(|e| &e.title).collect::<Vec<&String>>());
        let set = doc.get_first_itemset()?;
        println!("first itemset: {:?}", set);
        for _ in set.items() {}
        Ok(())
    }

    #[test]
    fn check_itemsets() -> Result<(), anyhow::Error> {
        dotenv::dotenv().ok();
        let pob = Pob::from_pastebin_data(TESTPOB.to_owned())?;
        let doc = pob.as_document()?;
        let itemsets = doc.get_item_sets();
        assert_eq!(itemsets.len(), 3);
        assert_eq!(itemsets[0].title(), "default");
        assert_eq!(itemsets[0].id(), 1);
        assert_eq!(itemsets[1].title(), "TR Champ High Budget");
        assert_eq!(itemsets[1].id(), 2);
        assert_eq!(itemsets[0].items().len(), 18);
        println!("{:?}", itemsets[0].items);
        Ok(())
    }

    #[test]
    fn check_skillsets_default() -> Result<(), anyhow::Error> {
        let pob = Pob::from_pastebin_data(TESTPOB.to_owned())?;
        let doc = pob.as_document()?;
        let skillsets = doc.get_skillsets();
        assert_eq!(skillsets.len(), 1);
        // assert_eq!(itemsets[0].title(), "default");
        // assert_eq!(itemsets[0].id(), 1);
        Ok(())
    }

    #[test]
    fn check_skillsets_many() -> Result<(), anyhow::Error> {
        let pob = Pob::from_pastebin_data(TESTPOB_GEMS.to_owned().trim().to_string())?;
        let doc = pob.as_document()?;
        let skillsets = doc.get_skillsets();
        assert_eq!(skillsets.len(), 7);
        // assert_eq!(itemsets[0].title(), "default");
        // assert_eq!(itemsets[0].id(), 1);
        Ok(())
    }
}
