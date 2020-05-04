use secrecy::{ExposeSecret, SecretString};
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPool;

pub mod import;
pub use import::DbImport;
type DbUrl = SecretString;

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Activity {
    pub name: String,
    id: i32,
    parent_id: Option<i32>,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct ActivityEvent {
    id: i32,
    parent_id: Option<i32>,
    activity_id: i32,
    notes: String,
}

pub struct Repo<'a> {
    pub pool: &'a PgPool,
}
impl<'a> Repo<'a> {
    pub async fn create_activity_event(
        &self,
        activity: &Activity,
        notes: &str,
        parent: Option<&ActivityEvent>,
    ) -> sqlx::Result<ActivityEvent> {
        sqlx::query_as!(
            ActivityEvent,
            "INSERT INTO activity_events ( activity_id, notes, parent_id ) \
            VALUES ( $1, $2, $3 ) \
            RETURNING * ",
            activity.id,
            notes,
            parent.map(|p| p.id)
        )
        .fetch_one(self.pool)
        .await
    }

    pub async fn find_or_create_activity(
        &self,
        name: &str,
        parent: Option<&Activity>,
    ) -> sqlx::Result<Activity> {
        match sqlx::query_as!(
            Activity,
            "select * from activities where lower(name) = lower(trim($1)) and parent_id=$2",
            name,
            parent.map(|p| p.id)
        )
        .fetch_optional(self.pool)
        .await?
        {
            Some(a) => Ok(a),
            None => {
                self.create_activity(NewActivity::with(name.trim()), parent)
                    .await
            }
        }
    }

    pub async fn create_activity(
        &self,
        activity: NewActivity,
        parent: Option<&Activity>,
    ) -> sqlx::Result<Activity> {
        sqlx::query_as!(
            Activity,
            "INSERT INTO activities ( name, parent_id ) VALUES ( $1, $2 ) RETURNING * ",
            activity.name,
            parent.map(|p| p.id)
        )
        .fetch_one(self.pool)
        .await
    }

    pub async fn fetch_activities(&self) -> sqlx::Result<Vec<Activity>> {
        sqlx::query_as!(Activity, "select * from activities")
            .fetch_all(self.pool)
            .await
    }
}

#[derive(Deserialize)]
pub struct NewActivity {
    pub name: String,
}

impl NewActivity {
    fn with(name: &str) -> NewActivity {
        NewActivity {
            name: name.to_owned(),
        }
    }
}

#[tracing::instrument]
pub async fn build_pool(db_url: DbUrl, num_cons: u32) -> sqlx::Result<PgPool> {
    PgPool::builder()
        .max_size(num_cons) // maximum number of connections in the pool
        .build(db_url.expose_secret())
        .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_util::reset_db;

    #[async_std::test]
    async fn test_activities() -> anyhow::Result<()> {
        dotenv::dotenv().ok();

        let pool = &reset_db().await?;
        let repo = Repo { pool };

        repo.find_or_create_activity("boom", None).await?;
        repo.find_or_create_activity("boom ", None).await?;
        repo.find_or_create_activity(" boom", None).await?;
        repo.find_or_create_activity("bOOm", None).await?;

        let activities = repo.fetch_activities().await?;
        assert_eq!(1, activities.len());
        Ok(())
    }
}
