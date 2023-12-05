use clap::{Parser, Subcommand};
use public_stash::models::PublicStashData;
use serde::Deserialize;
use std::fs::OpenOptions;
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::PathBuf;
use std::sync::Mutex;
use thiserror::Error;

#[derive(Subcommand, Debug)]
enum Commands {
    Merge {
        #[arg(short)]
        dumpdir: String,
        #[arg(short)]
        ndjson: Option<String>,
        #[arg(short)]
        method: i32,
    },
    Load {
        filename: String,
        #[arg(short)]
        method: i32,
    },
    ShowFirstItems {
        filename: String,
        count: usize,
    },
}

#[derive(Parser)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

// 2:54m
fn try_ndjson(dumpdir: &str, dst: &str) -> Result<(), anyhow::Error> {
    let path = PathBuf::from(dumpdir);
    let mut filepath = path.join("index.json");
    let dst = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(dst)?;
    let mut wr = BufWriter::new(dst);

    loop {
        if std::fs::metadata(&filepath).is_err() {
            break;
        }

        let f = OpenOptions::new().read(true).open(&filepath)?;
        let mut buf = BufReader::new(f);
        let data: PublicStashData = serde_json::from_reader(&mut buf)?;
        println!("read {filepath}", filepath = filepath.display());
        filepath = path.join(format!("{}.json", data.next_change_id));
        serde_json::to_writer(&mut wr, &data)?;
        wr.write_all(b"\n")?;
    }

    Ok(())
}

// 1:58m
fn try_ndjson_2(dumpdir: &str, dst: &str) -> Result<(), anyhow::Error> {
    let path = PathBuf::from(dumpdir);
    let mut filepath = path.join("index.json");
    let dst = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(dst)?;
    let mut wr = BufWriter::new(dst);
    let mut buf = vec![0; 10_000_000];

    loop {
        if std::fs::metadata(&filepath).is_err() {
            break;
        }

        let mut f = OpenOptions::new().read(true).open(&filepath)?;
        let s = f.read(&mut buf)?;
        let data: PublicStashData = serde_json::from_reader(&buf[0..s])?;
        println!("read {filepath}", filepath = filepath.display());
        filepath = path.join(format!("{}.json", data.next_change_id));
        serde_json::to_writer(&mut wr, &data)?;
        wr.write_all(b"\n")?;
    }

    Ok(())
}

fn load_ndjson_zstd(filepath: &str) -> Result<(), anyhow::Error> {
    let f = std::fs::File::open(filepath)?;
    let z = zstd::Decoder::new(f)?;
    let z = std::io::BufReader::new(z);
    use std::io::BufRead;
    for l in z.lines() {
        let data: PublicStashData = serde_json::from_str(&l.unwrap())?;
        println!("read change: {}", data.next_change_id);
    }
    Ok(())
}

fn load_ndjson_zstd_2(filepath: &str) -> Result<(), anyhow::Error> {
    #[derive(Deserialize)]
    struct Stash {
        next_change_id: String,
        // #[serde(borrow)]
        // stashes: &'a serde_json::value::RawValue,
    }
    let f = std::fs::File::open(filepath)?;
    let z = zstd::Decoder::new(f)?;
    let z = std::io::BufReader::new(z);
    use std::io::BufRead;
    for l in z.lines() {
        let l = l.unwrap();
        let data: Stash = serde_json::from_str(&l)?;
        println!("read change: {}", data.next_change_id);
    }
    Ok(())
}

#[derive(Error, Debug)]
enum CbError {
    #[error("stop")]
    StopError,
    #[error("everything")]
    EverythingError(#[from] anyhow::Error),
}

fn load_ndjson_cb<F>(filepath: &str, cb: F) -> Result<(), anyhow::Error>
where
    F: Fn(&PublicStashData) -> Result<(), CbError>,
{
    let f = std::fs::File::open(filepath)?;
    let z = zstd::Decoder::new(f)?;
    let z = std::io::BufReader::new(z);
    use std::io::BufRead;
    for l in z.lines() {
        let l = l.unwrap();
        let data: PublicStashData = serde_json::from_str(&l)?;
        match cb(&data) {
            Err(CbError::StopError) => return Ok(()),
            e => e?,
        }
    }
    Ok(())
}

fn show_first_items(filepath: &str, count: usize) -> Result<(), anyhow::Error> {
    let cur_count = Mutex::new(0usize);
    load_ndjson_cb(filepath, |stash| {
        let mut c = cur_count.lock().unwrap();
        for st in &stash.stashes {
            for it in &st.items {
                if *c >= count {
                    return Err(CbError::StopError);
                }
                println!(
                    "{}",
                    serde_json::to_string_pretty(&it)
                        .map_err(|e| CbError::EverythingError(e.into()))?
                );
                *c += 1;
            }
        }
        Ok(())
    })
}

fn main() -> Result<(), anyhow::Error> {
    let cli = Args::parse();
    match cli.command {
        Commands::Merge {
            dumpdir,
            ndjson,
            method,
        } => {
            match method {
                1 => try_ndjson(&dumpdir, ndjson.as_ref().unwrap())?,
                2 => try_ndjson_2(&dumpdir, ndjson.as_ref().unwrap())?,
                _ => panic!("select method 1,2"),
            };
        }
        Commands::Load { filename, method } => {
            match method {
                1 => load_ndjson_zstd(&filename)?,
                2 => load_ndjson_zstd_2(&filename)?,
                _ => panic!("select method 1,2"),
            };
        }
        Commands::ShowFirstItems { filename, count } => {
            show_first_items(&filename, count)?;
        }
    };
    Ok(())
}
