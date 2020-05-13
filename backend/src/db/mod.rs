use secrecy::{ExposeSecret, SecretString};
use sqlx::postgres::PgPool;

use crate::db::users::UsersRepo;
pub use activities::{ActivitiesRepo, Activity, NewActivity};
pub use activity_events::ActivityEventsRepo;
pub use import::DbImport;

mod activities;
mod activity_events;
mod users;

mod import;

pub trait RepoFactory {
    fn activities(&self) -> ActivitiesRepo;
    fn activity_events(&self) -> ActivityEventsRepo;
    fn users(&self) -> UsersRepo;
}

impl RepoFactory for PgPool {
    fn activities(&self) -> ActivitiesRepo {
        ActivitiesRepo { pool: self }
    }

    fn activity_events(&self) -> ActivityEventsRepo {
        ActivityEventsRepo { pool: self }
    }

    fn users(&self) -> UsersRepo {
        UsersRepo { pool: self }
    }
}

type DbUrl = SecretString;

#[tracing::instrument]
pub async fn build_pool(db_url: DbUrl, num_cons: u32) -> sqlx::Result<PgPool> {
    PgPool::builder()
        .max_size(num_cons) // maximum number of connections in the pool
        .build(db_url.expose_secret())
        .await
}
