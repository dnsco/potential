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

    use crate::db::{fetch_activities, find_or_create_activity};
    use crate::test_util::reset_db;
    use csv::Trim;
    use duct::cmd;

    #[async_std::test]
    async fn booyah() -> anyhow::Result<()> {
        dotenv::dotenv()?;
        let strength_url = env::var("STRENGTH_URL")?;
        let pool = reset_db().await?;

        // #todo: why does surf 502 but shelling out to curl work?
        // let spreadsheet = get_url(strength_url).await?;
        let spreadsheet = cmd!("curl", strength_url).read()?;

        let mut reader = csv::ReaderBuilder::new()
            .delimiter(b'\t')
            .trim(Trim::All)
            .from_reader(spreadsheet.as_bytes());

        dbg!(reader.headers()?);

        for record in reader.records() {
            let r = record?;
            let name = r.get(1).unwrap();
            if !name.is_empty() {
                find_or_create_activity(&pool, name).await?;
            }
        }

        let mut names: Vec<String> = fetch_activities(&pool)
            .await?
            .into_iter()
            .map(|a| a.name)
            .collect();
        names.sort();
        dbg!(names);
        Ok(())
    }

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
