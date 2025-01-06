use std::{
    collections::{HashMap, HashSet},
    env::args,
};

use application::{
    pipe_stashes::{insert_mods, parse_mods},
    ArchiveStashes, DirStashes,
};
use cassandra_cpp::{AsRustType, Cluster, LendingIterator, Map, MapIterator, Statement};
use public_stash::models::PublicStashData;
use rand::{distributions::Uniform, Rng};
use tokio::time::Instant;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = args().into_iter().collect::<Vec<_>>();
    if args.len() < 2 {
        eprintln!("Usage: request_cass <archive or dir>");
        return Ok(());
    }

    let mut stashes = if std::fs::metadata(&args[1]).unwrap().is_dir() {
        DirStashes::new(&args[1]).into_iter()
    } else {
        ArchiveStashes::new(&args[1]).into_iter()
    };
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

    let mut cluster = Cluster::default();
    cluster.set_contact_points("127.0.0.1")?;
    cluster.set_load_balance_round_robin();
    let session = cluster.connect().await?;

    let start = Instant::now();
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
    while let Some(row) = iter.next() {
        let id: String = row.get_by_name("id")?;
        let basetype: String = row.get_by_name("basetype")?;
        let mut affixes: MapIterator = row.get_by_name("affixes")?;
        println!("{} ({})", basetype, id);
        while let Some((k, v)) = affixes.next() {
            println!("{} = {}", k, v);
        }
    }

    let end = Instant::now();
    println!("time to search: {}", (end - start).as_millis());

    Ok(())
}
