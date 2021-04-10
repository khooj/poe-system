use base64::{URL_SAFE, decode_config};
use flate2::read::ZlibDecoder;
use roxmltree::Document;
use std::convert::TryFrom;
use std::io::Read;

pub struct Pob {
    original: String,
}

impl TryFrom<&str> for Pob {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let tmp = decode_config(value, URL_SAFE)?;
        let mut decoder = ZlibDecoder::new(&tmp[..]);
        let mut s = String::new();
        decoder.read_to_string(&mut s)?;
        Ok(Pob::new(s))
    }
}

#[derive(Debug)]
pub struct PobDocument<'a> {
    doc: Document<'a>,
}

impl<'a> Pob {
    pub fn new(data: String) -> Pob {
        Pob { original: data }
    }

    pub fn as_document(&'a self) -> Result<PobDocument<'a>, anyhow::Error> {
        let doc = Document::parse(&self.original)?;
        Ok(PobDocument{ doc })
    }
}

#[cfg(test)]
mod tests {
    const TESTPOB: &'static str = include_str!("pob.txt");

    use std::convert::TryFrom;
    use super::{Pob, PobDocument};

    #[test]
    fn parse_pob() -> Result<(), anyhow::Error> {
        let pob = Pob::try_from(TESTPOB)?;
        let doc = pob.as_document()?;
        println!("{:?}", doc);
        Ok(())
    }
}