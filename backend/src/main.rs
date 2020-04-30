use dotenv;
use std::env;
use tracing_subscriber::EnvFilter;

mod api;
mod db;

#[async_std::main]
async fn main() -> Result<(), anyhow::Error> {
    dotenv::dotenv().ok();
    let log_env = env::var("RUST_LOG").unwrap_or("info".into());
    let db_url = env::var("DATABASE_URL")?;
    let db_cons = 10;
    let listen = "0.0.0.0:8080";


    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::new(log_env))
        .init();

    let pool = db::build_pool(&db_url, db_cons).await?;
    api::new(pool).listen(listen).await?;
    Ok(())
}
