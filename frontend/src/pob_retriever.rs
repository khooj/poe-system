use super::make_request::get_text_body;
use super::make_request::Error;
use super::pob::Pob;

#[derive(Clone)]
pub struct HttpPobRetriever {}

#[derive(Debug, Clone)]
pub struct PastebinBuildUrl(String);

impl PastebinBuildUrl {
    pub fn new(url: &str) -> Result<Self, Error> {
        let token = url.split('/').collect::<Vec<_>>();
        let token = token
            .last()
            .ok_or(Error::CustomError(format!("wrong pastebin url: {}", url)))?;

        Ok(Self(token.to_string()))
    }

    pub fn pastebin_raw_url(&self) -> String {
        format!("https://pastebin.com/raw/{}", &self.0)
    }
}

impl HttpPobRetriever {
    pub fn new() -> HttpPobRetriever {
        HttpPobRetriever {}
    }

    pub async fn get_pob(&self, url: &str) -> Result<Pob, Error> {
        let pastebin = PastebinBuildUrl::new(url)?;
        let url = pastebin.pastebin_raw_url();

        let resp = get_text_body(&url).await?;

        let body = resp
            .text()
            .await
            .map_err(|e| Error::CustomError(e.to_string()))?;

        Ok(Pob::from_pastebin_data(body).map_err(|e| Error::CustomError(e.to_string()))?)
    }
}
