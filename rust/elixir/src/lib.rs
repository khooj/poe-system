use pob::{build_import_pob::import_build_from_pob, Pob};
use rustler::{
    types::OwnedBinary, Atom, Binary, Env, Error, NewBinary, NifStruct, ResourceArc, Term,
};

mod atoms {
    rustler::atoms! {
        ok,
        error,
    }
}

#[derive(Debug, thiserror::Error)]
pub enum RustError {
    #[error("serde_json error: {0}")]
    SerdeJson(#[from] serde_json::Error),
    #[error("import build from pob: {0}")]
    ImportPob(#[from] pob::build_import_pob::ImportPobError),
    #[error("convert u8 to str error: {0}")]
    Utf8(#[from] std::str::Utf8Error),
}

impl From<RustError> for Error {
    fn from(value: RustError) -> Self {
        Error::Term(Box::new(value.to_string()))
    }
}

#[rustler::nif(schedule = "DirtyCpu")]
fn extract_build_config<'a>(
    env: Env<'a>,
    pobxml: Binary<'a>,
    itemset: Binary<'a>,
) -> Result<Term<'a>, Error> {
    let pobxml = pobxml.trim_ascii();
    let pob = Pob::new(std::str::from_utf8(pobxml).map_err(RustError::from)?);
    let itemset = std::str::from_utf8(itemset.trim_ascii()).map_err(RustError::from)?;
    let info = import_build_from_pob(&pob, itemset).map_err(RustError::from)?;
    let data = serde_json::to_string(&info).map_err(RustError::from)?;
    let mut b = NewBinary::new(env, data.len());
    b.as_mut_slice().copy_from_slice(data.as_bytes());
    Ok(b.into())
}

rustler::init!("Elixir.RustPoe.Native");
