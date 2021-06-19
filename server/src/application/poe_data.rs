use lazy_static::lazy_static;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::iter::FromIterator;

#[derive(Deserialize)]
pub struct ModInfo {
    pub string: String,
}

#[derive(Deserialize)]
pub struct Mod {
    pub english: Vec<ModInfo>,
    pub ids: Vec<String>,
}

fn initialize_mods_impl(filename: &str) -> Result<Vec<Mod>, anyhow::Error> {
    let current_dir = std::env::current_dir()?;
    let jsons = current_dir.join("data");
    let jsonfile = jsons.join(filename);

    let f = File::open(&jsonfile)?;
    let buf = BufReader::new(f);

    let result: Vec<Mod> = serde_json::from_reader(buf)?;
    Ok(result)
}

fn initialize_mods(filename: &str) -> Vec<Mod> {
    match initialize_mods_impl(filename) {
        Ok(k) => k,
        Err(e) => panic!("{}", e),
    }
}

lazy_static! {
    pub static ref ACTIVE_SKILL_GEM: Vec<Mod> = initialize_mods("active_skill_gems.min.json");
    pub static ref ADVANCED_MOD: Vec<Mod> = initialize_mods("advanced_mod.min.json");
    pub static ref HASH_ACTIVE_SKILL_GEM: HashMap<String, usize> = {
        HashMap::from_iter(
            ACTIVE_SKILL_GEM
                .iter()
                .enumerate()
                .map(|(i, v)| (v.english[0].string.clone(), i)),
        )
    };
}
