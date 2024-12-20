use anyhow::{Context, Result};
use axum::{http::StatusCode, response::IntoResponse, Json};
use mcm_client::{get_camera, Camera};
use protocol::display::{
    advanced_display::AdvancedParameterSetting, base_display::BaseParameterSetting,
};
use serde::{Deserialize, Serialize};
use tracing::*;
use ts_rs::TS;
use url::Url;
use uuid::Uuid;
use web_client::send_request;

pub mod protocol;

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
pub struct CameraControl {
    #[ts(as = "String")]
    pub camera_uuid: Uuid,
    #[serde(flatten)]
    pub action: Action,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(tag = "action", content = "json")]
pub enum Action {
    #[serde(rename = "getSysConfig")]
    GetSysConfig,
    #[serde(rename = "getImageAdjustment")]
    GetImageAdjustment,
    #[serde(rename = "setImageAdjustment")]
    SetImageAdjustment(BaseParameterSetting),
    #[serde(rename = "setImageAdjustmentEx")]
    SetImageAdjustmentEx(AdvancedParameterSetting),
}

impl std::fmt::Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

#[instrument(level = "debug")]
pub async fn control(camera_control: Json<CameraControl>) -> impl IntoResponse {
    trace!("Got control query: {camera_control:#?}");

    let url = match get_camera_api_url(&camera_control.0).await {
        Ok(url) => url,
        Err(error) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, format!("{error:?}")).into_response();
        }
    };

    let action_value = serde_json::to_value(&camera_control.action).unwrap();
    let action_map = action_value.as_object().unwrap();
    let payload = action_map
        .get("json")
        .map(|value| value.to_string())
        .unwrap_or_default();

    trace!("URL: {url}, json: {payload}");

    let mut res: serde_json::Value =
        match send_request(reqwest::Client::new().post(url), payload).await {
            Ok(res) => res,
            Err(error) => {
                warn!("res from send_request: {error:#?}");
                return (StatusCode::INTERNAL_SERVER_ERROR, format!("{error:?}")).into_response();
            }
        };

    trace!("Answer from the camera: {res:#?}");

    let map = res.as_object().unwrap();
    if map.contains_key("device_ip") {
        let mut camera_control = camera_control.0.clone();
        camera_control.action = Action::GetImageAdjustment;

        let url = match get_camera_api_url(&camera_control).await {
            Ok(url) => url,
            Err(error) => {
                return (StatusCode::INTERNAL_SERVER_ERROR, format!("{error:?}")).into_response();
            }
        };

        res = match send_request(reqwest::Client::new().get(url), "".to_string()).await {
            Ok(res) => res,
            Err(error) => {
                warn!("res from send_request: {error:#?}");
                return (StatusCode::INTERNAL_SERVER_ERROR, format!("{error:?}")).into_response();
            }
        };

        trace!("Answer from the camera: {res:#?}");
    }

    (StatusCode::OK, res.to_string()).into_response()
}

#[instrument(level = "debug")]
pub async fn list() -> impl IntoResponse {
    let cameras = mcm_client::cameras().await;

    let json = match serde_json::to_string(&cameras) {
        Ok(json) => json,
        Err(error) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, format!("{error:?}")).into_response();
        }
    };

    json.into_response()
}

#[instrument(level = "debug")]
async fn get_camera_api_url(camera_control: &CameraControl) -> Result<Url> {
    let camera_uuid = camera_control.camera_uuid;
    let action_value = serde_json::to_value(&camera_control.action).unwrap();
    let action_map = action_value.as_object().unwrap();
    let action = action_map.get("action").unwrap().as_str().unwrap();

    let Camera {
        hostname,
        credentials,
        ..
    } = get_camera(&camera_uuid).await.context("Camera not found")?;

    let port = 80;

    match credentials {
        // NOTE: This part of the protocol is not working: only the get requests using cgi_action parameter are working.
        // I'm leaving it here in case they fix it:
        // Some(Credentials { username, password }) => {
        //     format!(
        //         "http://{hostname}:{port}/action/cgi_action?user={username}&pwd={password}&action={action}", password=hash_password(&password)
        //     )
        // }
        // None => format!("http://{hostname}:{port}/action/{action}"),
        // So for now we are just using the protocol without the authentication:
        _ => format!("http://{hostname}:{port}/action/{action}"),
    }
    .parse()
    .context("Invalid URl")
}

#[instrument(level = "debug")]
pub fn hash_password(password: &str) -> String {
    use md5::{Digest, Md5};

    let mut hasher = Md5::new();
    hasher.update(password.as_bytes());

    base16ct::lower::encode_string(&hasher.finalize())
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use utils::deserialize;

    use super::CameraControl;

    #[test]
    fn action_serde_test() {
        let payload = json!({
            "camera_uuid": "bc071801-c50f-8301-ac36-bc071801c50f",
            "action": "setImageAdjustmentEx",
            "json": {
                "onceAWB": 1
            }
        })
        .to_string();

        let camera_control = deserialize::<CameraControl>(&payload).unwrap();

        let serialized_action = serde_json::to_string(&camera_control.action).unwrap();

        let expected_action = json!({
            "action": "setImageAdjustmentEx",
            "json": {
                "onceAWB": 1
            }
        })
        .to_string();

        assert_eq!(expected_action, serialized_action);
    }
}
