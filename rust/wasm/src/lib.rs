mod utils;

use domain::build_calculation::stored_item::{Mod, StoredItem};
use pob::Pob;
use wasm_bindgen::prelude::*;
use web_sys::js_sys::Array;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[derive(Debug, thiserror::Error)]
pub enum WasmError {
    #[error("pob error: {0}")]
    Pob(#[from] pob::PobError),
    #[error("stub")]
    Stub,
    #[error("serde-wasm-bindgen error: {0}")]
    SerdeWasmBindgen(#[from] serde_wasm_bindgen::Error),
}

impl From<WasmError> for JsValue {
    fn from(value: WasmError) -> Self {
        JsValue::from_str(&value.to_string())
    }
}

#[wasm_bindgen]
pub fn get_pob_itemsets(s: String) -> Result<Vec<String>, WasmError> {
    let pob = Pob::from_pastebin_data(s)?;
    let doc = pob.as_document()?;
    Ok(doc.get_itemsets_list()?)
}

#[wasm_bindgen]
pub fn get_pob_skillsets(s: String) -> Result<Vec<String>, WasmError> {
    let pob = Pob::from_pastebin_data(s)?;
    let doc = pob.as_document()?;
    Ok(doc.get_skillsets_list()?)
}
