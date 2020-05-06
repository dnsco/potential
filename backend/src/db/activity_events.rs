use serde::Serialize;
use sqlx::postgres::PgPool;

use crate::db::Activity;

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct ActivityEvent {
    id: i32,
    parent_id: Option<i32>,
    activity_id: i32,
    notes: String,
}

pub struct ActivityEventsRepo<'pool> {
    pub pool: &'pool PgPool,
}

impl<'pool> ActivityEventsRepo<'pool> {
    pub async fn create(
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

    pub async fn fetch(&self) -> sqlx::Result<Vec<ActivityEvent>> {
        sqlx::query_as!(ActivityEvent, "select * from activity_events")
            .fetch_all(self.pool)
            .await
    }
}
