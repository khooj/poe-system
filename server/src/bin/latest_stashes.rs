use poe_system::ports::outbound::public_stash_retriever::Error;
use poe_system::{
    implementations::public_stash_retriever::Client,
    ports::outbound::public_stash_retriever::PublicStashData,
};
use std::io::{BufWriter, Write};
use std::{env::args, fs::OpenOptions};
use tracing::info;
use tracing_subscriber::fmt;

fn main() -> Result<(), std::io::Error> {
    fmt::init();

    let args: Vec<String> = args().collect();
    if args.len() < 2 {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "wrong argument size",
        ));
    }

    let mut client = Client::new("OAuth latest-stashes/0.1.0 (contact: bladoff@gmail.com)".into());
    let mut id: Option<String> = None;
    let f = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(&args[1])?;
    let mut buf = BufWriter::new(f);

    loop {
        let resp = match client.get_latest_stash(id.as_deref()) {
            Ok(r) => r,
            Err(e) => match e {
                Error::NextCycle => continue,
                _ => panic!("{}", e),
            },
        };

        let resp2 = serde_json::from_str::<PublicStashData>(&resp)?;

        info!("next stash id: {}", resp2.next_change_id);

        if resp2.stashes.len() == 0 {
            break;
        }

        id = Some(resp2.next_change_id);

        info!("writing {} entries", resp2.stashes.len());
        serde_json::to_writer(&mut buf, &resp2.stashes)?;
    }
    buf.flush()
}
