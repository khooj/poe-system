use application::{pipe_stashes::parse_mods};
use cassandra_cpp::{AsRustType, Cluster, LendingIterator, MapIterator};
use clap::{Parser, Subcommand};
use domain::Mod;
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
    Cassandra,
    Redis,
}

struct FoundItem {
    id: String,
    basetype: String,
    affixes: HashMap<String, String>,
}

type SearchResult = Result<Vec<FoundItem>, Box<dyn std::error::Error>>;

async fn search_in_cassandra(mods: &Vec<Mod>) -> SearchResult {
    let mut cluster = Cluster::default();
    cluster.set_contact_points("127.0.0.1")?;
    cluster.set_load_balance_round_robin();
    let session = cluster.connect().await?;
    let mut ids: HashSet<String> = HashSet::new();
    let mut first_loaded = false;
    for m in mods {
        let mut stmt = session.statement("SELECT item_id FROM poesystem.affixes WHERE affix = ?;");
        stmt.bind_string(0, &m.stat_id)?;
        let result = stmt.execute().await?;
        let mut iter = result.iter();
        let mut new_ids = HashSet::new();
        while let Some(row) = iter.next() {
            let id = row.get_by_name("item_id")?;
            new_ids.insert(id);
        }

        if !first_loaded {
            first_loaded = true;
            ids = new_ids;
        } else {
            ids = new_ids.intersection(&ids).cloned().collect();
        }

        if ids.len() < 10 {
            break;
        }
    }

    let mut stmt =
        session.statement("SELECT id, basetype, affixes FROM poesystem.items WHERE id = ?;");
    stmt.bind_string(0, &ids.iter().nth(0).expect("item not found"))?;
    let result = stmt.execute().await?;
    println!("found items");
    let mut iter = result.iter();
    let mut results = vec![];
    while let Some(row) = iter.next() {
        let id: String = row.get_by_name("id")?;
        let basetype: String = row.get_by_name("basetype")?;
        let mut affixes: MapIterator = row.get_by_name("affixes")?;
        let mut aff = HashMap::new();
        while let Some((k, v)) = affixes.next() {
            aff.insert(k.to_string(), v.to_string()).unwrap();
        }
        results.push(FoundItem {
            id,
            basetype,
            affixes: aff,
        })
    }

    Ok(results)
}

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

    //let (_, data) = &stashes.nth(0).unwrap();
    //let idx = rand::thread_rng().sample(Uniform::new(0usize, stash_info.stashes.len()));
    //let stash = &stash_info.stashes[idx];
    //
    //let idx = rand::thread_rng().sample(Uniform::new(0usize, stash.items.len()));
    let item = max_item.unwrap();
    println!(
        "trying to find similar item to {1} {0}",
        item.base_type, item.name
    );
    let mods = max_mods;

    let start = Instant::now();
    let result = match cli.command {
        Command::Cassandra => search_in_cassandra(&mods).await?,
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
