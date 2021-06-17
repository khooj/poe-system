use actix::prelude::*;
use itertools::Itertools;
use log::error;
use std::sync::{Arc, Mutex};

use crate::implementations::{builds_repository::DieselBuildsRepository, pob::pob::{ItemSet, Pob}};

pub struct BuildCalculatorActor {
    repo: Arc<Mutex<DieselBuildsRepository>>,
}

impl Actor for BuildCalculatorActor {
    type Context = Context<Self>;
}

#[derive(Message)]
#[rtype(result = "Result<String, anyhow::Error>")]
pub struct StartBuildCalculatingMsg {
    pob_url: String,
    itemset: Option<String>,
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
                ctx.notify(CalculateBuild { id: k.clone() });
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
    id: String,
}

impl Handler<CalculateBuild> for BuildCalculatorActor {
    type Result = Result<(), anyhow::Error>;

    fn handle(&mut self, msg: CalculateBuild, ctx: &mut Self::Context) -> Self::Result {
        let mut build = self.repo.lock().unwrap().get_build(&msg.id)?;

        let token = build.pob_url.split('/').collect::<Vec<_>>();
        let token = token.last().unwrap_or(&"");

        if token.is_empty() {
            let e = anyhow::anyhow!("wrong pastebin url");
            error!("{}", e);
            return Err(e);
        }

        let resp = reqwest::blocking::get(format!("https://pastebin.com/raw/{}", token))?;
        let data = resp.text()?;

        let pob = Pob::new(data);
        let pob_doc = pob.as_document()?;
        let itemsets = pob_doc.get_item_sets();

        let mut itemset: ItemSet;

        if build.itemset.is_empty() {
            itemset = match itemsets.first().ok_or(anyhow::anyhow!("empty itemset in build {}", build.id)) {
                Ok(k) => k.clone(),
                Err(e) => {
                    error!("{}", e);
                    return Err(e);
                }
            };
            build.itemset = itemset.title().to_owned();
        } else {
            itemset = match itemsets.into_iter()
            .find(|e| e.title() == build.itemset)
            .ok_or(anyhow::anyhow!("cant find itemset {} in build {}", build.itemset, build.id)) {
                Ok(k) => k,
                Err(e) => {
                    error!("{}", e);
                    return Err(e);
                }
            };
        }


        Ok(())
    }
}
