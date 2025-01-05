use compress_tools::tokio_support::{list_archive_files, uncompress_archive_file};
use core::str;
use futures::future::ok;
use std::env::args;
use std::os::unix::fs::MetadataExt;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};

//use std::fs::File;
//use std::io::{BufReader, BufWriter, Cursor, Read, Seek, Write};
use tokio::fs::File;

const NEXT_CHANGE_LINE: &str = "\"next_change_id\"";

enum Msg {
    Exit,
    Data(String),
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = args().into_iter().collect::<Vec<String>>();
    if args.len() < 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        return Ok(());
    }

    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();

    let handle = tokio::spawn(async move {
        loop {
            match rx.recv().await {
                Some(msg) => match msg {
                    Msg::Exit => return,
                    Msg::Data(s) => io::stdout()
                        .write_all(s.as_bytes())
                        .await
                        .expect("cannot write to stdout"),
                },
                None => {}
            }
        }
    });

    let mut src = File::open(&args[1]).await?;
    let mut archive = Vec::with_capacity(src.metadata().await?.size() as usize);
    src.read_to_end(&mut archive).await?;
    //let mut arc_buf = Cursor::new(&archive);
    let files = list_archive_files(&mut &archive[..]).await?;
    //println!("{:?}", files);
    let mut buf = Vec::with_capacity(10 * 1024 * 1024);
    let mut q = Some("index.json".to_string());
    while let Some(f) = q.take() {
        buf.clear();
        let bytes = uncompress_archive_file(&mut &archive[..], &mut buf, &f).await?;
        let s = unsafe { str::from_utf8_unchecked(&buf[..bytes]) };
        if let Some(mut idx) = s.find(NEXT_CHANGE_LINE) {
            idx += NEXT_CHANGE_LINE.len();
            let first_semi = s[idx..].find("\"").unwrap() + idx;
            let second_semi = s[first_semi + 1..].find("\"").unwrap() + first_semi + 1;
            let change_id = &s[first_semi + 1..second_semi];
            let filename = format!("{}.json", change_id);
            if files.contains(&filename) {
                q = Some(filename);
            }
        }
        tx.send(Msg::Data(s.to_string()))?;
    }

    tx.send(Msg::Exit)?;
    handle.await?;
    Ok(())
}
