use sqlx::postgres::PgPool;
use sqlx::Result as SqlxResult;

#[derive(Debug, sqlx::FromRow)]
pub struct Activity {
    name: String,
    id: i32,
}

pub async fn fetch_activities(pool: &PgPool) -> SqlxResult<Vec<Activity>> {
    sqlx::query_as!(Activity, "select * from activities")
        .fetch_all(pool)
        .await
}

pub async fn build_pool(db_url: &String, num_cons: u32) -> SqlxResult<PgPool> {
    PgPool::builder()
        .max_size(num_cons) // maximum number of connections in the pool
        .build(&db_url)
        .await
}
