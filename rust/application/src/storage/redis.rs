use std::{
    collections::{HashMap, HashSet},
    time::Duration,
};

use domain::{build_calculation::typed_item::TypedItem, item::types::Mod};
use redis::AsyncCommands;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RedisIndexError {
    #[error("connect")]
    Connect(#[from] redis::RedisError),
    #[error("channel")]
    Channel,
    #[error("task pool")]
    TaskPool(#[from] tokio_task_pool::Error),
    #[error("tokio join")]
    Join(#[from] tokio::task::JoinError),
}

pub struct RedisIndexOptions {
    uri: String,
    task_num: usize,
}

impl Default for RedisIndexOptions {
    fn default() -> Self {
        RedisIndexOptions {
            task_num: 6,
            uri: "redis://localhost:6379".to_string(),
        }
    }
}

impl RedisIndexOptions {
    pub fn set_uri(mut self, uri: &str) -> RedisIndexOptions {
        self.uri = uri.to_string();
        self
    }

    pub fn set_task_num(mut self, num: usize) -> RedisIndexOptions {
        self.task_num = num;
        self
    }

    pub fn build(self) -> Result<RedisIndexRepository, RedisIndexError> {
        let client = redis::Client::open(self.uri)?;
        let pool = tokio_task_pool::Pool::bounded(self.task_num)
            .with_spawn_timeout(Duration::from_secs(5))
            .with_run_timeout(Duration::from_secs(20));

        Ok(RedisIndexRepository { client, pool })
    }
}

pub struct RedisIndexRepository {
    client: redis::Client,
    pool: tokio_task_pool::Pool,
}

impl RedisIndexRepository {
    pub async fn delete_items(&mut self, items: Vec<String>) -> Result<(), RedisIndexError> {
        let mut conn = self
            .client
            .get_multiplexed_async_connection()
            .await
            .unwrap();
        let mut iter: redis::AsyncIter<String> = conn.scan().await.unwrap();
        let mut to_check = vec![];
        while let Some(item) = iter.next_item().await {
            if item.contains("affix") {
                to_check.push(item);
            }
        }
        std::mem::drop(iter);

        for chk in to_check {
            for chunk in items.chunks(10) {
                let _: usize = conn.srem(&chk, chunk).await.unwrap();
            }
        }
        Ok(())
    }

    pub async fn insert_items(&mut self, items: Vec<TypedItem>) -> Result<(), RedisIndexError> {
        let mut handles = vec![];
        let conn = self.client.get_multiplexed_async_connection().await?;

        for item in items {
            let mut conn = conn.clone();
            let h = self
                .pool
                .spawn(async move {
                    let mut affixes: HashMap<String, Vec<&str>> = HashMap::new();
                    for m in item.mods() {
                        let lst = affixes.entry(format!("affix:{}", m.stat_id)).or_default();
                        lst.push(item.id.as_str());
                    }
                    for (k, lst) in affixes {
                        let _: usize = conn.sadd(k, lst).await.unwrap();
                    }
                })
                .await?;
            handles.push(h);
        }

        for h in handles {
            h.await??;
        }

        Ok(())
    }

    pub async fn search(&mut self, mods: &Vec<Mod>) -> Result<Vec<String>, RedisIndexError> {
        let mut conn = self.client.get_multiplexed_async_connection().await?;
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
        }

        Ok(ids.into_iter().collect())
    }
}
