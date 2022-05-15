use super::parser::Mod;

#[derive(Clone, Default, Debug)]
pub struct Item {
    pub name: String,
    pub item_lvl: i32,
    pub league: String,
    pub base_type: String,
    pub mods: Vec<Mod>,
    pub class: String,
}

impl Into<Item> for ParsedItem {
    fn into(self) -> Item {
        let Item {
            name,
            item_lvl,
            league,
            base_type,
            mods,
            class,
        } = self;
        Item {
            name,
            item_lvl: item_lvl.into(),
            league: league.into(),
            base_type,
            mods: mods
                .into_iter()
                .map(|el| Mod::from_str_u8(&el.text, el.type_ as u8))
                .collect(),
            class: Class::from_itemclass(&class).expect("can't get class from itemclass"),
            ..PobItem::default()
        }
    }
}