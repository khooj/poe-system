use actix::prelude::*;
use std::{collections::HashMap, convert::TryInto};
use uuid::Uuid;

use crate::{
    domain::{item::Item, PastebinBuild},
    implementations::{
        models::{PobBuild, BuildMatch, PobFile},
        pob::pob::Pob,
        BuildsRepository, ItemsRepository,
    },
};
use tracing::{event, instrument, Level};

pub struct BuildCalculatorActor {
    pub repo: BuildsRepository,
    pub item_repo: ItemsRepository,
}

impl Actor for BuildCalculatorActor {
    type Context = Context<Self>;
}

#[derive(Message, Debug)]
#[rtype(result = "Result<String, anyhow::Error>")]
pub struct StartBuildCalculatingMsg {
    pub pob_url: String,
    pub itemset: Option<String>,
}

impl Handler<StartBuildCalculatingMsg> for BuildCalculatorActor {
    type Result = Result<String, anyhow::Error>;

    #[instrument(err, skip(self, ctx))]
    fn handle(&mut self, msg: StartBuildCalculatingMsg, ctx: &mut Self::Context) -> Self::Result {
        // TODO: transactional work
        let pastebin = PastebinBuild::new(&msg.pob_url)?;

        let pob_id = match self.repo.get_pob_file_id_by_url(&pastebin) {
            Ok(k) => k,
            _ => {
                let pob_build = ureq::get(&pastebin.pastebin_raw_url())
                    .call()?
                    .into_string()?;

                let new_pob = PobFile::new(format!("{}", Uuid::new_v4()), &pastebin, &pob_build);
                self.repo.save_new_pob_file(new_pob)?
            }
        };
        event!(Level::DEBUG, "saved pob file with id {}", pob_id);

        let mut builds = self.repo.get_build_by_url(&pastebin)?;
        let id = if builds.len() == 0 {
            let new_build = PobBuild {
                // TODO: inconsistent id assigning. Decide if it should be assigned in repo or domain
                id: format!("{}", Uuid::new_v4()),
                pob_file_id: pob_id,
                itemset: msg.itemset.unwrap_or(String::new()),
            };

            // TODO: ignoring itemset for now
            let id = self.repo.save_new_build(new_build)?;
            let build = self.repo.get_build(&id)?;
            builds.push(build);
            id
        } else {
            builds[0].id.clone()
        };

        for build in builds {
            ctx.notify(CalculateBuildAlgo { build });
        }

        Ok(id)
    }
}

#[derive(Message, Debug)]
#[rtype(result = "Result<(), anyhow::Error>")]
struct CalculateBuildAlgo {
    build: PobBuild,
}

impl Handler<CalculateBuildAlgo> for BuildCalculatorActor {
    type Result = Result<(), anyhow::Error>;

    #[instrument(err, skip(self, _ctx, msg), fields(build = ?msg.build))]
    fn handle(&mut self, msg: CalculateBuildAlgo, _ctx: &mut Self::Context) -> Self::Result {
        // TODO: need to fix unwrap()s

        let mut build = msg.build;
        let pob = self.repo.get_pob_file(&build.pob_file_id)?;
        let pob = Pob::from_pastebin_data(pob.encoded_pob)?;
        let pob_doc = pob.as_document()?;
        let itemsets = pob_doc.get_item_sets();

        event!(Level::INFO, "got itemsets for build {}", build.id);

        let itemset = if build.itemset.is_empty() {
            let itemset = itemsets
                .first()
                .ok_or(anyhow::anyhow!("empty itemset in build {}", build.id))?
                .clone();
            build.itemset = itemset.title().to_owned();
            itemset
        } else {
            itemsets
                .into_iter()
                .find(|e| e.title() == build.itemset)
                .ok_or(anyhow::anyhow!(
                    "cant find itemset {} in build {}",
                    build.itemset,
                    build.id
                ))?
        };

        event!(Level::INFO, "ready to calculate items");

        let items = itemset.items();
        let mut item_match = HashMap::new();

        for (idx, item) in items.iter().enumerate() {
            let domain_item: Item = item.clone().try_into()?;

            let db_items = self
                .item_repo
                .get_items_by_basetype(&domain_item.base_type)?;

            for db_item in &db_items {
                let mods_score = domain_item.calculate_similarity_score(&db_item);

                let e = item_match.entry(idx).or_insert((0i64, String::new()));
                e.1 = db_item.id.clone();
                e.0 = *mods_score;
            }
        }

        for (idx, (score, id)) in &item_match {
            let mtch = BuildMatch {
                id: build.id.clone(),
                idx: *idx as i32,
                score: *score as i32,
                item_id: id.clone(),
            };

            self.repo.new_build_match(&mtch)?;
        }

        event!(Level::INFO, "build {} done", build.id);

        Ok(())
    }
}
