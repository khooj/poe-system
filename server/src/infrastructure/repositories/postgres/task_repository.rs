use super::PgTransaction;
use anyhow::Result;
use serde_json::Value;
use sqlx::{
    types::{
        chrono::{DateTime, Utc},
        uuid::Uuid,
        Json,
    },
    PgPool,
};

#[derive(sqlx::Type, Debug, PartialEq)]
#[sqlx(type_name = "task_type", rename_all = "lowercase")]
pub enum TaskType {
    CalculateBuild,
}

pub struct Task {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub task_type: TaskType,
    pub data: Value,
}

impl Task {
    pub fn new(task_type: TaskType, data: Value) -> Task {
        Task {
            id: Uuid::new_v4(),
            created_at: Utc::now(),
            task_type,
            data,
        }
    }
}

#[derive(Clone)]
pub struct TaskRepository {
    pool: PgPool,
}

impl TaskRepository {
    pub fn new(pool: PgPool) -> TaskRepository {
        TaskRepository { pool }
    }

    pub async fn begin(&self) -> Result<PgTransaction<'_>> {
        Ok(PgTransaction {
            transaction: self.pool.begin().await?,
        })
    }

    pub async fn new_task(
        &self,
        transaction: &mut PgTransaction<'_>,
        task: Task,
    ) -> Result<String> {
        let _ = sqlx::query!(
            r#"
INSERT INTO tasks (id, created_at, task_type, data)
VALUES ($1, $2, $3, $4)
            "#,
            task.id,
            task.created_at,
            task.task_type as _,
            task.data,
        )
        .execute(&mut *transaction.transaction)
        .await?;
        Ok(task.id.to_string())
    }

    pub async fn get_latest_tasks(&self, limit: i64) -> Result<Vec<Task>> {
        let tasks = sqlx::query_as!(
            Task,
            r#"
SELECT id, created_at, task_type as "task_type: TaskType", data FROM tasks
ORDER BY created_at
LIMIT $1 
        "#,
            limit,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(tasks)
    }

    pub async fn remove_tasks(
        &self,
        transaction: &mut PgTransaction<'_>,
        ids: &[Uuid],
    ) -> Result<()> {
        let _ = sqlx::query!("DELETE FROM tasks WHERE id = ANY($1)", ids)
            .execute(&mut *transaction.transaction)
            .await?;
        Ok(())
    }
}
