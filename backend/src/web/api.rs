use http_types::StatusCode;
use serde::Serialize;

pub type ApiRequest<T> = tide::Request<T>;
//todo: stop erasing return type of functions so that we can return something meaningful (like Vec<Activity>)
pub type ApiResult = tide::Result<tide::Response>;

pub fn to_json_response<T: Serialize + std::fmt::Debug>(entitiy: T) -> ApiResult {
    tide::Response::new(StatusCode::Ok)
        .body_json(&entitiy)
        .map_err(|e| tide::Error::new(StatusCode::InternalServerError, e))
}

pub mod activities {
    use crate::db::{NewActivity, RepoFactory};
    use crate::web::api::{to_json_response, ApiRequest, ApiResult};

    pub async fn list(req: ApiRequest<impl RepoFactory>) -> ApiResult {
        let activities = req.state().activities().fetch().await?;
        to_json_response(activities)
    }

    pub async fn create(mut req: ApiRequest<impl RepoFactory>) -> ApiResult {
        let new: NewActivity = req.body_json().await?;
        let activity = req.state().activities().create(new, None).await?;
        to_json_response(&activity)
    }
}

pub mod activity_events {
    use crate::db::RepoFactory;
    use crate::web::api::{to_json_response, ApiRequest, ApiResult};

    pub async fn list(req: ApiRequest<impl RepoFactory>) -> ApiResult {
        let activities = req.state().activity_events().fetch().await?;
        to_json_response(activities)
    }
}
