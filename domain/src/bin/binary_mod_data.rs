use std::{
    fs::{File, OpenOptions},
    io::{BufWriter, Read, Write},
};

use clap::Parser;
use domain::prepare_data;

#[derive(Parser)]
struct Cli {
    input: String,
    output: String,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let mut f = File::open(cli.input)?;
    let mut buf = vec![];
    f.read_to_end(&mut buf)?;
    let data = prepare_data(&buf[..]);
    let output = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(cli.output)?;
    let mut buf = BufWriter::new(output);
    bincode::serialize_into(&mut buf, &data)?;
    buf.flush()?;
    Ok(())
}
