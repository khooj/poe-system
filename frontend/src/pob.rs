use anyhow::anyhow;
use base64::{decode_config, URL_SAFE};
use flate2::read::ZlibDecoder;
use log::{error, info};
use roxmltree::{Document, Node};
use std::{collections::HashMap, io::Read};
use std::{
    convert::{TryFrom, TryInto},
    str::FromStr,
};
use crate::make_request::Error;

pub struct Pob {
    original: String,
}

impl<'a> Pob {
    pub fn new(data: String) -> Pob {
        Pob { original: data }
    }

    pub fn from_pastebin_data(data: String) -> Result<Pob, Error> {
        let tmp = decode_config(data, URL_SAFE).map_err(|e| Error::CustomError(e.to_string()))?;
        let mut decoder = ZlibDecoder::new(&tmp[..]);
        let mut s = String::new();
        decoder.read_to_string(&mut s).map_err(|e| Error::CustomError(e.to_string()))?;
        Ok(Pob { original: s })
    }

    pub fn as_document(&'a self) -> Result<PobDocument<'a>, anyhow::Error> {
        let doc = Document::parse(&self.original)?;
        Ok(PobDocument { doc })
    }

    pub fn raw_data(&self) -> &str {
        &self.original
    }
}

#[derive(Debug)]
pub struct PobDocument<'a> {
    doc: Document<'a>,
}
