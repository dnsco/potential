use serde::Serialize;
use sqlx::PgPool;

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct User {
    pub id: i32,
}

pub struct UsersRepo<'pool> {
    pub pool: &'pool PgPool,
}

impl<'pool> UsersRepo<'pool> {
    pub async fn create(&self) -> sqlx::Result<User> {
        sqlx::query_as!(User, "INSERT INTO users default values RETURNING *")
            .fetch_one(self.pool)
            .await
    }

    pub async fn find(&self, id: i32) -> sqlx::Result<User> {
        sqlx::query_as!(User, "SELECT * FROM users WHERE id=$1", id)
            .fetch_one(self.pool)
            .await
    }
}
