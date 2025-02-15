use utils::stream_stashes::open_stashes;
use std::env::args;
use std::io::{self, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = args().into_iter().collect::<Vec<String>>();
    if args.len() < 2 {
        eprintln!("Usage: {} <archive or dir>", args[0]);
        return Ok(());
    }

    let stashes = open_stashes(&args[1]);
    let mut count = 0;
    for (_, content) in stashes {
        count += 1;
        io::stdout()
            .write_all(content.as_bytes())
            .expect("cannot write to stdout");
        io::stdout()
            .write_all(b"\n")
            .expect("cannot write newline to stdout");
    }
    io::stdout().flush()?;
    eprintln!("processed: {}", count);

    Ok(())
}
