use axum::{Json, Router, response::IntoResponse, routing::post};
use reqwest::StatusCode;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing::*;

use crate::{api, control_inner};

#[instrument(level = "trace")]
pub fn router() -> Router {
    Router::new()
        .route("/control", post(control))
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
}

#[instrument(level = "debug")]
pub async fn control(actuators_control: Json<api::ActuatorsControl>) -> impl IntoResponse {
    let res = match control_inner(actuators_control).await {
        Ok(res) => res,
        Err(error) => {
            warn!("Failed: {error:#?}");
            return (StatusCode::INTERNAL_SERVER_ERROR, format!("{error:?}")).into_response();
        }
    };

    (StatusCode::OK, res.to_string()).into_response()
}
