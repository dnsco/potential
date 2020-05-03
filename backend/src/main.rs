use secrecy::SecretString;
use std::env;
use tracing_subscriber::EnvFilter;

mod api;
mod db;

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
    api::new(pool).listen(listen).await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    use duct::cmd;

    #[async_std::test]
    async fn booyah() -> anyhow::Result<()> {
        dotenv::dotenv()?;
        let strength_url = env::var("STRENGTH_URL")?;
        dbg!(&strength_url);
        reset_db()?;

        // #todo: why does surf 502 but shelling out to curl work?
        // let spreadsheet = get_url(strength_url).await?;
        let spreadsheet = cmd!("curl", strength_url).read()?;
        dbg!(&spreadsheet);

        let mut reader = csv::ReaderBuilder::new()
            .delimiter(b'\t')
            .from_reader(spreadsheet.as_bytes());
        dbg!(reader.headers()?);
        dbg!(reader.records().next().unwrap()?);

        Ok(())
    }

    #[allow(dead_code)]
    async fn get_url(strength_url: String) -> anyhow::Result<String> {
        surf::get(strength_url)
            .recv_string()
            .await
            .map_err(|e| anyhow::anyhow!(e))
    }

    fn reset_db() -> anyhow::Result<()> {
        cmd!("dbmate", "drop").run()?;
        cmd!("dbmate", "create").run()?;
        cmd!("dbmate", "migrate").run()?;

        Ok(())
    }
}
