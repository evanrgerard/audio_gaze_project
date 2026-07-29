#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};



// Corresponds to op3_camera_setting_tool_msgs__msg__CameraParams
/// This represents the parameters of usb camera

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CameraParams {
    /// 0 - 255, step = 1, default: 128
    pub brightness: u32,

    /// 0 - 255, step = 1, default: 128
    pub contrast: u32,

    /// 0 - 255, step = 1, default: 128
    pub saturation: u32,

    /// 0 - 255, step = 1, default: 128
    pub sharpness: u32,

    /// 0 - 255, step = 1, default: 0
    pub gain: u32,

    /// default: true
    pub focus_automatic_continuous: bool,

    /// 0 - 250, step = 5, default: 0
    pub focus_absolute: i32,

    /// 0 - 3, default: 3
    pub auto_exposure: u32,

    /// 3 - 2047, step = 1, default: 250
    pub exposure_time_absolute: u32,

    /// default: true
    pub white_balance_automatic: bool,

    /// 2000 - 6500, step = 1, default: 4000
    pub white_balance_temperature: u32,

}



impl Default for CameraParams {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::CameraParams::default())
  }
}

impl rosidl_runtime_rs::Message for CameraParams {
  type RmwMsg = super::msg::rmw::CameraParams;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        brightness: msg.brightness,
        contrast: msg.contrast,
        saturation: msg.saturation,
        sharpness: msg.sharpness,
        gain: msg.gain,
        focus_automatic_continuous: msg.focus_automatic_continuous,
        focus_absolute: msg.focus_absolute,
        auto_exposure: msg.auto_exposure,
        exposure_time_absolute: msg.exposure_time_absolute,
        white_balance_automatic: msg.white_balance_automatic,
        white_balance_temperature: msg.white_balance_temperature,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      brightness: msg.brightness,
      contrast: msg.contrast,
      saturation: msg.saturation,
      sharpness: msg.sharpness,
      gain: msg.gain,
      focus_automatic_continuous: msg.focus_automatic_continuous,
      focus_absolute: msg.focus_absolute,
      auto_exposure: msg.auto_exposure,
      exposure_time_absolute: msg.exposure_time_absolute,
      white_balance_automatic: msg.white_balance_automatic,
      white_balance_temperature: msg.white_balance_temperature,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      brightness: msg.brightness,
      contrast: msg.contrast,
      saturation: msg.saturation,
      sharpness: msg.sharpness,
      gain: msg.gain,
      focus_automatic_continuous: msg.focus_automatic_continuous,
      focus_absolute: msg.focus_absolute,
      auto_exposure: msg.auto_exposure,
      exposure_time_absolute: msg.exposure_time_absolute,
      white_balance_automatic: msg.white_balance_automatic,
      white_balance_temperature: msg.white_balance_temperature,
    }
  }
}


// Corresponds to op3_camera_setting_tool_msgs__msg__V4lParameter

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct V4lParameter {

    // This member is not documented.
    #[allow(missing_docs)]
    pub name: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub value: i32,

}



impl Default for V4lParameter {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::V4lParameter::default())
  }
}

impl rosidl_runtime_rs::Message for V4lParameter {
  type RmwMsg = super::msg::rmw::V4lParameter;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        name: msg.name.as_str().into(),
        value: msg.value,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        name: msg.name.as_str().into(),
      value: msg.value,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      name: msg.name.to_string(),
      value: msg.value,
    }
  }
}


// Corresponds to op3_camera_setting_tool_msgs__msg__V4lParameters

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct V4lParameters {

    // This member is not documented.
    #[allow(missing_docs)]
    pub video_parameter: Vec<super::msg::V4lParameter>,

}



impl Default for V4lParameters {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::V4lParameters::default())
  }
}

impl rosidl_runtime_rs::Message for V4lParameters {
  type RmwMsg = super::msg::rmw::V4lParameters;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        video_parameter: msg.video_parameter
          .into_iter()
          .map(|elem| super::msg::V4lParameter::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        video_parameter: msg.video_parameter
          .iter()
          .map(|elem| super::msg::V4lParameter::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      video_parameter: msg.video_parameter
          .into_iter()
          .map(super::msg::V4lParameter::from_rmw_message)
          .collect(),
    }
  }
}


