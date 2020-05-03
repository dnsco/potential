use secrecy::{ExposeSecret, SecretString};
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPool;

type DbUrl = SecretString;

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Activity {
    pub name: String,
    id: i32,
    parent_id: Option<i32>,
}

#[allow(dead_code)]
pub async fn find_or_create_activity(pool: &PgPool, name: &str) -> sqlx::Result<Activity> {
    match sqlx::query_as!(
        Activity,
        "select * from activities where lower(name) = lower(trim($1))",
        name
    )
    .fetch_optional(pool)
    .await?
    {
        Some(a) => Ok(a),
        None => create_activity(pool, NewActivity::with(name.trim())).await,
    }
}

pub async fn fetch_activities(pool: &PgPool) -> sqlx::Result<Vec<Activity>> {
    sqlx::query_as!(Activity, "select * from activities")
        .fetch_all(pool)
        .await
}

#[derive(Deserialize)]
pub struct NewActivity {
    pub name: String,
}

impl NewActivity {
    fn with(name: &str) -> NewActivity {
        NewActivity {
            name: name.to_owned(),
        }
    }
}

pub async fn create_activity(pool: &PgPool, activity: NewActivity) -> sqlx::Result<Activity> {
    sqlx::query_as!(
        Activity,
        "INSERT INTO activities ( name ) VALUES ( $1 ) RETURNING * ",
        activity.name
    )
    .fetch_one(pool)
    .await
}

#[tracing::instrument]
pub async fn build_pool(db_url: DbUrl, num_cons: u32) -> sqlx::Result<PgPool> {
    PgPool::builder()
        .max_size(num_cons) // maximum number of connections in the pool
        .build(db_url.expose_secret())
        .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_util::reset_db;

    #[async_std::test]
    async fn test_activities() -> anyhow::Result<()> {
        dotenv::dotenv().ok();
        let pool = reset_db().await?;

        find_or_create_activity(&pool, "boom").await?;
        find_or_create_activity(&pool, "boom ").await?;
        find_or_create_activity(&pool, " boom").await?;
        find_or_create_activity(&pool, "bOOm").await?;

        let activities = fetch_activities(&pool).await?;
        assert_eq!(1, activities.len());
        Ok(())
    }
}
