use axum::{Json, Router, response::IntoResponse, routing::post};
use reqwest::StatusCode;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing::*;

use crate::{ActuatorsControl, control_inner};

pub fn router() -> Router {
    Router::new()
        .route("/", post(control))
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
}

pub async fn control(actuators_control: Json<ActuatorsControl>) -> impl IntoResponse {
    // let res = match control_inner(actuators_control).await {
    //     Ok(res) => res,
    //     Err(error) => {
    //         warn!("res from send_request: {error:#?}");
    //         return (StatusCode::INTERNAL_SERVER_ERROR, format!("{error:?}")).into_response();
    //     }
    // };
    let res: serde_json::Value = todo!();

    (StatusCode::OK, res.to_string()).into_response()
}
