use crate::domain::item::{Item, SimilarityScore};
use crate::domain::pob::pob::Pob;
use crate::domain::types::Class;
use crate::domain::PastebinBuildUrl;
use crate::infrastructure::repositories::postgres::build_repository::{
    Build, BuildItems, BuildRepository,
};
use crate::infrastructure::repositories::postgres::raw_item_repository::RawItemRepository;
use crate::infrastructure::repositories::postgres::task_repository::{
    Task, TaskRepository, TaskType,
};
use crate::infrastructure::repositories::postgres::PgTransaction;
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::convert::TryInto;
use tracing::{debug, trace, error, warn, info, instrument};

#[derive(Deserialize, Serialize)]
struct CalculateBuildTaskData {
    url: String,
    itemset: String,
    league: String,
}

#[derive(Clone)]
pub struct BuildCalculating {
    repository: RawItemRepository,
    tasks_repository: TaskRepository,
    build_repository: BuildRepository,
    http_client: reqwest::Client,
}

impl BuildCalculating {
    pub fn new(
        repository: RawItemRepository,
        tasks_repository: TaskRepository,
        build_repository: BuildRepository,
    ) -> Self {
        BuildCalculating {
            repository,
            tasks_repository,
            build_repository,
            http_client: reqwest::Client::new(),
        }
    }
}

impl BuildCalculating {
    #[instrument(err, skip(self))]
    pub async fn add_build_for_calculating(
        &self,
        url: &str,
        itemset: &str,
        league: &str,
    ) -> Result<String> {
        let mut tr = self.tasks_repository.begin().await?;
        let id = self
            .tasks_repository
            .new_task(
                &mut tr,
                Task::new(
                    TaskType::CalculateBuild,
                    serde_json::to_value(CalculateBuildTaskData {
                        url: url.to_string(),
                        itemset: itemset.to_string(),
                        league: league.to_string(),
                    })?,
                ),
            )
            .await?;

        tr.commit().await?;
        Ok(id)
    }

    #[instrument(err, skip(self))]
    pub async fn get_calculated_build(&self, id: &str) -> Result<Build> {
        Ok(self.build_repository.get_build(id).await?)
    }

    #[instrument(err, skip(self))]
    pub async fn calculate_next_build(&self) -> Result<()> {
        let tasks = self.tasks_repository.get_latest_tasks(1).await?;
        if tasks.is_empty() {
            debug!("no new tasks found, iterating");
            return Ok(());
        }
        let task = tasks
            .first()
            .ok_or(anyhow!("can't get first task to process"))?;
        let mut tr = self.tasks_repository.begin().await?;

        self.calculate_build(&mut tr, task).await?;
        self.tasks_repository
            .remove_tasks(&mut tr, &[task.id])
            .await?;
        tr.commit().await?;
        Ok(())
    }

    #[instrument(err, skip(tr, self))]
    async fn calculate_build(
        &self,
        tr: &mut PgTransaction<'_>,
        Task {
            id,
            task_type,
            data,
            ..
        }: &Task,
    ) -> Result<()> {
        if *task_type != TaskType::CalculateBuild {
            return Err(anyhow!(
                "can't calculate build for other tasks: {:?}",
                task_type
            ));
        }

        let build_info: CalculateBuildTaskData = serde_json::from_value(data.clone())?;
        let build = self.fetch_pob(&build_info.url).await?;
        trace!("got pob");
        let build_doc = build.as_document()?;
        let itemset = if build_info.itemset.is_empty() {
            build_doc.get_first_itemset()?
        } else {
            build_doc.get_itemset(&build_info.itemset)?
        };

        let mut required_items = BuildItems::default();
        let mut found_items = BuildItems::default();
        for item in itemset.items() {
            let item = item.clone();
            trace!("searching similar item");
            let (found_item, score) = self.find_similar_item(&item, &build_info.league).await;
            match item.class {
                Class::Helmet => {
                    required_items.helmet = item;
                    found_items.helmet = found_item;
                }
                Class::BodyArmour => {
                    required_items.body = item;
                    found_items.body = found_item;
                }
                Class::Belt => {
                    required_items.belt = item;
                    found_items.belt = found_item;
                }
                Class::Gloves => {
                    required_items.gloves = item;
                    found_items.gloves = found_item;
                }
                Class::Boots => {
                    required_items.boots = item;
                    found_items.boots = found_item;
                }
                Class::Amulet => {
                    required_items.amulet = item;
                    found_items.amulet = found_item;
                }
                _ => {}
            }
        }

        let result_build = Build::new(
            &id.to_string(),
            &build_info.itemset,
            &build_info.league,
            required_items,
            found_items,
        )?;

        self.build_repository.new_build(tr, result_build).await?;

        Ok(())
    }

    #[instrument(skip(self, item))]
    async fn find_similar_item(&self, item: &Item, league: &str) -> (Item, SimilarityScore) {
        use crate::infrastructure::poe_data::BASE_ITEMS;
        use tokio_stream::StreamExt;

        let mut highscore = SimilarityScore::default();
        let mut result_item = Item::default();
        let alternate_types = match BASE_ITEMS.get_alternate_types(&item.base_type) {
            Some(k) => k,
            None => {
                warn!("can't find alternate types, skipping this item");
                return (result_item, highscore);
            }
        };
        let mut cursor = self.repository.get_raw_items_cursor(&alternate_types, league).await;
        info!("start checking similar items: {:?}", item);
        while let Some(db_item) = cursor.next().await {
            if db_item.is_err() {
                continue;
            }
            trace!("processing new db item");

            let db_item = db_item.unwrap();
            let db_item = if let Ok(k) = db_item.try_into() {
                k
            } else {
                continue;
            };

            let calc = item.calculate_similarity_score(&db_item);
            if calc > highscore {
                info!("get higher score: {:?}", calc);
                highscore = calc;
                result_item = db_item;
            }
        }

        info!("found item {:?} with score {:?}", result_item, highscore);
        (result_item, highscore)
    }

    #[instrument(err, skip(self))]
    async fn fetch_pob(&self, url: &str) -> Result<Pob> {
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
