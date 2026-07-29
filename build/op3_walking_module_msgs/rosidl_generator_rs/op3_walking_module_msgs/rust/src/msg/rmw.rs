#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};


#[link(name = "op3_walking_module_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__op3_walking_module_msgs__msg__WalkingParam() -> *const std::ffi::c_void;
}

#[link(name = "op3_walking_module_msgs__rosidl_generator_c")]
extern "C" {
    fn op3_walking_module_msgs__msg__WalkingParam__init(msg: *mut WalkingParam) -> bool;
    fn op3_walking_module_msgs__msg__WalkingParam__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<WalkingParam>, size: usize) -> bool;
    fn op3_walking_module_msgs__msg__WalkingParam__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<WalkingParam>);
    fn op3_walking_module_msgs__msg__WalkingParam__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<WalkingParam>, out_seq: *mut rosidl_runtime_rs::Sequence<WalkingParam>) -> bool;
}

// Corresponds to op3_walking_module_msgs__msg__WalkingParam
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// walking init pose #######

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct WalkingParam {

    // This member is not documented.
    #[allow(missing_docs)]
    pub init_x_offset: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub init_y_offset: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub init_z_offset: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub init_roll_offset: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub init_pitch_offset: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub init_yaw_offset: f32,

    /// time parameter #####
    pub period_time: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub dsp_ratio: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub step_fb_ratio: f32,

    /// walking parameter ########
    pub x_move_amplitude: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub y_move_amplitude: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub z_move_amplitude: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub angle_move_amplitude: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub move_aim_on: bool,

    /// balance parameter ##########
    pub balance_enable: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub balance_hip_roll_gain: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub balance_knee_gain: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub balance_ankle_roll_gain: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub balance_ankle_pitch_gain: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub y_swap_amplitude: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub z_swap_amplitude: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub arm_swing_gain: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub pelvis_offset: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub hip_pitch_offset: f32,

    /// gain parameter ##########
    pub p_gain: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub i_gain: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub d_gain: i32,

}



impl Default for WalkingParam {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !op3_walking_module_msgs__msg__WalkingParam__init(&mut msg as *mut _) {
        panic!("Call to op3_walking_module_msgs__msg__WalkingParam__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for WalkingParam {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { op3_walking_module_msgs__msg__WalkingParam__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { op3_walking_module_msgs__msg__WalkingParam__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { op3_walking_module_msgs__msg__WalkingParam__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for WalkingParam {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for WalkingParam where Self: Sized {
  const TYPE_NAME: &'static str = "op3_walking_module_msgs/msg/WalkingParam";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__op3_walking_module_msgs__msg__WalkingParam() }
  }
}


