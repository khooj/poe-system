use std::{env::args, time::Duration};

use application::pipe_stashes::{insert_mods, parse_mods};
use cassandra_cpp::Cluster;
use public_stash::models::PublicStashData;
use tokio::io::{self, AsyncBufReadExt, BufReader};

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
    let affix_stmt = session
        .prepare("INSERT INTO poesystem.affixes(affix, value, item_id) VALUES (?, ?, ?);")
        .await
        .expect("cannot prepare affix query");
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
                let mods = parse_mods(&item);
                for m in mods {
                    let mut stmt = affix_stmt.bind();
                    stmt.bind_string(0, &m.stat_id)?;
                    stmt.bind_string(
                        1,
                        &m.numeric_value.map_or("-1".to_string(), |n| n.to_string()),
                    )?;
                    stmt.bind_string(2, &item.id.as_ref().expect("item does not have id"))?;
                    let handle = pool.spawn(async move { stmt.execute().await }).await?;
                    handles.push(handle);
                }
            }
        }
    }

    println!("handles size: {}", handles.len());
    for hndl in handles {
        hndl.await???;
    }

    Ok(())
}
