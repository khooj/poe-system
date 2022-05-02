use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::{Path, PathBuf};
use tracing::debug;

pub struct FileDumpDirectory {
    dir: PathBuf,
}

impl FileDumpDirectory {
    pub fn new(p: &str) -> Self {
        let p = PathBuf::from(p);
        FileDumpDirectory { dir: p }
    }

    pub fn find_latest_id(&self, known_id: &str) -> Result<String> {
        let mut found_id = known_id.to_owned();

        loop {
            let fp = self.dir.join(&found_id).with_extension("json");
            if !fp.exists() {
                return Ok(found_id);
            }

            let f = OpenOptions::new().read(true).open(&fp)?;
            let mut buf = BufReader::new(f);

            let mut data = vec![];
            let _ = buf.read_until(b',', &mut data);
            let data = String::from_utf8(data)?;
            let data = data
                .strip_prefix("{\"next_change_id\":\"")
                .ok_or(anyhow!("can't strip correctly"))?
                .to_owned();
            let data2 = data
                .strip_suffix("\",")
                .ok_or(anyhow!("can't strip suffix correctly"))?;
            found_id = data2.to_owned();
            debug!("iterate over id: {:?}", found_id);
        }
    }

    pub fn find_next_request<T>(&self, known_id: &str) -> Result<T> 
    where
        T: for<'de> Deserialize<'de>
    {
        let fp = self.dir.join(known_id).with_extension("json");
        if !fp.exists() {
            return Err(anyhow!("can't get next saved request"));
        }

        let f = OpenOptions::new().read(true).open(&fp)?;
        let mut buf = BufReader::new(f);

        let mut data = vec![];
        let _ = buf.read_until(b',', &mut data);
        let data = String::from_utf8(data)?;
        let data = data
            .strip_prefix("{\"next_change_id\":\"")
            .ok_or(anyhow!("can't strip correctly"))?
            .to_owned();
        let data2 = data
            .strip_suffix("\",")
            .ok_or(anyhow!("can't strip suffix correctly"))?;
        let found_id = data2.to_owned();

        let fp = self.dir.join(&found_id).with_extension("json");
        let f = OpenOptions::new().read(true).open(&fp)?;
        let buf = BufReader::new(f);
        Ok(serde_json::from_reader(buf)?)
    }

    pub fn save_request<T: Serialize>(&self, id: Option<&String>, resp: &T) -> Result<()> {
        let filepath = self
            .dir
            .join(id.unwrap_or(&"index".into()))
            .with_extension("json");
        let f = OpenOptions::new()
            .write(true)
            .create(true)
            .open(&filepath)?;
        let mut buf = BufWriter::new(f);
        serde_json::to_writer(&mut buf, &resp)?;
        buf.flush()?;
        Ok(())
    }
}
