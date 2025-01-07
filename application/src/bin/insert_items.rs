use std::{collections::HashMap, num::NonZeroUsize, time::Duration};

use application::pipe_stashes::{insert_mods, parse_mods};
use cassandra_cpp::Cluster;
use clap::{Parser, Subcommand};
use public_stash::models::PublicStashData;
use redis::AsyncCommands;
use tokio::io::{self, AsyncBufReadExt, BufReader, Lines, Stdin};

#[derive(Parser)]
#[command(version, about)]
struct Cli {
    #[arg(default_value = "30")]
    task_num: usize,
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Cassandra,
    Redis,
}

type SharedResult<T> = Result<T, Box<dyn std::error::Error>>;

async fn insert_into_cass(task_num: usize, mut lines: Lines<BufReader<Stdin>>) -> SharedResult<()> {
    let mut cluster = Cluster::default();
    cluster.set_contact_points("127.0.0.1")?;
    cluster.set_load_balance_round_robin();
    let session = cluster.connect().await?;
    session.execute("TRUNCATE TABLE poesystem.items;").await?;
    session.execute("TRUNCATE TABLE poesystem.affixes;").await?;

    let pool = tokio_task_pool::Pool::bounded(task_num)
        .with_spawn_timeout(Duration::from_secs(5))
        .with_run_timeout(Duration::from_secs(20));

    let insert_stmt = session
        .prepare("INSERT INTO poesystem.items(id, basetype, affixes) VALUES (?, ?, ?);")
        .await
        .expect("cannot prepare query");
    let affix_stmt = session
        .prepare("INSERT INTO poesystem.affixes(affix, value, item_id) VALUES (?, ?, ?);")
        .await
        .expect("cannot prepare affix query");
    let mut handles = vec![];
    let mut processed = 0;
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

    println!("processed: {}", processed);

    Ok(())
}

async fn insert_into_redis(mut lines: Lines<BufReader<Stdin>>) -> SharedResult<()> {
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

    let mut iter: redis::AsyncIter<String> = conn.scan().await?;
    let mut to_remove = vec![];
    while let Some(item) = iter.next_item().await {
        if item.contains("affix") {
            to_remove.push(item);
        }
    }

    std::mem::drop(iter);

    for item in to_remove {
        let l = conn.llen(&item).await?;
        conn.lpop(&item, NonZeroUsize::new(l)).await?;
    }

    while let Ok(Some(line)) = lines.next_line().await {
        let stash_info: PublicStashData = serde_json::from_str(&line)?;
        for stash in stash_info.stashes {
            let mut affixes: HashMap<String, Vec<String>> = HashMap::new();
            for item in stash.items {
                let mods = parse_mods(&item);
                for m in mods {
                    let lst = affixes.entry(format!("affix:{}", m.stat_id)).or_default();
                    lst.push(item.id.clone().unwrap());
                }
            }
            for (k, lst) in affixes {
                conn.lpush(k, lst).await?;
            }
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let rdr = BufReader::new(io::stdin());
    let lines = rdr.lines();

    match cli.command {
        Command::Cassandra => insert_into_cass(cli.task_num, lines).await?,
        Command::Redis => insert_into_redis(lines).await?,
    };

    Ok(())
}
