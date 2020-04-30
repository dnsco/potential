use http_types::headers::HeaderValue;
use tide::security::{CorsMiddleware, Origin};

use crate::api::util::ApiState;

pub fn new(pool: sqlx::PgPool) -> tide::Server<ApiState> {
    let cors = CorsMiddleware::new()
        .allow_methods("GET, POST, OPTIONS".parse::<HeaderValue>().unwrap())
        .allow_origin(Origin::from("*"))
        .allow_credentials(false);

    let mut app = tide::with_state(pool);
    app.middleware(cors);
    app.at("/")
        .get(|_| async { Ok(String::from("Server is up.")) });
    app.at("/activities")
        .get(activities::list)
        .post(activities::create);
    app
}

mod util {
    use http_types::StatusCode;
    use serde::Serialize;
    use sqlx::PgPool;

    pub type ApiState = PgPool;
    pub type ApiRequest = tide::Request<ApiState>;
    pub type ApiResult = tide::Result<tide::Response>;

    pub fn to_json_response<T: Serialize + std::fmt::Debug>(entitiy: T) -> ApiResult {
        tide::Response::new(StatusCode::Ok)
            .body_json(&entitiy)
            .map_err(|e| tide::Error::new(StatusCode::InternalServerError, e))
    }
}

pub mod activities {
    use crate::api::util::{to_json_response, ApiRequest, ApiResult};
    use crate::db::{create_activity, fetch_activities};
    use serde::Deserialize;

    pub async fn list(req: ApiRequest) -> ApiResult {
        let activities = fetch_activities(req.state()).await?;
        to_json_response(activities)
    }

    #[derive(Deserialize)]
    pub struct NewActivity {
        pub name: String,
    }

    pub async fn create(mut req: ApiRequest) -> ApiResult {
        let new: NewActivity = req.body_json().await?;
        let activity = create_activity(req.state(), new).await?;
        to_json_response(&activity)
    }
}
