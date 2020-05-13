use secrecy::SecretString;
use sqlx::PgPool;
use std::env;

use crate::db::build_pool;

//todo: make this operate over multiple test dbs so cargo test passes instead of individual files
pub async fn test_db() -> anyhow::Result<PgPool> {
    let db_url = SecretString::new(env::var("DATABASE_URL")?);
    let pool = build_pool(db_url, 2).await?;
    Ok(pool)
}
