use std::{collections::HashMap, time::Duration};

use application::pipe_stashes::parse_mods;
use async_channel::bounded;
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
    Redis,
}

type SharedResult<T> = Result<T, Box<dyn std::error::Error>>;

async fn insert_into_redis(
    task_num: usize,
    mut lines: Lines<BufReader<Stdin>>,
) -> SharedResult<()> {
    let cwd = std::env::current_dir()?;
    let client = redis::Client::open(format!(
        "redis+unix:{}",
        cwd.join("data")
            .join("r1")
            .join("redis.sock")
            .to_str()
            .unwrap()
    ))?;

    let pool = tokio_task_pool::Pool::bounded(task_num)
        .with_spawn_timeout(Duration::from_secs(5))
        .with_run_timeout(Duration::from_secs(20));

    let (tx, rx) = bounded(task_num);
    for _ in 0..task_num {
        tx.send(client.get_multiplexed_async_connection().await?)
            .await?;
    }

    let mut handles = vec![];
    let rx_cloned = rx.clone();
    let tx_cloned = tx.clone();
    let h = pool
        .spawn(async move {
            let rx = rx_cloned;
            let tx = tx_cloned;
            let mut conn = rx.recv().await.unwrap();
            let mut iter: redis::AsyncIter<String> = conn.scan().await.unwrap();
            let mut to_remove = vec![];
            while let Some(item) = iter.next_item().await {
                if item.contains("affix") {
                    to_remove.push(item);
                }
            }
            std::mem::drop(iter);

            if !to_remove.is_empty() {
                let _: usize = conn.del(&to_remove).await.unwrap();
            }
            tx.send(conn).await.unwrap();
        })
        .await?;
    h.await??;

    while let Ok(Some(line)) = lines.next_line().await {
        let rx_cloned = rx.clone();
        let tx_cloned = tx.clone();
        let h = pool
            .spawn(async move {
                let tx = tx_cloned;
                let rx = rx_cloned;
                let mut conn = rx.recv().await.unwrap();
                let stash_info: PublicStashData = serde_json::from_str(&line).unwrap();
                let mut affixes: HashMap<String, Vec<&str>> = HashMap::new();
                for stash in &stash_info.stashes {
                    for item in &stash.items {
                        let mods = parse_mods(item);
                        for m in mods {
                            let lst = affixes.entry(format!("affix:{}", m.stat_id)).or_default();
                            lst.push(item.id.as_ref().unwrap());
                        }
                    }
                }
                for (k, lst) in affixes {
                    let _: usize = conn.sadd(k, lst).await.unwrap();
                }
                tx.send(conn).await.unwrap();
            })
            .await?;
        handles.push(h);
    }

    for h in handles {
        h.await??;
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let rdr = BufReader::new(io::stdin());
    let lines = rdr.lines();

    match cli.command {
        Command::Redis => insert_into_redis(cli.task_num, lines).await?,
    };

    Ok(())
}
