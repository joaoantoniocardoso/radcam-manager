use axum::{Json, Router, extract::Query, routing::get};
use serde::{Deserialize, Serialize};
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing::*;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct NotificationsQuery {
    pub camera_uuid: Uuid,
}

#[derive(Serialize)]
pub struct NotificationsResponse {
    pub notifications: Vec<autopilot::api::Notification>,
}

#[instrument(level = "trace")]
pub fn router() -> Router {
    Router::new()
        .route("/", get(list))
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive())
}

#[instrument(level = "debug")]
async fn list(Query(query): Query<NotificationsQuery>) -> Json<NotificationsResponse> {
    let notifications = autopilot::notifications(&query.camera_uuid).await;

    Json(NotificationsResponse { notifications })
}
