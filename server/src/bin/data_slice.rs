use poe_system::infrastructure::public_stash_retriever::Client;
use poe_system::interfaces::public_stash_retriever::Error;
use std::io::BufRead;
use std::io::{BufReader, BufWriter, Error as IoError, ErrorKind, Write};
use std::path::Path;
use std::{env::args, fs::OpenOptions};
use tracing::{debug, info};
use tracing_subscriber::fmt;

fn main() -> Result<(), IoError> {
    fmt::init();

    let args: Vec<String> = args().collect();
    if args.len() < 2 {
        return Err(IoError::new(ErrorKind::InvalidInput, "wrong argument size"));
    }

    let mut client = Client::new("OAuth data_slice/0.1.0 (contact: bladoff@gmail.com)".into());
    let dir = Path::new(&args[1]);
    let mut id = try_find_saved_id("index", &dir);

    loop {
        let resp = match client.get_latest_stash(id.as_deref()) {
            Ok(r) => r,
            Err(e) => match e {
                Error::NextCycle => continue,
                _ => panic!("{}", e),
            },
        };

        info!("next stash id: {}", resp.next_change_id);

        if resp.stashes.len() == 0 {
            break;
        }

        info!("saving stash_id: {:?}", id);

        let filepath = dir
            .join(&id.unwrap_or("index".into()))
            .with_extension("json");
        let f = OpenOptions::new()
            .write(true)
            .create(true)
            .open(&filepath)?;
        let mut buf = BufWriter::new(f);
        serde_json::to_writer(&mut buf, &resp)?;
        buf.flush()?;

        id = Some(resp.next_change_id);
    }

    Ok(())
}

fn try_find_saved_id(latest_id: &str, dir: &Path) -> Option<String> {
    let mut found_id = Some(latest_id.to_owned());

    loop {
        let fp = dir.join(found_id.as_ref().unwrap()).with_extension("json");
        if !fp.exists() {
            return found_id;
        }

        let f = OpenOptions::new().read(true).open(&fp).expect("file");
        let mut buf = BufReader::new(f);

        let mut data = vec![];
        let _ = buf.read_until(b',', &mut data);
        let data = String::from_utf8(data).expect("cant convert to valid string");
        let data = data
            .strip_prefix("{\"next_change_id\":\"")
            .expect("strip1")
            .to_owned();
        let data2 = data.strip_suffix("\",").expect("strip2");
        found_id = Some(data2.to_owned());
        debug!("iterate over id: {:?}", found_id);
    }
}
