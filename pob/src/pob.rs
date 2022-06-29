use super::parser::{parse_pob_item, ParsedItem};
use base64::{decode_config, URL_SAFE};
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
    TypeError(#[from] mods::TypeError),
    #[error("xml error")]
    XmlError(#[from] roxmltree::Error),
    #[error("base64 error")]
    Base64Error(#[from] base64::DecodeError),
    #[error("io")]
    IoError(#[from] std::io::Error),
    #[error("int parse")]
    ParseIntError(#[from] core::num::ParseIntError),
}
pub struct Pob {
    original: String,
}

impl<'a> Pob {
    pub fn new(data: String) -> Pob {
        Pob { original: data }
    }

    pub fn from_pastebin_data(data: String) -> Result<Pob, PobError> {
        let tmp = decode_config(data, URL_SAFE)?;
        let mut decoder = ZlibDecoder::new(&tmp[..]);
        let mut s = String::new();
        decoder.read_to_string(&mut s)?;
        Ok(Pob { original: s })
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
    items: Vec<ParsedItem>,
}

impl ItemSet {
    fn try_from(node: &Node, items_map: &HashMap<i32, ParsedItem>) -> Result<ItemSet, PobError> {
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
            items.push(items_map.get(&id).unwrap_or(&ParsedItem::default()).clone());
        }

        Ok(ItemSet {
            id,
            title: String::from(title),
            items,
        })
    }

    pub fn get_nth_item(&self, nth: usize) -> Option<&ParsedItem> {
        self.items.iter().nth(nth)
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn items(&self) -> &Vec<ParsedItem> {
        &self.items
    }
}

#[derive(Debug)]
pub struct PobDocument<'a> {
    doc: Document<'a>,
}

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
                let (_, itm) = parse_pob_item::<VerboseError<&str>>(&item.text().unwrap_or(""))
                    .expect("can't parse item");
                items.insert(id, itm);
            }
        }

        for set in items_node.descendants() {
            if set.tag_name().name() == "ItemSet" {
                if let Ok(s) = ItemSet::try_from(&set, &items) {
                    itemsets.push(s);
                }
            }
        }

        itemsets
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

#[cfg(test)]
mod tests {
    const TESTPOB: &'static str = include_str!("pob.txt");
    const TESTPOB2: &'static str = include_str!("pob2.txt");
    const TESTPOB3: &'static str = include_str!("pob3.txt");

    use super::Pob;

    #[test]
    fn parse_pob() -> Result<(), anyhow::Error> {
        dotenv::dotenv().ok();
        let pob = Pob::from_pastebin_data(TESTPOB.to_owned())?;
        let doc = pob.as_document()?;
        let _ = doc.get_item_sets();
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
}
