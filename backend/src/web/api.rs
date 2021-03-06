use serde::Serialize;

pub type ApiRequest<T> = tide::Request<T>;
//todo: stop erasing return type of functions so that we can return something meaningful (like Vec<Activity>)
pub type ApiResult = tide::Result<tide::Body>;

pub fn to_json_response<T: Serialize + std::fmt::Debug>(entitiy: T) -> ApiResult {
    tide::Body::from_json(&entitiy)
}

pub mod activities {
    use crate::db::{NewActivity, RepoFactory};
    use crate::web::api::{to_json_response, ApiRequest, ApiResult};

    pub async fn list(req: ApiRequest<impl RepoFactory>) -> ApiResult {
        let pool = req.state();
        let user = pool.users().find(1).await?;
        let activities = pool.activities().fetch_for(&user).await?;
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
