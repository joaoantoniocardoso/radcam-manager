use axum::{
    routing::{get, post},
    Router,
};
use radcam_commands;
use tracing::*;

#[instrument(level = "trace")]
pub fn router() -> Router {
    Router::new()
        .route("/list", get(radcam_commands::list))
        .route("/control", post(radcam_commands::control))
}
