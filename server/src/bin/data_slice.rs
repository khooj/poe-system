use poe_system::infrastructure::public_stash_retriever::Client;
use poe_system::interfaces::public_stash_retriever::Error;
use poe_system::utils::file_dump::FileDumpDirectory;
use std::env::args;
use std::io::{Error as IoError, ErrorKind};
use tracing::info;
use tracing_subscriber::fmt;

fn main() -> Result<(), IoError> {
    fmt::init();

    let args: Vec<String> = args().collect();
    if args.len() < 2 {
        return Err(IoError::new(ErrorKind::InvalidInput, "wrong argument size"));
    }

    let mut client = Client::new("OAuth data_slice/0.1.0 (contact: bladoff@gmail.com)".into());
    let dump_dir = FileDumpDirectory::new(&args[1]);
    let mut id = dump_dir
        .find_latest_id("index")
        .expect("can't get latest id");

    loop {
        let resp = match client.get_latest_stash(Some(&id)) {
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

        dump_dir
            .save_request(Some(&id), &resp)
            .expect("can't save request");
        id = resp.next_change_id;
    }

    Ok(())
}
