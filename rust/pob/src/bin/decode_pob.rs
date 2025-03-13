use std::{
    fs::{read_to_string, OpenOptions},
    io::Write,
};

use clap::Parser;
use pob::Pob;

#[derive(Parser)]
struct Cli {
    input: String,
    output: String,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let p = Pob::from_pastebin_data(read_to_string(cli.input)?.trim().to_string())?;
    let mut f = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(cli.output)?;
    f.write_all(p.get_original().as_bytes())?;
    f.flush()?;
    Ok(())
}
