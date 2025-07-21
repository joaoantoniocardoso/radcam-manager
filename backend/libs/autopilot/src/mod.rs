pub mod api;
mod mavlink;
pub mod parameters;
pub mod routes;

use anyhow::Result;
use axum::Json;
use tracing::*;

pub use routes::router;

#[instrument(level = "debug")]
pub(crate) async fn control_inner(
    actuators_control: Json<api::ActuatorsControl>,
) -> Result<serde_json::Value> {
    debug!("Got control query: {actuators_control:#?}");

    unimplemented!()
}
