use poe_system::implementations::public_stash_retriever::Client;
use poe_system::ports::public_stash_retriever::{Error, Retriever};
use std::io::{BufWriter, Write};
use std::{env::args, fs::OpenOptions};
use log::info;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = args().collect();
    if args.len() < 2 {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "wrong argument size",
        ));
    }

    let mut stashes_info = Vec::with_capacity(110_000);
    let mut client: Box<dyn Retriever> = Box::new(Client::new(
        "OAuth latest-stashes/0.1.0 (contact: bladoff@gmail.com)",
    ));
    let mut id: Option<String> = None;
    let f = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(&args[1])?;
    let mut buf = BufWriter::new(f);

    loop {
        let mut resp = match client.get_latest_stash(id.as_deref()).await {
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

        stashes_info.append(&mut resp.stashes);
        id = Some(resp.next_change_id);
        info!("now stashes info len: {}", stashes_info.len());

        if stashes_info.len() >= 100_000 {
            info!("writing {} entries", stashes_info.len());
            serde_json::to_writer(&mut buf, &stashes_info)?;
            stashes_info.clear();
        }
    }

    info!("flushing");
    serde_json::to_writer(&mut buf, &stashes_info)?;
    buf.flush()
}