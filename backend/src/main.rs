use dotenv;
use std::env;
use tracing::{Level};
use tracing_subscriber;

mod api;
mod db;

#[async_std::main]
async fn main() -> Result<(), anyhow::Error> {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt().with_max_level(Level::TRACE).init();

    let db_url = env::var("DATABASE_URL")?;
    let db_cons = 10;
    let listen = "0.0.0.0:8080";

    let pool = db::build_pool(&db_url, db_cons).await?;
    api::new(pool).listen(listen).await?;
    Ok(())
}
