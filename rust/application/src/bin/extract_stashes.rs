use utils::stream_stashes::open_stashes;
use std::env::args;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = args().into_iter().collect::<Vec<String>>();
    if args.len() < 4 {
        eprintln!("Usage: {} <archive or dir> <extract_dir> <count>", args[0]);
        return Ok(());
    }

    let stashes = open_stashes(&args[1]);

    let dir = PathBuf::from(&args[2]);
    if !dir.exists() {
        std::fs::create_dir_all(&dir)?;
    }

    let max_count = (&args[3]).parse().expect("cannot parse count");
    let mut count = 0;
    for (filename, content) in stashes {
        count += 1;
        std::fs::write(dir.join(filename), content)?;
        if count >= max_count {
            break;
        }
    }
    eprintln!("processed: {}", count);

    Ok(())
}
