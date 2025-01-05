use std::{env::args, time::Duration};

use cassandra_cpp::{CassCollection, Cluster, Map, Statement};
use domain::{Mod, ModType};
use public_stash::models::{Item, PublicStashData};
use tokio::io::{self, AsyncBufReadExt, BufReader};
use uuid::Uuid;

fn parse_mods(item: &Item) -> Map {
    let mut affixes = Map::new();
    let mods = [
        &item.utility_mods,
        &item.enchant_mods,
        &item.scourge_mods,
        &item.implicit_mods,
        &item.explicit_mods,
        &item.crafted_mods,
        &item.fractured_mods,
        &item.veiled_mods,
    ];

    let mods: Vec<String> = mods
        .into_iter()
        .filter_map(|s| s.clone())
        .flatten()
        .collect::<Vec<_>>();

    let mods = mods
        .iter()
        .map(|m| (m.as_str(), ModType::Explicit))
        .collect::<Vec<_>>();
    let mods = Mod::many_by_stat(&mods);

    for m in mods {
        affixes.append_string(&m.stat_id).unwrap();
        affixes
            .append_string(
                &m.numeric_value
                    .map(|n| n.to_string())
                    .unwrap_or("-1".to_string()),
            )
            .unwrap();
    }
    affixes
}

fn insert_mods(mut stmt: Statement, item: &Item) -> Statement {
    stmt.bind_string(0, item.id.as_ref().unwrap_or(&Uuid::new_v4().to_string()))
        .unwrap();
    stmt.bind_string(1, &item.base_type).unwrap();
    let affixes = parse_mods(&item);
    stmt.bind_map(2, affixes).unwrap();
    stmt
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = args().into_iter().collect::<Vec<String>>();
    if args.len() < 2 {
        eprintln!("Usage: insert_into_cass <concurrent_task_num>");
        return Ok(());
    }

    let task_num = args[1].parse().unwrap();
    let mut cluster = Cluster::default();
    cluster.set_contact_points("127.0.0.1")?;
    cluster.set_load_balance_round_robin();
    let session = cluster.connect().await?;
    session.execute("TRUNCATE TABLE poesystem.items;").await?;

    let pool = tokio_task_pool::Pool::bounded(task_num)
        .with_spawn_timeout(Duration::from_secs(5))
        .with_run_timeout(Duration::from_secs(20));

    let rdr = BufReader::new(io::stdin());
    let mut lines = rdr.lines();
    let mut handles = vec![];
    let mut processed = 0;
    let insert_stmt = session
        .prepare("INSERT INTO poesystem.items(id, basetype, affixes) VALUES (?, ?, ?);")
        .await
        .expect("cannot prepare query");
    while let Ok(Some(line)) = lines.next_line().await {
        processed += 1;
        if (processed + 1) % 20 == 0 {
            println!("processed lines: {}", processed);
        }

        let stash_info: PublicStashData =
            serde_json::from_str(&line).expect("cannot deserialize stash");
        for stash in stash_info.stashes {
            for item in stash.items {
                let stmt = insert_mods(insert_stmt.bind(), &item);
                let handle = pool.spawn(async move { stmt.execute().await }).await?;
                handles.push(handle);
            }
        }
    }

    println!("handles size: {}", handles.len());
    for hndl in handles {
        hndl.await???;
    }

    Ok(())
}
