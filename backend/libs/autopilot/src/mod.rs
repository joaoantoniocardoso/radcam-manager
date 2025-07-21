pub mod api;
mod mavlink;
pub mod parameters;
pub mod routes;

use anyhow::Result;
use axum::Json;
use serde::{Deserialize, Serialize};
use tracing::*;

use crate::parameters::{ActuatorsParameters, CLOSEST_POINTS, FURTHEST_POINTS};

pub use routes::router;
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub parameters: ActuatorsParameters,
    pub closest_points: api::FocusZoomPoints,
    pub furthest_points: api::FocusZoomPoints,
    pub state: api::ActuatorsState,
}

impl Default for CameraActuators {
    fn default() -> Self {
        Self {
            parameters: ActuatorsParameters::default(),
            closest_points: api::FocusZoomPoints(CLOSEST_POINTS.to_vec()),
            furthest_points: api::FocusZoomPoints(FURTHEST_POINTS.to_vec()),
            state: api::ActuatorsState::default(),
        }
    }
}

#[instrument(level = "debug")]
pub(crate) async fn control_inner(
    actuators_control: Json<api::ActuatorsControl>,
) -> Result<serde_json::Value> {
    debug!("Got control query: {actuators_control:#?}");

    unimplemented!()
}
