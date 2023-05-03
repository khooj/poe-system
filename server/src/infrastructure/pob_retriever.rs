use anyhow::Result;
use pob::pob::Pob;
use url::Url;
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

#[derive(Clone)]
pub struct HttpPobRetriever {
    client: reqwest::Client,
    host: Option<Url>,
}

impl HttpPobRetriever {
    pub fn new() -> HttpPobRetriever {
        HttpPobRetriever {
            client: reqwest::Client::new(),
            host: None,
        }
    }

    pub fn new_with_host(host: &str) -> Result<HttpPobRetriever> {
        let client = reqwest::Client::new();
        let host = Url::parse(host)?;
        Ok(HttpPobRetriever {
            client,
            host: Some(host),
        })
    }

    pub async fn get_pob(&self, url: &str) -> Result<Pob> {
        let url = if self.host.is_some() {
            let url = Url::parse(url)?;
            let host = self.host.as_ref().unwrap();
            host.join(url.path())?.to_string()
        } else {
            let pastebin = PastebinBuildUrl::new(url)?;
            pastebin.pastebin_raw_url()
        };

        let resp = self.client.get(&url).send().await?;

        let body = resp.text().await?;

        Ok(Pob::from_pastebin_data(body)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_api() {
        let _ = HttpPobRetriever::new();
        let _ = HttpPobRetriever::new_with_host("http://example.org").expect("fail");
    }
}
