use axum::response::IntoResponse;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ServerMetadata {
    pub name: &'static str,
    pub description: &'static str,
    pub icon: &'static str,
    pub company: &'static str,
    pub version: &'static str,
    pub new_page: bool,
    pub webpage: &'static str,
    pub api: &'static str,
    pub extras: Extras,
}

#[derive(Debug, Serialize)]
pub struct Extras {
    pub cockpit: &'static str,
}

#[derive(Debug, Serialize)]
pub struct CockpitExtras {
    pub target_system: String,
    pub target_cockpit_api_version: String,
    pub widgets: Vec<CockpitWidget>,
}

#[derive(Debug, Serialize)]
pub struct CockpitWidget {
    pub name: String,
    pub config_iframe_url: Option<String>,
    pub iframe_url: String,
    pub iframe_icon: String,
    pub version: String,
}

impl Default for ServerMetadata {
    fn default() -> Self {
        Self {
            name: "RadCam Manager",
            description: "The official management interface for RadCam",
            icon: "mdi-camera-outline",
            company: "BlueRobotics",
            version: option_env!("CARGO_PKG_VERSION").unwrap_or("0.0.0"),
            new_page: false,
            webpage: "https://github.com/BluerRobotics/radcam-manager",
            api: "/docs",
            extras: Extras {
                cockpit: "/cockpit_extras.json",
            },
        }
    }
}

pub async fn cockpit_extras() -> impl IntoResponse {
    let cameras = mcm_client::cameras().await;

    let version = option_env!("CARGO_PKG_VERSION")
        .unwrap_or("0.0.0")
        .to_string();

    let mut widgets = cameras
        .into_iter()
        .filter_map(|(camera_uuid, camera)| {
            Some(CockpitWidget {
                name: format!("RadCam ({})", camera.hostname),
                config_iframe_url: None,
                iframe_url: format!("/?uuid={camera_uuid}"),
                iframe_icon: "/assets/logo.svg".to_string(),
                version: version.clone(),
            })
        })
        .collect::<Vec<CockpitWidget>>();

    widgets.push(CockpitWidget {
        name: "RadCam (any)".to_string(),
        config_iframe_url: None,
        iframe_url: "".to_string(),
        iframe_icon: "/assets/logo.svg".to_string(),
        version: version.clone(),
    });

    let cockpit_extras = CockpitExtras {
        target_system: "Cockpit".to_string(),
        target_cockpit_api_version: "1.0.0".to_string(),
        widgets,
    };

    let json = serde_json::to_string_pretty(&cockpit_extras).unwrap();

    json.into_response()
}

/// The "register_service" route is used by BlueOS extensions manager
pub async fn server_metadata() -> impl IntoResponse {
    let server_metadata = ServerMetadata::default();

    let json = serde_json::to_string_pretty(&server_metadata).unwrap();

    json.into_response()
}
