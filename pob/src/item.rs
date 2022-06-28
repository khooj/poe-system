use mods::Mod;

#[derive(Clone, Default, Debug)]
pub struct Item {
    pub name: String,
    pub base_type: String,
    pub mods: Vec<Mod>,
    pub class: String,
}
