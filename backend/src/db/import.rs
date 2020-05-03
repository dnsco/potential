use csv::Trim;
use serde::Deserialize;
use sqlx::PgPool;
use std::io;

use crate::db::find_or_create_activity;
use chrono::{DateTime, Utc};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Record {
    #[serde(with = "maybe_date")]
    date: Option<DateTime<Utc>>,
    exercise: String,
    reps: String,
    sets: String,
}

struct Import<T> {
    reader: csv::Reader<T>,
}

impl<T: io::Read> Import<T> {
    fn from(spreadsheet: T) -> Self {
        Self {
            reader: csv::ReaderBuilder::new()
                .delimiter(b'\t')
                .trim(Trim::All)
                .from_reader(spreadsheet),
        }
    }

    pub fn rows(mut self) -> Vec<Record> {
        self.reader
            .deserialize::<Record>()
            .into_iter()
            .filter_map(Result::ok)
            .filter(|r| Option::is_some(&r.date))
            .collect()
    }
}

#[allow(dead_code)]
pub async fn import(pool: &PgPool, spreadsheet: String) -> anyhow::Result<()> {
    for row in Import::from(spreadsheet.as_bytes()).rows() {
        find_or_create_activity(&pool, &row.exercise).await?;
    }

    Ok(())
}

mod maybe_date {
    use chrono::{DateTime, TimeZone, Utc};
    use serde::{self, Deserialize, Deserializer};

    const FORMAT: &str = "%Y-%m-%d %H:%M:%S";
    const TWO_OCLOCK_PM: &str = " 14:00:00";

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<DateTime<Utc>>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let mut s = String::deserialize(deserializer)?;
        let val = if s.is_empty() {
            None
        } else {
            s.push_str(TWO_OCLOCK_PM);

            Some(
                Utc.datetime_from_str(&s, FORMAT)
                    .map_err(serde::de::Error::custom)?,
            )
        };

        Ok(val)
    }
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
