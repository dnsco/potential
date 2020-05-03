use csv::Trim;
use sqlx::PgPool;
use std::io;

use crate::db::find_or_create_activity;

#[allow(dead_code)]
pub async fn import(pool: &PgPool, spreadsheet: String) -> anyhow::Result<()> {
    let mut reader = reader(spreadsheet.as_bytes());

    for record in reader.records() {
        let r = record?;
        let name = r.get(1).unwrap();
        if !name.is_empty() {
            find_or_create_activity(&pool, name).await?;
        }
    }

    Ok(())
}

fn reader<R: io::Read>(spreadsheet: R) -> csv::Reader<R> {
    csv::ReaderBuilder::new()
        .delimiter(b'\t')
        .trim(Trim::All)
        .from_reader(spreadsheet)
}

#[cfg(test)]
mod tests {
    use super::*;

    use duct::cmd;
    use std::env;

    use crate::db::fetch_activities;
    use crate::test_util::reset_db;

    #[async_std::test]
    async fn test_import() -> anyhow::Result<()> {
        dotenv::dotenv()?;
        let strength_url = env::var("STRENGTH_URL")?;
        let pool = reset_db().await?;

        // #todo: why does surf 502 but shelling out to curl work?
        // let spreadsheet = get_url(strength_url).await?;
        let spreadsheet = cmd!("curl", strength_url).read()?;
        import(&pool, spreadsheet).await?;
        let mut names: Vec<String> = fetch_activities(&pool)
            .await?
            .into_iter()
            .map(|a| a.name)
            .collect();
        names.sort();
        dbg!(names);
        Ok(())
    }
}
