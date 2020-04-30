use serde::Serialize;
use sqlx::postgres::PgPool;

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Activity {
    name: String,
    id: i32,
}

pub async fn fetch_activities(pool: &PgPool) -> sqlx::Result<Vec<Activity>> {
    sqlx::query_as!(Activity, "select * from activities")
        .fetch_all(pool)
        .await
}

pub async fn create_activity(pool: &PgPool, name: &str) -> sqlx::Result<Activity> {
    sqlx::query_as!(
        Activity,
        "INSERT INTO activities ( name ) VALUES ( $1 ) RETURNING * ",
        name
    )
    .fetch_one(pool)
    .await
}

pub async fn build_pool(db_url: &String, num_cons: u32) -> sqlx::Result<PgPool> {
    PgPool::builder()
        .max_size(num_cons) // maximum number of connections in the pool
        .build(&db_url)
        .await
}
