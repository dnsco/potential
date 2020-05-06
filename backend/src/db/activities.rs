use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPool;

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Activity {
    pub name: String,
    pub id: i32,
    parent_id: Option<i32>,
}

pub struct ActivitiesRepo<'pool> {
    pub pool: &'pool PgPool,
}

#[derive(Deserialize)]
pub struct NewActivity {
    pub name: String,
}

impl<'pool> ActivitiesRepo<'pool> {
    pub async fn find_or_create(
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
                let activity = NewActivity {
                    name: name.trim().into(),
                };
                self.create(activity, parent).await
            }
        }
    }

    pub async fn create(
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

    pub async fn fetch(&self) -> sqlx::Result<Vec<Activity>> {
        sqlx::query_as!(Activity, "select * from activities")
            .fetch_all(self.pool)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_util::reset_db;

    #[async_std::test]
    async fn test_find_or_create() -> anyhow::Result<()> {
        dotenv::dotenv().ok();

        let pool = &reset_db().await?;
        let repo = ActivitiesRepo { pool };

        repo.find_or_create("boom", None).await?;
        repo.find_or_create("boom ", None).await?;
        repo.find_or_create(" boom", None).await?;
        repo.find_or_create("bOOm", None).await?;

        let activities = repo.fetch().await?;
        assert_eq!(1, activities.len());
        Ok(())
    }
}
