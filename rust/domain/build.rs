use std::fs::{copy, create_dir, metadata};
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // if metadata("dist").is_err() {
    //     create_dir("dist")?;
    // }
    // let files = [
    //     "stat_translations.min.json",
    //     "base_items.min.json",
    //     "stats.min.json",
    // ];
    // let repoe = PathBuf::from("../RePoE/RePoE/data");
    // let dist = PathBuf::from("dist");
    // for i in files {
    //     copy(repoe.join(i), dist.join(i))?;
    // }
    Ok(())
}
