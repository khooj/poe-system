use std::env;

use pob::{Pob, PobError};
use thiserror::Error;
use tradeapi::{
    query::{Builder, BuilderError, StatQuery, StatQueryType, TypeFilters},
    Client, ClientError,
};

#[derive(Error, Debug)]
pub enum CalculateBuildError {
    #[error("dummy error")]
    Dummy,
    #[error("pob parsing error")]
    PobError(#[from] PobError),
    #[error("api client error")]
    ClientError(#[from] ClientError),
}

pub async fn calculate_build_cost(pob_base64: &str) -> Result<(), CalculateBuildError> {
    let pob = Pob::from_pastebin_data(pob_base64.to_string())?;
    let pob_doc = pob.as_document()?;
    let first_itemset = pob_doc.get_first_itemset()?;

    let poesessid = env::var("POESESSID").expect("poesessid not defined");
    let mut api_client = Client::new(
        "Mozilla/5.0 (X11; Linux x86_64; rv:109.0) Gecko/20100101 Firefox/110.0".to_string(),
        &poesessid,
        "Crucible",
    );

    let item = first_itemset.get_nth_item(0).unwrap();

    let mut builder = Builder::new();
    builder.set_type(&item.base_type);
    // builder.set_type_filters(TypeFilters::default().set_category(&item.base_type));
    println!("search query: {}", serde_json::to_string(&builder).unwrap());

    let search_id = api_client.get_search_id(builder).await?;
    println!("results len: {}", search_id.result.len());
    let results = api_client
        .fetch_results(
            search_id.result.iter().take(5).cloned().collect(),
            &search_id.id,
        )
        .await?;
    println!("trade api results: {:?}", results);
    Ok(())
}
