pub mod item;

use anyhow::Result;
use std::convert::{AsRef, Into};

#[derive(Debug)]
pub struct PastebinToken(String);

impl PastebinToken {
    pub fn new(s: String) -> Self {
        PastebinToken(s)
    }
}

#[derive(Debug, Clone)]
pub struct PastebinBuildUrl(String);

impl PastebinBuildUrl {
    pub fn new(url: &str) -> Result<Self> {
        let token = url.split('/').collect::<Vec<_>>();
        let token = token
            .last()
            .ok_or(anyhow::anyhow!("wrong pastebin url: {}", url))?;

        Ok(Self(token.to_string()))
    }

    pub fn from_token(token: PastebinToken) -> Self {
        Self(token.0)
    }

    pub fn pastebin_raw_url(&self) -> String {
        format!("https://pastebin.com/raw/{}", &self.0)
    }
}

impl AsRef<str> for PastebinBuildUrl {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Into<String> for PastebinBuildUrl {
    fn into(self) -> String {
        self.as_ref().to_owned()
    }
}

#[cfg(test)]
mod test {}
