use serde::{Deserialize, Serialize};
use async_trait::async_trait;

#[derive(Deserialize, Serialize)]
pub struct Item {
    pub verified: bool,
    pub w: u32,
    pub h: u32,
    pub icon: String,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PublicStashChange {
    pub id: String,
    pub public: bool,
    pub account_name: Option<String>,
    pub last_character_name: Option<String>,
    pub stash: Option<String>,
    pub stash_type: String,
    pub league: Option<String>,
    pub items: Vec<Item>,
}

#[derive(Deserialize, Serialize)]
pub struct PublicStashData {
    #[serde(rename = "nextChangeId")]
    pub next_change_id: String,
    pub stashes: Vec<PublicStashChange>,
}

#[async_trait]
pub trait PublicStashRetriever {
    type Error;

    async fn get_latest_stash(&mut self, id: Option<&str>) -> Result<PublicStashData, Self::Error>;
}

#[cfg(test)]
mod test {
    use super::PublicStashChange;

    const EXAMPLE_STASH_CHANGE: &str = include_str!("example-stash.json");
    #[test]
    fn deserializing_public_stash_change() -> Result<(), anyhow::Error> {
        let _: PublicStashChange = serde_json::from_str(&EXAMPLE_STASH_CHANGE)?;
        Ok(())
    }
}
