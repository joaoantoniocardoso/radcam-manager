use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use ts_rs::TS;

use super::*;

#[skip_serializing_none]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
pub struct BaseParameterSetting {
    /// Tonal. Range: [0..=255]
    pub hue: Option<u8>,
    /// Brightness. Range: [0..=255]
    pub brightness: Option<u8>,
    /// Sharpness. Range: [0..=255]
    pub sharpness: Option<u8>,
    /// Contrast. Range: [0..=255]
    pub contrast: Option<u8>,
    /// Saturation. Range: [0..=255]
    pub saturation: Option<u8>,
    /// Gamma. Range: [0..=255]
    pub gamma: Option<u8>,
    /// Exposure Time.
    pub max_exposure: Option<MaxExposureValue>,
    /// Restores all BaseParameterSetting parameters when `1`
    pub set_default: Option<u8>,
}

#[cfg(test)]
mod tests {

    use serde_json::json;

    use utils::deserialize;

    use super::*;

    #[test]
    fn setting_some_parameters() {
        let json = json!({
            "hue": 128,
            "brightness": 128,
            "sharpness": 128,
            "contrast": 128,
            "saturation": 128,
            "gamma": 137,
        })
        .to_string();

        let params = deserialize::<BaseParameterSetting>(&json).expect("Failed deserializing");

        let expected_params = BaseParameterSetting {
            hue: Some(128),
            brightness: Some(128),
            sharpness: Some(128),
            contrast: Some(128),
            saturation: Some(128),
            gamma: Some(137),
            ..Default::default()
        };

        assert_eq!(expected_params, params);
    }
}
