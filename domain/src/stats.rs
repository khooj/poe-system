use macros::static_array_from_file;

pub static STATS_SIMPLE: &[&'static str] = static_array_from_file!("domain/src/stats.txt");
