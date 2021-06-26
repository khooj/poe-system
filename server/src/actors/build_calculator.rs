use actix::prelude::*;
use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};
use itertools::Itertools;
use std::{collections::HashMap, convert::TryInto};
use uuid::Uuid;

use crate::{
    actors::{builds_repository::BuildsRepositoryActor, item_repository::ItemsRepositoryActor},
    domain::item::Item,
    implementations::{
        models::{NewBuildMatch, PobBuild},
        pob::pob::{ItemSet, Pob},
    },
};
use anyhow::anyhow;
use tracing::{error, event, span, Instrument, Level};

use super::builds_repository::{GetBuild, GetBuildByUrl, NewBuildMatch as MsgNewBuildMatch, SaveNewBuild};
use super::item_repository::GetItemsByBasetype;

pub struct BuildCalculatorActor {
    pub repo: Addr<BuildsRepositoryActor>,
    pub item_repo: Addr<ItemsRepositoryActor>,
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
    type Result = ResponseActFuture<Self, Result<String, anyhow::Error>>;

    fn handle(&mut self, msg: StartBuildCalculatingMsg, _ctx: &mut Self::Context) -> Self::Result {
        let repo = self.repo.clone();
        let span = span!(
            Level::INFO,
            "BuildCalculatorActor(StartBuildCalculatingMsg)"
        );
        Box::pin(
            async move {
                let builds = match repo.send(GetBuildByUrl{ url: msg.pob_url.clone() }).await {
                    Ok(k) => k,
                    Err(e) => {
                        event!(Level::INFO, "build not found, creating new {}", url=msg.pob_url);
                        vec![]
                    }
                };
                match repo.send(SaveNewBuild {
                        pob: msg.pob_url,
                        itemset: msg.itemset.unwrap_or(String::new()),
                    })
                    .await
                {
                    Ok(s) => match s {
                        Ok(k) => {
                            let build = match repo.send(GetBuild { id: k.clone() }).await {
                                Ok(k) => match k {
                                    Ok(s) => s,
                                    Err(e) => {
                                        let e = anyhow::anyhow!("actor err: {}", e);
                                        event!(Level::ERROR, "{}", e);
                                        return Err(e);
                                    }
                                },
                                Err(e) => {
                                    let e = anyhow::anyhow!("err repo: {}", e);
                                    event!(Level::ERROR, "{}", e);
                                    return Err(e);
                                }
                            };

                            let token = build.pob_url.split('/').collect::<Vec<_>>();
                            let token = token.last().unwrap_or(&"").to_string();
                            event!(Level::INFO, "token extracted from {} for {}: {}", build.pob_url, k, token);

                            if token.is_empty() {
                                let e = anyhow::anyhow!("wrong pastebin url");
                                event!(Level::ERROR, "{}", e);
                                return Err(e);
                            }
                            Ok((CalculateBuild { build, token }, k))
                        }
                        Err(e) => {
                            event!(Level::ERROR, "cant save new build for calculating: {}", e);
                            Err(anyhow::anyhow!(
                                "cant save new build for calculating: {}",
                                e
                            ))
                        }
                    },
                    Err(e) => {
                        event!(Level::ERROR, "cant send msg for repo: {}", e);
                        Err(anyhow::anyhow!(
                            "cant send msg for repo: {}",
                            e
                        ))
                    }
                }
            }
            .instrument(span)
            .into_actor(self)
            .map(|res, _, ctx| match res {
                Ok(k) => {
                    ctx.notify(k.0);
                    Ok(k.1)
                }
                Err(e) => Err(e),
            }),
        )
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

    fn handle(&mut self, msg: CalculateBuild, _ctx: &mut Self::Context) -> Self::Result {
        let span = span!(Level::INFO, "BuildCalculatorActor(CalculateBuild)");
        Box::pin(
            async move {
                let resp = reqwest::get(format!("https://pastebin.com/raw/{}", msg.token)).await;
                let k = match resp {
                    Ok(k) => match k.text().await {
                        Ok(s) => s,
                        Err(e) => {
                            let e = anyhow!("cant get pastebin: {}", e);
                            event!(Level::ERROR, "{}", e);
                            return Err(e);
                        }
                    },
                    Err(e) => {
                        let e = anyhow!("cant get pastebin: {}", e);
                        event!(Level::ERROR, "{}", e);
                        return Err(e);
                    }
                };
                Ok((k, msg.build))
            }
            .instrument(span)
            .into_actor(self)
            .map(|result, _act, ctx| {
                if result.is_err() {
                    let e = anyhow!("cant request pastebin");
                    error!("{}", e);
                    return Err(e);
                }
                let (pob_data, build) = result.unwrap();
                event!(Level::INFO, "successfully got build data for build {}", build.id);
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
    type Result = ResponseFuture<Result<(), anyhow::Error>>;

    fn handle(&mut self, msg: CalculateBuildAlgo, _ctx: &mut Self::Context) -> Self::Result {
        let item_repo = self.item_repo.clone();
        let repo = self.repo.clone();
        let span = span!(Level::INFO, "BuildCalculatorActor(CalculateBuildAlgo)");
        Box::pin(
            async move {
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

                    let db_items = item_repo
                        .send(GetItemsByBasetype {
                            base_type: domain_item.base_type.clone(),
                        })
                        .await??;

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
                                    grp.map(|(o, d)| {
                                        matcher.fuzzy_match(&d.text, &o.text).unwrap_or(0i64)
                                    })
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
                        item_id: id.clone(),
                    };

                    match repo.send(MsgNewBuildMatch { mtch }).await? {
                        Err(e) => {
                            event!(Level::ERROR, "error while inserting new build match: {}", e);
                        }
                        _ => {}
                    }
                }

                Ok(())
            }
            .instrument(span),
        )
    }
}
