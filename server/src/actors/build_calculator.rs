use actix::prelude::*;
use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};
use itertools::Itertools;
use log::error;
use std::sync::{Arc, Mutex};
use std::{collections::HashMap, convert::TryInto};
use uuid::Uuid;

use crate::{
    domain::item::Item,
    implementations::{
        builds_repository::DieselBuildsRepository,
        item_repository::DieselItemRepository,
        models::{NewBuildMatch, PobBuild},
        pob::pob::{ItemSet, Pob},
    },
};
use anyhow::anyhow;

pub struct BuildCalculatorActor {
    pub repo: Arc<Mutex<DieselBuildsRepository>>,
    pub item_repo: Arc<Mutex<DieselItemRepository>>,
}

impl Actor for BuildCalculatorActor {
    type Context = Context<Self>;
}

#[derive(Message)]
#[rtype(result = "Result<String, anyhow::Error>")]
pub struct StartBuildCalculatingMsg {
    pub pob_url: String,
    pub itemset: Option<String>,
}

impl Handler<StartBuildCalculatingMsg> for BuildCalculatorActor {
    type Result = Result<String, anyhow::Error>;

    fn handle(&mut self, msg: StartBuildCalculatingMsg, ctx: &mut Self::Context) -> Self::Result {
        match self
            .repo
            .lock()
            .unwrap()
            .save_new_build(&msg.pob_url, msg.itemset.as_deref().unwrap_or(&""))
        {
            Ok(k) => {
                let build = self.repo.lock().unwrap().get_build(&k)?;

                let token = build.pob_url.split('/').collect::<Vec<_>>();
                let token = token.last().unwrap_or(&"").to_string();

                if token.is_empty() {
                    let e = anyhow::anyhow!("wrong pastebin url");
                    error!("{}", e);
                    return Err(e);
                }
                ctx.notify(CalculateBuild { build, token });
                Ok(k)
            }
            Err(e) => {
                error!("cant save new build for calculating: {}", e);
                Err(anyhow::anyhow!(
                    "cant save new build for calculating: {}",
                    e
                ))
            }
        }
    }
}

#[derive(Message)]
#[rtype(result = "Result<(), anyhow::Error>")]
struct CalculateBuild {
    build: PobBuild,
    token: String,
}

impl Handler<CalculateBuild> for BuildCalculatorActor {
    type Result = ResponseActFuture<Self, Result<(), anyhow::Error>>;

    fn handle(&mut self, msg: CalculateBuild, ctx: &mut Self::Context) -> Self::Result {
        Box::pin(
            async move {
                let resp = reqwest::get(format!("https://pastebin.com/raw/{}", msg.token)).await;
                let k = match resp {
                    Ok(k) => match k.text().await {
                        Ok(s) => s,
                        Err(e) => {
                            let e = anyhow!("cant get pastebin: {}", e);
                            error!("{}", e);
                            return Err(e);
                        }
                    },
                    Err(e) => {
                        let e = anyhow!("cant get pastebin: {}", e);
                        error!("{}", e);
                        return Err(e);
                    }
                };
                Ok((k, msg.build))
            }
            .into_actor(self)
            .map(|result, act, ctx| {
                if result.is_err() {
                    let e = anyhow!("cant request pastebin");
                    error!("{}", e);
                    return Err(e);
                }
                let (pob_data, build) = result.unwrap();
                ctx.notify(CalculateBuildAlgo { pob_data, build });
                Ok(())
            }),
        )
    }
}

#[derive(Message)]
#[rtype(result = "Result<(), anyhow::Error>")]
struct CalculateBuildAlgo {
    build: PobBuild,
    pob_data: String,
}

impl Handler<CalculateBuildAlgo> for BuildCalculatorActor {
    type Result = Result<(), anyhow::Error>;

    fn handle(&mut self, msg: CalculateBuildAlgo, ctx: &mut Self::Context) -> Self::Result {
        let mut build = msg.build;
        let pob = Pob::new(msg.pob_data);
        let pob_doc = pob.as_document()?;
        let itemsets = pob_doc.get_item_sets();

        let itemset: ItemSet;

        if build.itemset.is_empty() {
            itemset = match itemsets
                .first()
                .ok_or(anyhow::anyhow!("empty itemset in build {}", build.id))
            {
                Ok(k) => k.clone(),
                Err(e) => {
                    error!("{}", e);
                    return Err(e);
                }
            };
            build.itemset = itemset.title().to_owned();
        } else {
            itemset = match itemsets
                .into_iter()
                .find(|e| e.title() == build.itemset)
                .ok_or(anyhow::anyhow!(
                    "cant find itemset {} in build {}",
                    build.itemset,
                    build.id
                )) {
                Ok(k) => k,
                Err(e) => {
                    error!("{}", e);
                    return Err(e);
                }
            };
        }

        let items = itemset.items();
        let matcher = SkimMatcherV2::default();
        let mut item_match = HashMap::new();

        for (idx, item) in items.iter().enumerate() {
            let domain_item: Item = item.clone().try_into()?;

            let db_items = self
                .item_repo
                .lock()
                .unwrap()
                .get_items_by_basetype(&domain_item.base_type)?;

            for db_item in &db_items {
                let mods_scores = domain_item
                    .mods
                    .iter()
                    .cartesian_product(db_item.mods.iter())
                    .group_by(|(el, _)| el.text.clone())
                    .into_iter()
                    .map(|(k, grp)| {
                        (
                            k,
                            grp.map(|(o, d)| matcher.fuzzy_match(&d.text, &o.text).unwrap_or(0i64))
                                .max()
                                .unwrap_or(0i64),
                        )
                    })
                    .collect::<HashMap<String, i64>>();

                let e = item_match.entry(idx).or_insert((0i64, String::new()));
                e.1 = db_item.id.clone();
                e.0 = mods_scores.values().sum();
            }
        }

        for (idx, (score, id)) in &item_match {
            let mtch = NewBuildMatch {
                id: Uuid::new_v4().to_hyphenated().to_string(),
                idx: *idx as i32,
                score: *score as i32,
                item_id: &id,
            };

            self.repo.lock().unwrap().new_build_match(&mtch)?;
        }

        Ok(())
    }
}
