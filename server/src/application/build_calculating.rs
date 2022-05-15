use crate::domain::item::{Item, SimilarityScore};
use crate::domain::types::{Class, ItemLvl, League, Mod, ModType, Rarity};
use crate::infrastructure::poe_data::BASE_ITEMS;
use crate::infrastructure::repositories::postgres::build_repository::{
    Build, BuildItems, BuildRepository,
};
use crate::infrastructure::repositories::postgres::raw_item_repository::RawItemRepository;
use crate::infrastructure::repositories::postgres::task_repository::{
    Task, TaskRepository, TaskType,
};
use crate::infrastructure::repositories::postgres::PgTransaction;
use anyhow::{anyhow, Result};
use pob::pob::parser::ParsedItem;
use pob::pob::{item::Item as PobItem, pob::Pob};
use serde::{Deserialize, Serialize};
use std::convert::TryInto;
use tracing::{debug, error, info, instrument, trace, warn};

impl TryInto<Item> for ParsedItem {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<Item, Self::Error> {
        let rarity: Rarity = self.rarity.try_into()?;
        let item_lvl: ItemLvl = self.item_lvl.into();

        let itemclass = BASE_ITEMS.get_item_class(&self.base_line).ok_or(anyhow!(
            "can't get itemclass from basetype: {}",
            self.base_line
        ))?;
        Ok(Item {
            league: League::Standard,
            item_lvl,
            name: self.name.to_owned(),
            base_type: self.base_line,
            mods: self
                .affixes
                .into_iter()
                .map(|el| Mod::from_str_u8(&el.text, el.type_ as u8))
                .collect(),
            class: match Class::from_itemclass(itemclass) {
                Ok(k) => k,
                Err(e) => {
                    error!(
                        "can't get class from given itemclass: {} with err: {}",
                        itemclass, e
                    );
                    Class::default()
                }
            },
            ..Item::default()
        })
    }
}

#[derive(Deserialize, Serialize)]
struct CalculateBuildTaskData {
    pobdata: String,
    itemset: String,
    league: String,
}

#[derive(Clone)]
pub struct BuildCalculating {
    repository: RawItemRepository,
    tasks_repository: TaskRepository,
    build_repository: BuildRepository,
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
        }
    }
}

impl BuildCalculating {
    #[instrument(err, skip(self))]
    pub async fn add_build_for_calculating(
        &self,
        pobdata: &str,
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
                        pobdata: pobdata.to_string(),
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
    pub async fn get_calculated_build(&self, id: &str) -> Result<Option<Build>> {
        Ok(self.build_repository.get_build(id).await?)
    }

    #[instrument(err, skip(self))]
    pub async fn calculate_next_build(&self) -> Result<()> {
        let tasks = self.tasks_repository.get_latest_tasks(1).await?;
        if tasks.is_empty() {
            trace!("no new tasks found, iterating");
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

        trace!("deserializing task");
        let build_info: CalculateBuildTaskData = serde_json::from_value(data.clone())?;
        let build = Pob::from_pastebin_data(build_info.pobdata)?;
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
            let item: Result<Item> = item.try_into();
            if let Err(e) = item {
                info!(err = %e, "can't convert parsed pob item to domain item");
                continue;
            }
            let item = item.unwrap();

            trace!("searching similar item");
            let (found_item, score) = self.find_similar_item(&item, &build_info.league).await;
            let item: Item = item.into();
            match item.class {
                Class::Helmet => {
                    // println!("required: {:?}\nfound: {:?}", item, found_item);
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
    async fn find_similar_item(&self, item: &PobItem, league: &str) -> (Item, SimilarityScore) {
        use crate::infrastructure::poe_data::BASE_ITEMS;
        use tokio_stream::StreamExt;

        debug!(
            "check required item short: {} {}",
            item.name, item.base_type
        );
        trace!(item = ?item, "checking required item");
        let mut highscore = SimilarityScore::default();
        let mut result_item = Item::default();
        let alternate_types = match BASE_ITEMS.get_alternate_types(&item.base_type) {
            Some(k) => k,
            None => {
                warn!("can't find alternate types, skipping this item");
                return (result_item, highscore);
            }
        };
        trace!("found alternate types: {:?}", alternate_types);
        let mut cursor = self
            .repository
            .get_raw_items_cursor(&alternate_types, league)
            .await;
        while let Some(db_item) = cursor.next().await {
            if db_item.is_err() {
                match db_item {
                    Err(e) => {
                        error!(err = %e, "continue");
                        continue;
                    }
                    _ => {}
                }
            }

            let db_item = db_item.unwrap();
            trace!(db_item = ?db_item, "db item before convert");
            let db_item: Item = if let Ok(k) = db_item.try_into() {
                k
            } else {
                continue;
            };
            trace!(db_item = ?db_item, "db item after convert");

            debug!(
                "calculate similarity with {} {}",
                db_item.name, db_item.base_type
            );
            trace!(req_item = ?item, db_item = ?db_item, "calculate similarity");
            let calc = db_item.calculate_similarity_score_with_pob(item);
            if calc > highscore {
                trace!("get higher score: {:?}", calc);
                highscore = calc;
                result_item = db_item;
            }
        }

        debug!(
            "found item {} {} with score {:?}",
            result_item.name, result_item.base_type, highscore
        );
        trace!("found item {:?} with score {:?}", result_item, highscore);
        (result_item, highscore)
    }
}
