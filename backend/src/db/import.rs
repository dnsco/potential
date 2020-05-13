use chrono::NaiveDate;
use csv::Trim;
use serde::{Deserialize, Deserializer};

use std::collections::HashMap;
use std::io;

use crate::db::users::User;
use crate::db::{ActivitiesRepo, ActivityEventsRepo, RepoFactory};
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
    days: ParsedDays,
    activities: ActivitiesRepo<'a>,
    activity_events: ActivityEventsRepo<'a>,
}

impl<'a> DbImport<'a> {
    pub fn from<T: RepoFactory>(repo: &'a T, strength_url: String) -> Result<Self, io::Error> {
        // #todo: why does surf 502 but shelling out to curl work?
        // let spreadsheet = get_url(strength_url).await?;
        let spreadsheet = cmd!("curl", strength_url).read()?;

        Ok(DbImport {
            days: ParsedCsv::from(spreadsheet.as_bytes()).days(),
            activities: repo.activities(),
            activity_events: repo.activity_events(),
        })
    }

    pub async fn run(self, user: &User) -> anyhow::Result<()> {
        let workout_activity = self
            .activities
            .find_or_create("Workout", None, user)
            .await?;

        for (date, records) in self.days {
            let workout = self
                .activity_events
                .create(&workout_activity, &format!("Workout {}", date), None)
                .await?;

            for record in records {
                let exercise = self
                    .activities
                    .find_or_create(&record.exercise, Some(&workout_activity), user)
                    .await?;

                self.activity_events
                    .create(&exercise, &record.sets, Some(&workout))
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

    use crate::test_util::test_db;
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
        let pool = &test_db().await?;

        let user = pool.users().create().await?;
        DbImport::from(pool, strength_url)?.run(&user).await?;

        let names = pool
            .activities()
            .fetch_for(&user)
            .await?
            .into_iter()
            .map(|a| a.name)
            .collect::<Vec<String>>()
            .tap(|n| n.sort());

        dbg!(names);
        Ok(())
    }
}
