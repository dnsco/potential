use crate::db::{DbImport, RepoFactory};

use http_types::headers::HeaderValue;
use http_types::StatusCode;
use sqlx::PgPool;
use std::env;
use tide::security::{CorsMiddleware, Origin};

mod api;

pub fn new(pool: PgPool) -> tide::Server<PgPool> {
    let cors = CorsMiddleware::new()
        .allow_methods("GET, POST, OPTIONS".parse::<HeaderValue>().unwrap())
        .allow_origin(Origin::from("*"))
        .allow_credentials(false);

    let mut app = tide::with_state(pool);
    app.middleware(cors);

    app.at("/")
        .get(|_| async { Ok(String::from("Server is up.")) });

    let mut api = app.at("/api");
    api.at("/activities")
        .get(api::activities::list)
        .post(api::activities::create);
    api.at("/activity_events").get(api::activity_events::list);

    let mut db = app.at("/db");
    db.at("/import").get(import);
    db.at("/reset").get(reset);

    app
}

pub async fn import(req: tide::Request<impl RepoFactory>) -> tide::Result<String> {
    let strength_url = env::var("STRENGTH_URL")?;
    let pool = req.state();
    let user = pool.users().find(1).await?;

    DbImport::from(pool, strength_url)?
        .run(&user)
        .await
        .map_err(|e| tide::Error::from_str(StatusCode::InternalServerError, e))?;

    Ok(String::from("Import Success"))
}

pub async fn reset(req: tide::Request<PgPool>) -> tide::Result<String> {
    let user = crate::db::reset(req.state())
        .await
        .map_err(|e| tide::Error::from_str(StatusCode::InternalServerError, e))?;
    Ok(format!("db reset: {:?}", user))
}
