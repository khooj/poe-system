use application::pipe_stashes::parse_mods;
use clap::{Parser, Subcommand};
use domain::types::Mod;
use public_stash::models::PublicStashData;
use redis::AsyncCommands;
use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
};
use tokio::time::Instant;
use utils::stream_stashes::open_stashes;

#[derive(Parser)]
#[command(version, about)]
struct Cli {
    src: PathBuf,
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Redis,
}

struct FoundItem {
    id: String,
    basetype: String,
    affixes: HashMap<String, String>,
}

type SearchResult = Result<Vec<FoundItem>, Box<dyn std::error::Error>>;

async fn search_in_redis(mods: &Vec<Mod>) -> SearchResult {
    let cwd = std::env::current_dir()?;
    let client = redis::Client::open(format!(
        "redis+unix:{}",
        cwd.join("data")
            .join("r1")
            .join("redis.sock")
            .to_str()
            .unwrap()
    ))?;
    let mut conn = client.get_multiplexed_async_connection().await?;
    let mut ids: HashSet<String> = HashSet::new();
    let mut first_loaded = false;
    for m in mods {
        let k = format!("affix:{}", m.stat_id);
        let new_ids: HashSet<String> = conn.smembers(&k).await?;

        if !first_loaded {
            first_loaded = true;
            ids = new_ids.clone();
        } else {
            ids = ids.intersection(&new_ids).cloned().collect();
        }

        if ids.len() < 10 {
            break;
        }
    }

    let results = ids
        .into_iter()
        .map(|id| FoundItem {
            id,
            basetype: String::new(),
            affixes: HashMap::new(),
        })
        .collect();

    Ok(results)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let stashes = open_stashes(&cli.src);
    let mut max_item = None;
    let mut max_mods = vec![];
    for (_, data) in stashes {
        let stash_info: PublicStashData = serde_json::from_str(&data)?;
        for stash in stash_info.stashes {
            for item in stash.items {
                let mods = parse_mods(&item);
                if mods.len() > max_mods.len() {
                    max_mods = mods;
                    max_item = Some(item);
                }
            }
        }
    }
    let item = max_item.unwrap();
    println!(
        "trying to find similar item to {1} {0}",
        item.base_type, item.name
    );
    let mods = max_mods;

    let start = Instant::now();
    let result = match cli.command {
        Command::Redis => search_in_redis(&mods).await?,
    };
    let end = Instant::now();

    for it in result {
        println!("{} ({})", it.basetype, it.id);
        for (k, v) in it.affixes {
            println!("{} = {}", k, v);
        }
    }

    println!("time to search: {}ms", (end - start).as_millis());

    Ok(())
}
