use secrecy::SecretString;
use std::env;
use tracing_subscriber::EnvFilter;

mod db;
mod web;

#[async_std::main]
async fn main() -> Result<(), anyhow::Error> {
    dotenv::dotenv().ok();
    let log_env = env::var("RUST_LOG").unwrap_or_else(|_| "info".into());
    let db_url = SecretString::new(env::var("DATABASE_URL")?);
    let db_cons = 10;
    let listen = "0.0.0.0:8080";

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::new(log_env))
        .init();

    let pool = db::build_pool(db_url, db_cons).await?;
    web::new(pool).listen(listen).await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    #[allow(dead_code)]
    async fn get_url(strength_url: String) -> anyhow::Result<String> {
        surf::get(strength_url)
            .recv_string()
            .await
            .map_err(|e| anyhow::anyhow!(e))
    }
}

#[cfg(test)]
mod test_util;
