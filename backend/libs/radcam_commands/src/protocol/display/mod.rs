use serde::{Deserialize, Serialize};
use serde_repr::*;
use ts_rs::TS;

pub mod advanced_display;
pub mod base_display;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DisplayCommand {
    /// 23.1. Get Base Display Parameter Setting
    GetBase(base_display::BaseParameterSetting),
    /// 23.2. Set Base Display Parameter Setting
    SetBase(base_display::BaseParameterSetting),
    /// 23.3. Get Advanced Display Parameter Setting
    GetAdvanced(advanced_display::AdvancedParameterSetting),
    /// 23.4. Set Advanced Display Parameter Setting
    SetAdvanced(advanced_display::AdvancedParameterSetting),
    /// 23.5. Base Display Parameter Restore
    RestoreBase,
    /// 23.6. Advanced Display Parameter Restore
    RestoreAdvanced,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize_repr, Deserialize_repr, TS)]
#[repr(u16)]
pub enum MaxExposureValue {
    #[default]
    _12 = 12,
    _25 = 25,
    _30 = 30,
    _50 = 50,
    _60 = 60,
    _100 = 100,
    _200 = 200,
    _400 = 400,
    _800 = 800,
    _1000 = 1000,
    _2000 = 2000,
    _4000 = 4000,
    _8000 = 8000,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize_repr, Deserialize_repr, TS)]
#[repr(u8)]
pub enum AntiFogValue {
    #[default]
    Open = 0,
    Close = 1,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize_repr, Deserialize_repr, TS)]
#[repr(u8)]
pub enum FrameTurboValue {
    #[default]
    Open = 0,
    HighFrameRates = 1,
    UltraHighFrameRates = 2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize_repr, Deserialize_repr, TS)]
#[repr(u8)]
pub enum SceneModeValue {
    #[default]
    FaceCapture = 1,
    LicensePlateCapture = 2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize_repr, Deserialize_repr, TS)]
#[repr(u8)]
pub enum AeStrategyModeValue {
    #[default]
    HighLightPriority = 0,
    LowLightPriority = 1,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize_repr, Deserialize_repr, TS)]
#[repr(u8)]
pub enum AutoExposureExValue {
    #[default]
    Auto = 0,
    Manual = 1,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize_repr, Deserialize_repr, TS)]
#[repr(u16)]
pub enum ExposuretimeValue {
    #[default]
    _12 = 12,
    _25 = 25,
    _30 = 30,
    _50 = 50,
    _60 = 60,
    _100 = 100,
    _200 = 200,
    _400 = 400,
    _800 = 800,
    _1000 = 1000,
    _2000 = 2000,
    _4000 = 4000,
    _8000 = 8000,
    _10000 = 10000,
    _34464 = 34464,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize_repr, Deserialize_repr, TS)]
#[repr(u8)]
pub enum AutoAWB {
    #[default]
    Auto = 0,
    Manual = 1,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize_repr, Deserialize_repr, TS)]
#[repr(u8)]
pub enum AwbAutoModeValue {
    #[default]
    Scene0 = 0,
    Scene1 = 1,
    Scene2 = 2,
    Scene3 = 3,
    Scene4 = 4,
    Scene5 = 5,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize_repr, Deserialize_repr, TS)]
#[repr(u8)]
pub enum AutoGainModeValue {
    #[default]
    Auto = 0,
    Manual = 1,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize_repr, Deserialize_repr, TS)]
#[repr(u8)]
pub enum ManualAGainEnableValue {
    #[default]
    Open = 0,
    Close = 1,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize_repr, Deserialize_repr, TS)]
#[repr(u8)]
pub enum ManualDGainEnableValue {
    #[default]
    Open = 0,
    Close = 1,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize_repr, Deserialize_repr, TS)]
#[repr(u8)]
pub enum RotateValue {
    #[default]
    _0deg = 0,
    _90deg = 1,
    _180deg = 2,
    _270deg = 3,
}
