mod utils;

use pob::Pob;
use wasm_bindgen::prelude::*;
use web_sys::console;

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
pub fn get_pob_itemsets(s: &str) -> Result<Vec<String>, WasmError> {
    console::log_1(&s.into());
    let pob = Pob::new(s);
    let doc = pob.as_document()?;
    Ok(doc.get_itemsets_list()?)
}
