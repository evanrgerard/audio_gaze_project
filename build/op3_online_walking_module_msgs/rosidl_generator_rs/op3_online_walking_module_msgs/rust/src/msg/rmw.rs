#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};


#[link(name = "op3_online_walking_module_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__op3_online_walking_module_msgs__msg__JointPose() -> *const std::ffi::c_void;
}

#[link(name = "op3_online_walking_module_msgs__rosidl_generator_c")]
extern "C" {
    fn op3_online_walking_module_msgs__msg__JointPose__init(msg: *mut JointPose) -> bool;
    fn op3_online_walking_module_msgs__msg__JointPose__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<JointPose>, size: usize) -> bool;
    fn op3_online_walking_module_msgs__msg__JointPose__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<JointPose>);
    fn op3_online_walking_module_msgs__msg__JointPose__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<JointPose>, out_seq: *mut rosidl_runtime_rs::Sequence<JointPose>) -> bool;
}

// Corresponds to op3_online_walking_module_msgs__msg__JointPose
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct JointPose {

    // This member is not documented.
    #[allow(missing_docs)]
    pub mov_time: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub pose: sensor_msgs::msg::rmw::JointState,

}



impl Default for JointPose {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !op3_online_walking_module_msgs__msg__JointPose__init(&mut msg as *mut _) {
        panic!("Call to op3_online_walking_module_msgs__msg__JointPose__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for JointPose {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { op3_online_walking_module_msgs__msg__JointPose__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { op3_online_walking_module_msgs__msg__JointPose__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { op3_online_walking_module_msgs__msg__JointPose__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for JointPose {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for JointPose where Self: Sized {
  const TYPE_NAME: &'static str = "op3_online_walking_module_msgs/msg/JointPose";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__op3_online_walking_module_msgs__msg__JointPose() }
  }
}


#[link(name = "op3_online_walking_module_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__op3_online_walking_module_msgs__msg__KinematicsPose() -> *const std::ffi::c_void;
}

#[link(name = "op3_online_walking_module_msgs__rosidl_generator_c")]
extern "C" {
    fn op3_online_walking_module_msgs__msg__KinematicsPose__init(msg: *mut KinematicsPose) -> bool;
    fn op3_online_walking_module_msgs__msg__KinematicsPose__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<KinematicsPose>, size: usize) -> bool;
    fn op3_online_walking_module_msgs__msg__KinematicsPose__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<KinematicsPose>);
    fn op3_online_walking_module_msgs__msg__KinematicsPose__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<KinematicsPose>, out_seq: *mut rosidl_runtime_rs::Sequence<KinematicsPose>) -> bool;
}

// Corresponds to op3_online_walking_module_msgs__msg__KinematicsPose
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct KinematicsPose {

    // This member is not documented.
    #[allow(missing_docs)]
    pub name: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub mov_time: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub pose: geometry_msgs::msg::rmw::Pose,

}



impl Default for KinematicsPose {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !op3_online_walking_module_msgs__msg__KinematicsPose__init(&mut msg as *mut _) {
        panic!("Call to op3_online_walking_module_msgs__msg__KinematicsPose__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for KinematicsPose {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { op3_online_walking_module_msgs__msg__KinematicsPose__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { op3_online_walking_module_msgs__msg__KinematicsPose__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { op3_online_walking_module_msgs__msg__KinematicsPose__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for KinematicsPose {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for KinematicsPose where Self: Sized {
  const TYPE_NAME: &'static str = "op3_online_walking_module_msgs/msg/KinematicsPose";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__op3_online_walking_module_msgs__msg__KinematicsPose() }
  }
}


#[link(name = "op3_online_walking_module_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__op3_online_walking_module_msgs__msg__FootStepCommand() -> *const std::ffi::c_void;
}

#[link(name = "op3_online_walking_module_msgs__rosidl_generator_c")]
extern "C" {
    fn op3_online_walking_module_msgs__msg__FootStepCommand__init(msg: *mut FootStepCommand) -> bool;
    fn op3_online_walking_module_msgs__msg__FootStepCommand__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<FootStepCommand>, size: usize) -> bool;
    fn op3_online_walking_module_msgs__msg__FootStepCommand__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<FootStepCommand>);
    fn op3_online_walking_module_msgs__msg__FootStepCommand__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<FootStepCommand>, out_seq: *mut rosidl_runtime_rs::Sequence<FootStepCommand>) -> bool;
}

// Corresponds to op3_online_walking_module_msgs__msg__FootStepCommand
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct FootStepCommand {

    // This member is not documented.
    #[allow(missing_docs)]
    pub command: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub start_leg: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub step_num: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub step_time: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub step_length: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub side_length: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub step_angle: f64,

}



impl Default for FootStepCommand {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !op3_online_walking_module_msgs__msg__FootStepCommand__init(&mut msg as *mut _) {
        panic!("Call to op3_online_walking_module_msgs__msg__FootStepCommand__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for FootStepCommand {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { op3_online_walking_module_msgs__msg__FootStepCommand__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { op3_online_walking_module_msgs__msg__FootStepCommand__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { op3_online_walking_module_msgs__msg__FootStepCommand__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for FootStepCommand {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for FootStepCommand where Self: Sized {
  const TYPE_NAME: &'static str = "op3_online_walking_module_msgs/msg/FootStepCommand";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__op3_online_walking_module_msgs__msg__FootStepCommand() }
  }
}


#[link(name = "op3_online_walking_module_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__op3_online_walking_module_msgs__msg__FootStepArray() -> *const std::ffi::c_void;
}

#[link(name = "op3_online_walking_module_msgs__rosidl_generator_c")]
extern "C" {
    fn op3_online_walking_module_msgs__msg__FootStepArray__init(msg: *mut FootStepArray) -> bool;
    fn op3_online_walking_module_msgs__msg__FootStepArray__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<FootStepArray>, size: usize) -> bool;
    fn op3_online_walking_module_msgs__msg__FootStepArray__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<FootStepArray>);
    fn op3_online_walking_module_msgs__msg__FootStepArray__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<FootStepArray>, out_seq: *mut rosidl_runtime_rs::Sequence<FootStepArray>) -> bool;
}

// Corresponds to op3_online_walking_module_msgs__msg__FootStepArray
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct FootStepArray {

    // This member is not documented.
    #[allow(missing_docs)]
    pub moving_foot: rosidl_runtime_rs::Sequence<i32>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub data: rosidl_runtime_rs::Sequence<geometry_msgs::msg::rmw::Pose2D>,

}

impl FootStepArray {
    /// Left foot constant
    pub const LEFT_FOOT: i32 = 0;

    /// Right foot constant
    pub const RIGHT_FOOT: i32 = 1;

}


impl Default for FootStepArray {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !op3_online_walking_module_msgs__msg__FootStepArray__init(&mut msg as *mut _) {
        panic!("Call to op3_online_walking_module_msgs__msg__FootStepArray__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for FootStepArray {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { op3_online_walking_module_msgs__msg__FootStepArray__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { op3_online_walking_module_msgs__msg__FootStepArray__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { op3_online_walking_module_msgs__msg__FootStepArray__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for FootStepArray {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for FootStepArray where Self: Sized {
  const TYPE_NAME: &'static str = "op3_online_walking_module_msgs/msg/FootStepArray";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__op3_online_walking_module_msgs__msg__FootStepArray() }
  }
}


#[link(name = "op3_online_walking_module_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__op3_online_walking_module_msgs__msg__PreviewRequest() -> *const std::ffi::c_void;
}

#[link(name = "op3_online_walking_module_msgs__rosidl_generator_c")]
extern "C" {
    fn op3_online_walking_module_msgs__msg__PreviewRequest__init(msg: *mut PreviewRequest) -> bool;
    fn op3_online_walking_module_msgs__msg__PreviewRequest__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<PreviewRequest>, size: usize) -> bool;
    fn op3_online_walking_module_msgs__msg__PreviewRequest__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<PreviewRequest>);
    fn op3_online_walking_module_msgs__msg__PreviewRequest__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<PreviewRequest>, out_seq: *mut rosidl_runtime_rs::Sequence<PreviewRequest>) -> bool;
}

// Corresponds to op3_online_walking_module_msgs__msg__PreviewRequest
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PreviewRequest {

    // This member is not documented.
    #[allow(missing_docs)]
    pub control_cycle: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub lipm_height: f64,

}



impl Default for PreviewRequest {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !op3_online_walking_module_msgs__msg__PreviewRequest__init(&mut msg as *mut _) {
        panic!("Call to op3_online_walking_module_msgs__msg__PreviewRequest__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for PreviewRequest {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { op3_online_walking_module_msgs__msg__PreviewRequest__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { op3_online_walking_module_msgs__msg__PreviewRequest__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { op3_online_walking_module_msgs__msg__PreviewRequest__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for PreviewRequest {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for PreviewRequest where Self: Sized {
  const TYPE_NAME: &'static str = "op3_online_walking_module_msgs/msg/PreviewRequest";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__op3_online_walking_module_msgs__msg__PreviewRequest() }
  }
}


#[link(name = "op3_online_walking_module_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__op3_online_walking_module_msgs__msg__PreviewResponse() -> *const std::ffi::c_void;
}

#[link(name = "op3_online_walking_module_msgs__rosidl_generator_c")]
extern "C" {
    fn op3_online_walking_module_msgs__msg__PreviewResponse__init(msg: *mut PreviewResponse) -> bool;
    fn op3_online_walking_module_msgs__msg__PreviewResponse__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<PreviewResponse>, size: usize) -> bool;
    fn op3_online_walking_module_msgs__msg__PreviewResponse__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<PreviewResponse>);
    fn op3_online_walking_module_msgs__msg__PreviewResponse__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<PreviewResponse>, out_seq: *mut rosidl_runtime_rs::Sequence<PreviewResponse>) -> bool;
}

// Corresponds to op3_online_walking_module_msgs__msg__PreviewResponse
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PreviewResponse {

    // This member is not documented.
    #[allow(missing_docs)]
    pub k_row: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub k_col: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub k: rosidl_runtime_rs::Sequence<f64>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub p_row: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub p_col: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub p: rosidl_runtime_rs::Sequence<f64>,

}



impl Default for PreviewResponse {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !op3_online_walking_module_msgs__msg__PreviewResponse__init(&mut msg as *mut _) {
        panic!("Call to op3_online_walking_module_msgs__msg__PreviewResponse__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for PreviewResponse {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { op3_online_walking_module_msgs__msg__PreviewResponse__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { op3_online_walking_module_msgs__msg__PreviewResponse__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { op3_online_walking_module_msgs__msg__PreviewResponse__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for PreviewResponse {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for PreviewResponse where Self: Sized {
  const TYPE_NAME: &'static str = "op3_online_walking_module_msgs/msg/PreviewResponse";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__op3_online_walking_module_msgs__msg__PreviewResponse() }
  }
}


#[link(name = "op3_online_walking_module_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__op3_online_walking_module_msgs__msg__WalkingParam() -> *const std::ffi::c_void;
}

#[link(name = "op3_online_walking_module_msgs__rosidl_generator_c")]
extern "C" {
    fn op3_online_walking_module_msgs__msg__WalkingParam__init(msg: *mut WalkingParam) -> bool;
    fn op3_online_walking_module_msgs__msg__WalkingParam__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<WalkingParam>, size: usize) -> bool;
    fn op3_online_walking_module_msgs__msg__WalkingParam__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<WalkingParam>);
    fn op3_online_walking_module_msgs__msg__WalkingParam__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<WalkingParam>, out_seq: *mut rosidl_runtime_rs::Sequence<WalkingParam>) -> bool;
}

// Corresponds to op3_online_walking_module_msgs__msg__WalkingParam
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct WalkingParam {

    // This member is not documented.
    #[allow(missing_docs)]
    pub dsp_ratio: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub lipm_height: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub foot_height_max: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub zmp_offset_x: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub zmp_offset_y: f64,

}



impl Default for WalkingParam {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !op3_online_walking_module_msgs__msg__WalkingParam__init(&mut msg as *mut _) {
        panic!("Call to op3_online_walking_module_msgs__msg__WalkingParam__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for WalkingParam {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { op3_online_walking_module_msgs__msg__WalkingParam__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { op3_online_walking_module_msgs__msg__WalkingParam__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { op3_online_walking_module_msgs__msg__WalkingParam__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for WalkingParam {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for WalkingParam where Self: Sized {
  const TYPE_NAME: &'static str = "op3_online_walking_module_msgs/msg/WalkingParam";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__op3_online_walking_module_msgs__msg__WalkingParam() }
  }
}


#[link(name = "op3_online_walking_module_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__op3_online_walking_module_msgs__msg__Step2D() -> *const std::ffi::c_void;
}

#[link(name = "op3_online_walking_module_msgs__rosidl_generator_c")]
extern "C" {
    fn op3_online_walking_module_msgs__msg__Step2D__init(msg: *mut Step2D) -> bool;
    fn op3_online_walking_module_msgs__msg__Step2D__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Step2D>, size: usize) -> bool;
    fn op3_online_walking_module_msgs__msg__Step2D__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Step2D>);
    fn op3_online_walking_module_msgs__msg__Step2D__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Step2D>, out_seq: *mut rosidl_runtime_rs::Sequence<Step2D>) -> bool;
}

// Corresponds to op3_online_walking_module_msgs__msg__Step2D
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// 2D StepData

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Step2D {
    /// step pose as relative offset to last leg
    pub step2d: geometry_msgs::msg::rmw::Pose2D,

    /// which leg to be used (left/right/no, see below)
    pub moving_foot: u8,

}

impl Step2D {
    /// Left foot constant
    pub const LEFT_FOOT_SWING: u8 = 1;

    /// Right foot constant
    pub const RIGHT_FOOT_SWING: u8 = 2;

    /// Standing constant
    pub const STANDING: u8 = 3;

}


impl Default for Step2D {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !op3_online_walking_module_msgs__msg__Step2D__init(&mut msg as *mut _) {
        panic!("Call to op3_online_walking_module_msgs__msg__Step2D__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Step2D {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { op3_online_walking_module_msgs__msg__Step2D__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { op3_online_walking_module_msgs__msg__Step2D__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { op3_online_walking_module_msgs__msg__Step2D__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Step2D {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Step2D where Self: Sized {
  const TYPE_NAME: &'static str = "op3_online_walking_module_msgs/msg/Step2D";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__op3_online_walking_module_msgs__msg__Step2D() }
  }
}


#[link(name = "op3_online_walking_module_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__op3_online_walking_module_msgs__msg__Step2DArray() -> *const std::ffi::c_void;
}

#[link(name = "op3_online_walking_module_msgs__rosidl_generator_c")]
extern "C" {
    fn op3_online_walking_module_msgs__msg__Step2DArray__init(msg: *mut Step2DArray) -> bool;
    fn op3_online_walking_module_msgs__msg__Step2DArray__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Step2DArray>, size: usize) -> bool;
    fn op3_online_walking_module_msgs__msg__Step2DArray__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Step2DArray>);
    fn op3_online_walking_module_msgs__msg__Step2DArray__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Step2DArray>, out_seq: *mut rosidl_runtime_rs::Sequence<Step2DArray>) -> bool;
}

// Corresponds to op3_online_walking_module_msgs__msg__Step2DArray
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Step2DArray {

    // This member is not documented.
    #[allow(missing_docs)]
    pub step_time: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub footsteps_2d: rosidl_runtime_rs::Sequence<super::super::msg::rmw::Step2D>,

}



impl Default for Step2DArray {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !op3_online_walking_module_msgs__msg__Step2DArray__init(&mut msg as *mut _) {
        panic!("Call to op3_online_walking_module_msgs__msg__Step2DArray__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Step2DArray {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { op3_online_walking_module_msgs__msg__Step2DArray__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { op3_online_walking_module_msgs__msg__Step2DArray__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { op3_online_walking_module_msgs__msg__Step2DArray__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Step2DArray {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Step2DArray where Self: Sized {
  const TYPE_NAME: &'static str = "op3_online_walking_module_msgs/msg/Step2DArray";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__op3_online_walking_module_msgs__msg__Step2DArray() }
  }
}


