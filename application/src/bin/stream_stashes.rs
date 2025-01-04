use compress_tools::{list_archive_files, uncompress_archive_file};
use futures::future::ok;
use std::env::args;
use std::fs::File;
use std::io::{BufWriter, Read, Seek, Write};

const NEXT_CHANGE_LINE: &str = "\"next_change_id\"";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = args().into_iter().collect::<Vec<String>>();
    if args.len() < 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        return Ok(());
    }

    let mut src = File::open(&args[1])?;
    let files = list_archive_files(&src)?;
    //println!("{:?}", files);
    let mut buf = vec![];

    let mut q = Some("index.json".to_string());
    while let Some(f) = q.take() {
        buf.clear();
        src.seek(std::io::SeekFrom::Start(0))?;
        uncompress_archive_file(&mut src, &mut buf, &f)?;
        let s = String::from_utf8(buf.clone())?;
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
        println!("{}", s);
    }

    Ok(())
}
