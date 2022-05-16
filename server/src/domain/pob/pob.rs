use super::{parser::{parse_pob_item, ParsedItem}, item::Item};
use crate::{
    domain::{
        types::{Class, ItemLvl, League, Mod, ModType, Rarity},
    },
    infrastructure::poe_data::BASE_ITEMS,
};
use base64::{decode_config, URL_SAFE};
use flate2::read::ZlibDecoder;
use roxmltree::{Document, Node};
use std::{collections::HashMap, io::Read};
use std::{
    convert::{TryFrom, TryInto},
    str::FromStr,
};
use tracing::{error, info};
use anyhow::anyhow;
use nom::error::VerboseError;

pub struct Pob {
    original: String,
}

impl<'a> Pob {
    pub fn new(data: String) -> Pob {
        Pob { original: data }
    }

    pub fn from_pastebin_data(data: String) -> Result<Pob, anyhow::Error> {
        let tmp = decode_config(data, URL_SAFE)?;
        let mut decoder = ZlibDecoder::new(&tmp[..]);
        let mut s = String::new();
        decoder.read_to_string(&mut s)?;
        Ok(Pob { original: s })
    }

    pub fn as_document(&'a self) -> Result<PobDocument<'a>, anyhow::Error> {
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
    fn try_from(node: &Node, items_map: &HashMap<i32, Item>) -> Result<ItemSet, anyhow::Error> {
        let id = node
            .attribute("id")
            .ok_or(anyhow::anyhow!("cant get itemset id"))?;
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
        self.items.iter().nth(nth)
    }
}

impl TryInto<Item> for ParsedItem {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<Item, Self::Error> {
        let rarity: Rarity = self.rarity.try_into()?;
        let item_lvl: ItemLvl = self.item_lvl.into();

        let itemclass = BASE_ITEMS.get_item_class(&self.base_line).ok_or(anyhow!(
            "can't get itemclass from basetype: {}",
            self.base_line
        ))?;
        Ok(Item {
            league: League::Standard,
            item_lvl,
            name: self.name.to_owned(),
            base_type: self.base_line,
            mods: self.affixes,
            class: match Class::from_itemclass(itemclass) {
                Ok(k) => k,
                Err(e) => {
                    error!(
                        "can't get class from given itemclass: {} with err: {}",
                        itemclass, e
                    );
                    Class::default()
                }
            },
            ..Item::default()
        })
    }
}

impl ItemSet {
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

#[derive(Debug)]
pub struct PobDocument<'a> {
    doc: Document<'a>,
}

impl<'a> PobDocument<'a> {
    pub fn get_item_sets(&self) -> Vec<ItemSet> {
        let mut itemsets = vec![];
        let mut items: HashMap<i32, Item> = HashMap::new();
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
                let (_, itm) =
                    parse_pob_item::<VerboseError<&str>>(&item.text().unwrap_or("")).expect("can't parse item");
                let item_parsed = match itm.try_into() {
                    Ok(k) => k,
                    Err(e) => {
                        error!(
                            "cant convert into domain item: {} with err: {}",
                            item.text().unwrap(),
                            e
                        );
                        continue;
                    }
                };
                items.insert(id, item_parsed);
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

    pub fn get_first_itemset(&self) -> Result<ItemSet, anyhow::Error> {
        let itemsets = self.get_item_sets();

        itemsets
            .into_iter()
            .nth(0)
            .ok_or(anyhow::anyhow!("pob dont have itemsets"))
    }

    pub fn get_itemset(&self, title: &str) -> Result<ItemSet, anyhow::Error> {
        if title.is_empty() {
            return self.get_first_itemset();
        }

        let itemsets = self.get_item_sets();

        itemsets
            .into_iter()
            .find(|e| e.title == title)
            .ok_or(anyhow::anyhow!("cant find itemset"))
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
        for i in set.items() {}
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
