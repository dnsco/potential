use crate::db::build_pool;
use duct::cmd;
use secrecy::SecretString;
use sqlx::PgPool;
use std::env;

//todo: make this operate over multiple test dbs so cargo test passes instead of individual files
pub async fn reset_db() -> anyhow::Result<PgPool> {
    cmd!("dbmate", "drop").run()?;
    cmd!("dbmate", "create").run()?;
    cmd!("dbmate", "migrate").run()?;
    let db_url = SecretString::new(env::var("DATABASE_URL")?);
    let pool = build_pool(db_url, 2).await?;
    Ok(pool)
}
