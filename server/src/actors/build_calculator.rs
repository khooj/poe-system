use actix::prelude::*;
use log::error;
use std::sync::{Arc, Mutex};

use crate::implementations::builds_repository::DieselBuildsRepository;

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

    fn handle(&mut self, msg: StartBuildCalculatingMsg, _ctx: &mut Self::Context) -> Self::Result {
        match self
            .repo
            .lock()
            .unwrap()
            .save_new_build(&msg.pob_url, msg.itemset.as_deref().unwrap_or(&""))
        {
            Ok(k) => Ok(k),
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
