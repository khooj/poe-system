use std::collections::HashMap;

use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;

fn string_if_null<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let r: Option<String> = Option::deserialize(deserializer)?;
    Ok(r.unwrap_or_default())
}

#[derive(Deserialize, Debug)]
pub struct Object {
    pub id: u32,
    pub name: String,
    #[serde(rename = "baseType")]
    pub base_type: String,
    #[serde(rename = "itemClass")]
    pub item_class: u32,
    #[serde(rename = "chaosValue")]
    pub chaos_value: f32,
    #[serde(rename = "divineValue")]
    pub divine_value: f32,
}

#[derive(Deserialize, Debug)]
pub struct Response {
    pub lines: Vec<Object>,
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_MODELS: &str = include_str!("test_models.json");

    #[test]
    fn de() {
        let _: Response = serde_json::from_str(TEST_MODELS).unwrap();
    }
}
