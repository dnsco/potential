use crate::db::fetch_activities;
use http_types::headers::HeaderValue;
use sqlx::PgPool;
use tide::security::{CorsMiddleware, Origin};
use tide::{Request, Server};

pub fn new(pool: PgPool) -> Server<PgPool> {
    let rules = CorsMiddleware::new()
        .allow_methods("GET, POST, OPTIONS".parse::<HeaderValue>().unwrap())
        .allow_origin(Origin::from("*"))
        .allow_credentials(false);

    let mut app = tide::with_state(pool);
    app.middleware(rules);
    app.at("/").get(|req: Request<PgPool>| async move {
        let pool = req.state();
        let activities = fetch_activities(pool).await?;
        Ok(format!("{:?}", activities))
    });
    app
}
