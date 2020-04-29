use crate::db::fetch_activities;
use sqlx::PgPool;
use tide::{Request, Server};

pub fn new(pool: PgPool) -> Server<PgPool> {
    let mut app = tide::with_state(pool);
    app.at("/").get(|req: Request<PgPool>| async move {
        let pool = req.state();
        let activities = fetch_activities(pool).await?;
        Ok(format!("{:?}", activities))
    });
    app
}
