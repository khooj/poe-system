use crate::domain::item::{Item, SimilarityScore};
use crate::domain::pob::pob::Pob;
use crate::domain::PastebinBuildUrl;
use crate::infrastructure::repositories::postgres::raw_item_repository::RawItemRepository;
use anyhow::Result;
use tracing::{debug, error, info, instrument};

pub struct BuildCalculating {
    repository: RawItemRepository,
    http_client: reqwest::Client,
}

impl BuildCalculating {
    #[instrument(skip(self))]
    pub async fn find_similar_item(&self, item: &Item) -> (Item, SimilarityScore) {
        use crate::infrastructure::poe_data::BASE_ITEMS;
        use tokio_stream::StreamExt;

        let mut highscore = SimilarityScore::default();
        let mut result_item = Item::default();
        let alternate_types = BASE_ITEMS.get_alternate_types(&item.base_type);
        let mut cursor = self.repository.get_raw_items_cursor(&alternate_types).await;
        while let Some(db_item) = cursor.next().await {
            if db_item.is_err() {
                continue;
            }

            let db_item = db_item.unwrap();
            let db_item = if let Ok(k) = Item::try_from_dbitem(&db_item) {
                k
            } else {
                continue;
            };

            let calc = item.calculate_similarity_score(&db_item);
            if calc > highscore {
                highscore = calc;
                result_item = db_item;
            }
        }

        (result_item, highscore)
    }

    #[instrument(err, skip(self))]
    pub async fn fetch_pob(&self, url: &str) -> Result<Pob> {
        let pastebin = PastebinBuildUrl::new(url)?;
        let resp = self
            .http_client
            .get(&pastebin.pastebin_raw_url())
            .send()
            .await?;

        let body = resp.text().await?;

        Ok(Pob::from_pastebin_data(body)?)
    }
}
