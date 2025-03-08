mod utils;

use pob::{
    build_import_pob::{import_build_from_pob, ImportPobError},
    Pob,
};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[derive(Debug, thiserror::Error)]
pub enum WasmError {
    #[error("pob error")]
    Pob(#[from] pob::PobError),
    #[error("import pob error")]
    ImportPob(#[from] ImportPobError),
    #[error("stub")]
    Stub,
    #[error("serde-wasm-bindgen error")]
    SerdeWasmBindgen(#[from] serde_wasm_bindgen::Error),
}

impl From<WasmError> for JsValue {
    fn from(value: WasmError) -> Self {
        JsValue::from_str(&value.to_string())
    }
}

#[wasm_bindgen]
pub fn get_pob_itemsets(s: &str) -> Result<Vec<String>, WasmError> {
    let pob = Pob::new(s);
    let doc = pob.as_document()?;
    Ok(doc
        .get_item_sets()
        .iter()
        .map(|is| is.title().to_string())
        .collect())
}

#[wasm_bindgen(unchecked_return_type = "BuildInfo")]
pub fn extract_build_config(pobtext: &str, itemset: &str) -> Result<JsValue, WasmError> {
    let pob = Pob::new(pobtext);
    let build_info = import_build_from_pob(&pob, itemset)?;
    Ok(serde_wasm_bindgen::to_value(&build_info)?)
}
