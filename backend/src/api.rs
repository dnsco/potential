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
    let mut acts = app.at("/activities");
    acts.get(activities::list).post(activities::create);
    acts.at("/import").get(activities::import);
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
    use std::env;

    use crate::api::util::{to_json_response, ApiRequest, ApiResult};
    use crate::api::AsStdError;
    use crate::db::{DbImport, NewActivity, Repo};

    pub async fn import(req: ApiRequest) -> ApiResult {
        let repo = Repo { pool: req.state() };
        let strength_url = env::var("STRENGTH_URL")?;

        DbImport::from(&repo, strength_url)?
            .run()
            .await
            .map_err(AsStdError)?;

        to_json_response(())
    }

    pub async fn list(req: ApiRequest) -> ApiResult {
        let repo = Repo { pool: req.state() };
        let activities = repo.fetch_activities().await?;
        to_json_response(activities)
    }

    pub async fn create(mut req: ApiRequest) -> ApiResult {
        let new: NewActivity = req.body_json().await?;
        let repo = Repo { pool: req.state() };
        let activity = repo.create_activity(new, None).await?;
        to_json_response(&activity)
    }
}

#[derive(thiserror::Error, Debug)]
#[error(transparent)]
pub struct AsStdError(#[from] anyhow::Error);
