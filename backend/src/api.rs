use crate::db::{create_activity, fetch_activities};
use http_types::headers::HeaderValue;
use http_types::StatusCode;
use serde::Serialize;
use sqlx::PgPool;
use tide::security::{CorsMiddleware, Origin};
use tide::{Request, Response, Server};

type State = PgPool;

pub fn new(pool: PgPool) -> Server<PgPool> {
    let cors = CorsMiddleware::new()
        .allow_methods("GET, POST, OPTIONS".parse::<HeaderValue>().unwrap())
        .allow_origin(Origin::from("*"))
        .allow_credentials(false);

    let mut app = tide::with_state(pool);
    app.middleware(cors);
    app.at("/")
        .get(|_| async { Ok(String::from("Server is up.")) });
    app.at("/activities")
        .get(|req: Request<State>| async move {
            let activities = fetch_activities(req.state()).await?;
            Ok(to_json_response(activities))
        })
        .post(|req: Request<State>| async move {
            let activity = create_activity(req.state(), "Hai").await?;
            Ok(to_json_response(&activity))
        });
    app
}

fn to_json_response<T: Serialize + std::fmt::Debug>(entitiy: T) -> Response {
    Response::new(StatusCode::Ok)
        .body_json(&entitiy)
        .expect(&format!("Failed to serialize {:?}", entitiy))
}
