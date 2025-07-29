pub mod api;
mod manager;
mod mavlink;
pub mod parameters;
pub mod routes;
mod settings_translations;

use anyhow::{Context, Result};
use axum::Json;
use serde::{Deserialize, Serialize};
use tracing::*;

pub use manager::init;

use crate::{
    manager::MANAGER,
    parameters::{ActuatorsParameters, CLOSEST_POINTS, FURTHEST_POINTS},
};

pub use routes::router;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
struct CameraActuators {
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
    use api::Action;

    debug!("Got control query: {actuators_control:#?}");

    let res = match &actuators_control.action {
        Action::ExportLuaScript => {
            let mut manager = MANAGER.get().unwrap().write().await;

            manager.export_script().await?;

            serde_json::to_value({})?
        }
        Action::GetActuatorsState => {
            let (camera_settings_data, gimbal_manager_pitchway_data) = {
                let manager = MANAGER.get().unwrap().read().await;

                let camera_settings_data = manager.mavlink.request_camera_settings().await?;

                let gimbal_manager_pitchway_data =
                    manager.mavlink.request_gimbal_manager_pitchyaw().await?;

                (camera_settings_data, gimbal_manager_pitchway_data)
            };

            let mut manager = MANAGER.get().unwrap().write().await;

            let state = &mut manager
                .settings
                .actuators
                .get_mut(&actuators_control.camera_uuid)
                .context("Camera's actuators not configured")?
                .state;

            state.focus.replace(camera_settings_data.focusLevel);
            state.zoom.replace(camera_settings_data.zoomLevel);
            state.tilt.replace(gimbal_manager_pitchway_data.pitch);

            settings::MANAGER
                .get()
                .unwrap()
                .write()
                .await
                .settings
                .save()
                .await?;

            serde_json::to_value(state)?
        }
        Action::SetActuatorsState(new_state) => {
            let mut manager = MANAGER.get().unwrap().write().await;

            let state = manager
                .update_state(&actuators_control.camera_uuid, new_state)
                .await?;

            serde_json::to_value(state)?
        }
        Action::GetActuatorsConfig => {
            let manager = MANAGER.get().unwrap().read().await;

            let config: &api::ActuatorsConfig = &manager
                .settings
                .actuators
                .get(&actuators_control.camera_uuid)
                .context("Camera's actuators not configured")?
                .into();

            serde_json::to_value(config)?
        }
        Action::SetActuatorsConfig(new_config) => {
            let mut manager = MANAGER.get().unwrap().write().await;

            manager
                .update_config(&actuators_control.camera_uuid, new_config, false)
                .await?;

            let config: &api::ActuatorsConfig = &manager
                .settings
                .actuators
                .get(&actuators_control.camera_uuid)
                .context("Camera's actuators not configured")?
                .into();

            serde_json::to_value(config)?
        }
        Action::ResetActuatorsConfig => {
            let mut manager = MANAGER.get().unwrap().write().await;

            manager.reset_config(&actuators_control.camera_uuid).await?;

            let config: &api::ActuatorsConfig = &manager
                .settings
                .actuators
                .get(&actuators_control.camera_uuid)
                .context("Camera's actuators not configured")?
                .into();

            serde_json::to_value(config)?
        }
    };

    Ok(res)
}
