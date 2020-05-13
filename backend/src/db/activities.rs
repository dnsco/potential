use crate::db::users::User;
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPool;

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Activity {
    pub id: i32,
    user_id: i32,
    pub name: String,
    parent_id: Option<i32>,
}

pub struct ActivitiesRepo<'pool> {
    pub pool: &'pool PgPool,
}

#[derive(Deserialize)]
pub struct NewActivity {
    pub user_id: i32,
    pub name: String,
}

impl<'pool> ActivitiesRepo<'pool> {
    pub async fn find_or_create(
        &self,
        name: &str,
        parent: Option<&Activity>,
        user: &User,
    ) -> sqlx::Result<Activity> {
        match self.find_by_user_and_name(user, name).await? {
            Some(a) => Ok(a),
            None => {
                let activity = NewActivity {
                    user_id: user.id,
                    name: name.trim().into(),
                };
                self.create(activity, parent).await
            }
        }
    }

    async fn find_by_user_and_name(
        &self,
        user: &User,
        name: &str,
    ) -> sqlx::Result<Option<Activity>> {
        sqlx::query_as!(
            Activity,
            "select * from activities where user_id=$1 and lower(name) = lower(trim($2))",
            user.id,
            name,
        )
        .fetch_optional(self.pool)
        .await
    }

    pub async fn create(
        &self,
        activity: NewActivity,
        parent: Option<&Activity>,
    ) -> sqlx::Result<Activity> {
        sqlx::query_as!(
            Activity,
            "INSERT INTO activities ( user_id, name, parent_id ) VALUES ( $1, $2, $3 ) RETURNING * ",
            activity.user_id,
            activity.name,
            parent.map(|p| p.id)
        )
            .fetch_one(self.pool)
            .await
    }

    pub async fn fetch_for(&self, user: &User) -> sqlx::Result<Vec<Activity>> {
        sqlx::query_as!(
            Activity,
            "select * from activities where user_id = $1",
            user.id
        )
        .fetch_all(self.pool)
        .await
    }
}

#[cfg(test)]
mod tests {
    use crate::db::RepoFactory;
    use crate::test_util::test_db;

    #[async_std::test]
    async fn test_find_or_create() -> anyhow::Result<()> {
        dotenv::dotenv().ok();

        let pool = &test_db().await?;
        let user = &pool.users().create().await?;
        let repo = pool.activities();

        repo.find_or_create("boom", None, user).await?;
        repo.find_or_create("boom ", None, user).await?;
        repo.find_or_create(" boom", None, user).await?;
        repo.find_or_create("bOOm", None, user).await?;

        let activities = repo.fetch_for(user).await?;
        assert_eq!(1, activities.len());
        Ok(())
    }
}
