use application::{ArchiveStashes, DirStashes};
use std::env::args;
use std::io::{self, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = args().into_iter().collect::<Vec<String>>();
    if args.len() < 2 {
        eprintln!("Usage: {} <archive or dir>", args[0]);
        return Ok(());
    }

    let stashes = if std::fs::metadata(&args[1]).unwrap().is_dir() {
        DirStashes::new(&args[1]).into_iter()
    } else {
        ArchiveStashes::new(&args[1]).into_iter()
    };

    let mut count = 0;
    for content in stashes {
        count += 1;
        io::stdout()
            .write_all(content.as_bytes())
            .expect("cannot write to stdout");
    }
    eprintln!("processed: {}", count);

    Ok(())
}
