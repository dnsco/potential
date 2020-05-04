use chrono::NaiveDate;
use csv::Trim;
use serde::{Deserialize, Deserializer};

use std::collections::HashMap;
use std::io;

use crate::db::Repo;
use duct::cmd;
use serde::de::Error as DeserError;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Record {
    #[serde(deserialize_with = "deserialize_date")]
    date: Option<NaiveDate>,
    exercise: String,
    reps: String,
    sets: String,
}

struct ParsedCsv<T> {
    reader: csv::Reader<T>,
}

type ParsedDays = HashMap<NaiveDate, Vec<Record>>;

impl<T: io::Read> ParsedCsv<T> {
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
            .filter_map(Result::ok)
            .filter(|r| Option::is_some(&r.date))
            .collect()
    }

    pub fn days(self) -> ParsedDays {
        let mut days: HashMap<NaiveDate, Vec<Record>> = HashMap::new();
        for row in self.rows() {
            let date = row.date.as_ref().unwrap();
            match days.get_mut(date) {
                Some(records) => records.push(row),
                None => {
                    days.insert(date.clone(), vec![row]);
                }
            };
        }
        days
    }
}

pub struct DbImport<'a> {
    repo: &'a Repo<'a>,
    days: ParsedDays,
}

impl<'a> DbImport<'a> {
    pub fn from(repo: &'a Repo<'a>, strength_url: String) -> Result<Self, io::Error> {
        // #todo: why does surf 502 but shelling out to curl work?
        // let spreadsheet = get_url(strength_url).await?;
        let spreadsheet = cmd!("curl", strength_url).read()?;

        Ok(DbImport {
            repo: &repo,
            days: ParsedCsv::from(spreadsheet.as_bytes()).days(),
        })
    }

    pub async fn run(self) -> anyhow::Result<()> {
        let workout_activity = self.repo.find_or_create_activity("Workout", None).await?;

        for (date, records) in self.days {
            let workout = self
                .repo
                .create_activity_event(&workout_activity, &format!("Workout {}", date), None)
                .await?;
            for record in records {
                let exercise = self
                    .repo
                    .find_or_create_activity(&record.exercise, Some(&workout_activity))
                    .await?;

                self.repo
                    .create_activity_event(&exercise, &record.sets, Some(&workout))
                    .await?;
            }
        }

        Ok(())
    }
}

const DATE_FORMAT: &str = "%Y-%m-%d";

pub fn deserialize_date<'de, D>(deserializer: D) -> Result<Option<NaiveDate>, D::Error>
where
    D: Deserializer<'de>,
{
    Option::deserialize(deserializer)?
        .map(|s: String| NaiveDate::parse_from_str(&s, DATE_FORMAT).map_err(DeserError::custom))
        .transpose()
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::test_util::reset_db;
    use std::env;
    use tap::TapOps;

    const TEST_SHEET: &str = "Date	Exercise	Reps	Sets\n\
    2020-04-01	Bicep Curl	6	25, 30, 35, 37.5 (cheat at 5)
    2020-04-02	Meow	7	30, 40, 45👍, 22 (woot)
    2020-04-01	Miliatary Press	7	30, 35, 37.5, 40, 45👍
    ";

    #[test]
    fn test_record() -> anyhow::Result<()> {
        let import = ParsedCsv::from(TEST_SHEET.as_bytes());
        let days = import.days();
        assert_eq!(2, days.len());
        let keys = days.keys().collect::<Vec<&NaiveDate>>().tap(|y| y.sort());
        let apr_1 = days.get(keys.first().unwrap()).unwrap();
        let apr_2 = days.get(keys.last().unwrap()).unwrap();

        assert_eq!(2, apr_1.len());
        assert_eq!(1, apr_2.len());

        Ok(())
    }

    #[async_std::test]
    async fn test_import() -> anyhow::Result<()> {
        dotenv::dotenv()?;
        let strength_url = env::var("STRENGTH_URL")?;
        let repo = Repo {
            pool: &reset_db().await?,
        };

        DbImport::from(&repo, strength_url)?.run().await?;

        let names = repo
            .fetch_activities()
            .await?
            .into_iter()
            .map(|a| a.name)
            .collect::<Vec<String>>()
            .tap(|n| n.sort());

        dbg!(names);
        Ok(())
    }
}
