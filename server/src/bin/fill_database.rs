use anyhow::anyhow;
use poe_system::utils::file_dump::FileDumpDirectory;
use std::env::{args, var};
use std::io::{Error as IOError, ErrorKind};
use std::path::PathBuf;
use tracing::info;
use tracing_subscriber::fmt;

fn main() -> Result<(), IOError> {
    dotenv::dotenv().ok();
    fmt::init();

    let json_data_dir = var("REPOE_DATA_DIR").expect("can't get repoe data dir env");
    let json_data_dir = PathBuf::from(json_data_dir);

    let args: Vec<String> = args().collect();
    if args.len() < 2 {
        return Err(IOError::new(ErrorKind::InvalidInput, "wrong argument size"));
    }

    let dump_dir = FileDumpDirectory::new(&args[1]);
    Ok(())
}
