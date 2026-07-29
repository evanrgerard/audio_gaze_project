#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};


#[link(name = "op3_camera_setting_tool_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__op3_camera_setting_tool_msgs__msg__CameraParams() -> *const std::ffi::c_void;
}

#[link(name = "op3_camera_setting_tool_msgs__rosidl_generator_c")]
extern "C" {
    fn op3_camera_setting_tool_msgs__msg__CameraParams__init(msg: *mut CameraParams) -> bool;
    fn op3_camera_setting_tool_msgs__msg__CameraParams__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<CameraParams>, size: usize) -> bool;
    fn op3_camera_setting_tool_msgs__msg__CameraParams__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<CameraParams>);
    fn op3_camera_setting_tool_msgs__msg__CameraParams__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<CameraParams>, out_seq: *mut rosidl_runtime_rs::Sequence<CameraParams>) -> bool;
}

// Corresponds to op3_camera_setting_tool_msgs__msg__CameraParams
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// This represents the parameters of usb camera

#[repr(C)]
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
    unsafe {
      let mut msg = std::mem::zeroed();
      if !op3_camera_setting_tool_msgs__msg__CameraParams__init(&mut msg as *mut _) {
        panic!("Call to op3_camera_setting_tool_msgs__msg__CameraParams__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for CameraParams {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { op3_camera_setting_tool_msgs__msg__CameraParams__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { op3_camera_setting_tool_msgs__msg__CameraParams__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { op3_camera_setting_tool_msgs__msg__CameraParams__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for CameraParams {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for CameraParams where Self: Sized {
  const TYPE_NAME: &'static str = "op3_camera_setting_tool_msgs/msg/CameraParams";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__op3_camera_setting_tool_msgs__msg__CameraParams() }
  }
}


#[link(name = "op3_camera_setting_tool_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__op3_camera_setting_tool_msgs__msg__V4lParameter() -> *const std::ffi::c_void;
}

#[link(name = "op3_camera_setting_tool_msgs__rosidl_generator_c")]
extern "C" {
    fn op3_camera_setting_tool_msgs__msg__V4lParameter__init(msg: *mut V4lParameter) -> bool;
    fn op3_camera_setting_tool_msgs__msg__V4lParameter__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<V4lParameter>, size: usize) -> bool;
    fn op3_camera_setting_tool_msgs__msg__V4lParameter__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<V4lParameter>);
    fn op3_camera_setting_tool_msgs__msg__V4lParameter__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<V4lParameter>, out_seq: *mut rosidl_runtime_rs::Sequence<V4lParameter>) -> bool;
}

// Corresponds to op3_camera_setting_tool_msgs__msg__V4lParameter
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct V4lParameter {

    // This member is not documented.
    #[allow(missing_docs)]
    pub name: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub value: i32,

}



impl Default for V4lParameter {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !op3_camera_setting_tool_msgs__msg__V4lParameter__init(&mut msg as *mut _) {
        panic!("Call to op3_camera_setting_tool_msgs__msg__V4lParameter__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for V4lParameter {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { op3_camera_setting_tool_msgs__msg__V4lParameter__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { op3_camera_setting_tool_msgs__msg__V4lParameter__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { op3_camera_setting_tool_msgs__msg__V4lParameter__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for V4lParameter {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for V4lParameter where Self: Sized {
  const TYPE_NAME: &'static str = "op3_camera_setting_tool_msgs/msg/V4lParameter";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__op3_camera_setting_tool_msgs__msg__V4lParameter() }
  }
}


#[link(name = "op3_camera_setting_tool_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__op3_camera_setting_tool_msgs__msg__V4lParameters() -> *const std::ffi::c_void;
}

#[link(name = "op3_camera_setting_tool_msgs__rosidl_generator_c")]
extern "C" {
    fn op3_camera_setting_tool_msgs__msg__V4lParameters__init(msg: *mut V4lParameters) -> bool;
    fn op3_camera_setting_tool_msgs__msg__V4lParameters__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<V4lParameters>, size: usize) -> bool;
    fn op3_camera_setting_tool_msgs__msg__V4lParameters__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<V4lParameters>);
    fn op3_camera_setting_tool_msgs__msg__V4lParameters__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<V4lParameters>, out_seq: *mut rosidl_runtime_rs::Sequence<V4lParameters>) -> bool;
}

// Corresponds to op3_camera_setting_tool_msgs__msg__V4lParameters
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct V4lParameters {

    // This member is not documented.
    #[allow(missing_docs)]
    pub video_parameter: rosidl_runtime_rs::Sequence<super::super::msg::rmw::V4lParameter>,

}



impl Default for V4lParameters {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !op3_camera_setting_tool_msgs__msg__V4lParameters__init(&mut msg as *mut _) {
        panic!("Call to op3_camera_setting_tool_msgs__msg__V4lParameters__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for V4lParameters {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { op3_camera_setting_tool_msgs__msg__V4lParameters__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { op3_camera_setting_tool_msgs__msg__V4lParameters__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { op3_camera_setting_tool_msgs__msg__V4lParameters__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for V4lParameters {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for V4lParameters where Self: Sized {
  const TYPE_NAME: &'static str = "op3_camera_setting_tool_msgs/msg/V4lParameters";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__op3_camera_setting_tool_msgs__msg__V4lParameters() }
  }
}


