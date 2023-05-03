use lazy_static::lazy_static;
use macros::static_array_from_file;
use std::{collections::HashMap, env::current_dir};

pub static STATS_IDS: &[&'static str] = static_array_from_file!("tradeapi/dist/stats_ids.txt");

lazy_static! {
    pub static ref STAT_TO_ID: HashMap<String, String> = {
        use std::env::var_os;
        use std::fs::OpenOptions;
        use std::io::BufReader;
        use std::path::PathBuf;

        let path = match var_os("DIST_DIR") {
            Some(v) => PathBuf::from(v.to_str().expect("can't convert DIST_DIR value")),
            None => current_dir().expect("can't get current dir").join("dist"),
        };
        let path = path.join("stats_to_ids.json");

        let f = OpenOptions::new()
            .read(true)
            .open(&path)
            .expect("can't open stats_to_ids.json file");

        let mut buf = BufReader::new(f);
        let r: HashMap<String, String> =
            serde_json::from_reader(&mut buf).expect("can't deserialize data from json");

        r
    };
}

#[cfg(test)]
mod tests {
    use super::STAT_TO_ID;

    #[test]
    fn check_stats_to_id() {
        assert_eq!(STAT_TO_ID.len(), 6843);
    }
}
