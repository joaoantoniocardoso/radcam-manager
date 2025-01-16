use std::net::Ipv4Addr;

use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
pub struct Info {
    /// Name of the program
    pub name: String,
    /// Version/tag
    pub version: String,
    /// Git SHA
    pub sha: String,
    pub build_date: String,
    /// Authors name
    pub authors: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
pub struct ApiVideoSource {
    pub name: String,
    pub source: String,
    pub formats: Vec<Format>,
    // pub controls: Vec<Control>, # We don't need this
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Format {
    pub encode: VideoEncodeType,
    pub sizes: Vec<Size>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum VideoEncodeType {
    H264,
    H265,
    Mjpg,
    Rgb,
    Unknown(String),
    Yuyv,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Size {
    pub width: u32,
    pub height: u32,
    pub intervals: Vec<FrameInterval>,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct FrameInterval {
    pub numerator: u32,
    pub denominator: u32,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct StreamInformation {
    pub endpoints: Vec<Url>,
    pub configuration: CaptureConfiguration,
    pub extended_configuration: Option<ExtendedConfiguration>,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum CaptureConfiguration {
    Video(VideoCaptureConfiguration),
    Redirect(serde_json::Value),
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct ExtendedConfiguration {
    pub thermal: bool,
    pub disable_mavlink: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct VideoCaptureConfiguration {
    pub encode: VideoEncodeType,
    pub height: u32,
    pub width: u32,
    pub frame_interval: FrameInterval,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct StreamStatus {
    pub id: uuid::Uuid,
    pub running: bool,
    pub video_and_stream: VideoAndStreamInformation,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct VideoAndStreamInformation {
    pub name: String,
    pub stream_information: StreamInformation,
    pub video_source: VideoSourceType,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub enum VideoSourceType {
    Onvif(VideoSourceOnvif),
    // NOTE: we are using Value below as a workaround to skip the need to define all the MCM types we don't use here.
    Gst(serde_json::Value),
    Local(serde_json::Value),
    Redirect(serde_json::Value),
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct VideoSourceOnvif {
    pub name: String,
    pub source: VideoSourceOnvifType,
    #[serde(flatten)]
    pub device_information: OnvifDeviceInformation,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum VideoSourceOnvifType {
    Onvif(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct OnvifDeviceInformation {
    pub manufacturer: String,
    pub model: String,
    pub firmware_version: String,
    pub serial_number: String,
    pub hardware_id: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct PostStream {
    pub name: String,
    pub source: String,
    pub stream_information: StreamInformation,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct RemoveStream {
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct OnvifDevice {
    pub uuid: uuid::Uuid,
    pub ip: Ipv4Addr,
    pub types: Vec<String>,
    pub hardware: Option<String>,
    pub name: Option<String>,
    pub urls: Vec<Url>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct AuthenticateOnvifDeviceRequest {
    /// Onvif Device UUID, obtained via `/onvif/devices` get request
    pub device_uuid: uuid::Uuid,
    /// Username for the Onvif Device
    pub username: String,
    /// Password for the Onvif Device
    pub password: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct UnauthenticateOnvifDeviceRequest {
    /// Onvif Device UUID, obtained via `/onvif/devices` get request
    pub device_uuid: uuid::Uuid,
}
