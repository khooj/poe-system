use super::parser::{parse_pob_item, PobItem as ParsedItem};
use crate::domain::item::{Category, Item, ItemLvl, League, Mod, ModType, Rarity};
use base64::{decode_config, URL_SAFE};
use flate2::read::ZlibDecoder;
use roxmltree::{Document, Node};
use std::{collections::HashMap, io::Read};
use std::{
    convert::{TryFrom, TryInto},
    str::FromStr,
};

pub struct Pob {
    original: String,
}

impl TryFrom<&str> for Pob {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let tmp = decode_config(value, URL_SAFE)?;
        let mut decoder = ZlibDecoder::new(&tmp[..]);
        let mut s = String::new();
        decoder.read_to_string(&mut s)?;
        Ok(Pob::new(s))
    }
}

impl<'a> Pob {
    pub fn new(data: String) -> Pob {
        Pob { original: data }
    }

    pub fn as_document(&'a self) -> Result<PobDocument<'a>, anyhow::Error> {
        let doc = Document::parse(&self.original)?;
        Ok(PobDocument { doc })
    }
}

pub struct ItemSet {
    title: String,
    id: i32,
    items: Vec<Item>,
}

impl ItemSet {
    fn try_from(node: &Node, items_map: &HashMap<i32, Item>) -> Result<ItemSet, anyhow::Error> {
        let id = node.attribute("id").unwrap();
        let id = i32::from_str(id)?;
        let title = node.attribute("title").map_or("default", |v| v);
        let mut items = vec![];

        for item in node.descendants() {
            let id = item.attribute("itemId").map_or("-1", |v| v);
            let id = i32::from_str(id)?;
            if id == -1 || id == 0 {
                continue;
            }
            items.push(items_map.get(&id).unwrap().clone());
        }

        Ok(ItemSet {
            id,
            title: String::from(title),
            items,
        })
    }
}

impl TryInto<Item> for ParsedItem {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<Item, Self::Error> {
        let rarity: Rarity = self.rarity.try_into()?;
        let item_lvl: ItemLvl = self.item_lvl.into();
        let mut mods: Vec<Mod> = self
            .implicits
            .iter()
            .map(|e| Mod::from_str(e, ModType::Implicit))
            .collect();
        mods.extend(
            self.affixes
                .iter()
                .map(|e| Mod::from_str(e, ModType::Explicit)),
        );
        Ok(Item {
            league: League::Standard,
            item_lvl,
            rarity,
            name: self.name.to_owned(),
            base_type: self.base_line.to_owned(),
            ..Item::default()
        })
    }
}

pub struct PobItem(String);

impl TryFrom<PobItem> for Item {
    type Error = anyhow::Error;

    fn try_from(value: PobItem) -> Result<Self, Self::Error> {
        let (_, parsed_item) = parse_pob_item::<()>(&value.0)?;

        Ok(parsed_item.try_into()?)
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
        let items_node = nodes.find(|&x| x.tag_name().name() == "Items").unwrap();

        for item in items_node.descendants() {
            if item.tag_name().name() == "Item" {
                let id = item.attribute("id").unwrap();
                let id = i32::from_str(id).unwrap();
                let item_parsed = Item::try_from(PobItem(item.text().unwrap().to_owned())).unwrap();
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
}

#[cfg(test)]
mod tests {
    const TESTPOB: &'static str = include_str!("pob.txt");

    use super::{Pob, PobDocument};
    use std::convert::TryFrom;

    #[test]
    fn parse_pob() -> Result<(), anyhow::Error> {
        let pob = Pob::try_from(TESTPOB)?;
        let doc = pob.as_document()?;
        Ok(())
    }

    #[test]
    fn check_itemsets() -> Result<(), anyhow::Error> {
        let pob = Pob::try_from(TESTPOB)?;
        let doc = pob.as_document()?;
        let itemsets = doc.get_item_sets();
        assert_eq!(itemsets.len(), 3);
        assert_eq!(itemsets[0].title(), "default");
        assert_eq!(itemsets[0].id(), 1);
        assert_eq!(itemsets[1].title(), "TR Champ High Budget");
        assert_eq!(itemsets[1].id(), 2);
        assert_eq!(itemsets[0].items().len(), 18);
        Ok(())
    }
}