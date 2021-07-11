use actix::prelude::*;
use itertools::Itertools;
use std::{collections::HashMap, convert::TryInto};
use uuid::Uuid;

use crate::{
    domain::item::Item,
    implementations::{
        models::{NewBuildMatch, PobBuild},
        pob::pob::Pob,
        BuildsRepository, ItemsRepository,
    },
};
use strsim::levenshtein;
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
        let repo = self.repo.clone();
        let mut builds = self.repo.get_build_by_url(&msg.pob_url)?;
        let id = if builds.len() == 0 {
            // TODO: ignoring itemset for now
            let id = repo.save_new_build(&msg.pob_url, &msg.itemset.unwrap_or(String::new()))?;
            let build = repo.get_build(&id)?;
            builds.push(build);
            id
        } else {
            builds[0].id.clone()
        };

        for build in builds {
            let token = build.pob_url.split('/').collect::<Vec<_>>();
            let token = token.last().unwrap_or(&"").to_string();
            event!(
                Level::INFO,
                "token extracted {} for {}: {}",
                build.pob_url,
                build.id,
                token
            );

            if token.is_empty() {
                event!(Level::ERROR, "wrong pastebin url {}", build.pob_url);
                continue;
            }

            ctx.notify(CalculateBuildAlgo { build, token });
        }

        Ok(id)
    }
}

#[derive(Message, Debug)]
#[rtype(result = "Result<(), anyhow::Error>")]
struct CalculateBuildAlgo {
    build: PobBuild,
    token: String,
}

impl Handler<CalculateBuildAlgo> for BuildCalculatorActor {
    type Result = Result<(), anyhow::Error>;

    #[instrument(err, skip(self, _ctx, msg), fields(build = ?msg.build))]
    fn handle(&mut self, msg: CalculateBuildAlgo, _ctx: &mut Self::Context) -> Self::Result {
        // TODO: need to fix unwrap()s

        let pob_build = ureq::get(&format!("https://pastebin.com/raw/{}", msg.token))
            .call()?
            .into_string()?;

        let mut build = msg.build;
        let pob = Pob::from_pastebin_data(pob_build)?;
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
            let mtch = NewBuildMatch {
                id: Uuid::new_v4().to_hyphenated().to_string(),
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
