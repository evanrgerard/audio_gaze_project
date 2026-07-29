#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};


#[link(name = "op3_tuning_module_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__op3_tuning_module_msgs__msg__JointOffsetData() -> *const std::ffi::c_void;
}

#[link(name = "op3_tuning_module_msgs__rosidl_generator_c")]
extern "C" {
    fn op3_tuning_module_msgs__msg__JointOffsetData__init(msg: *mut JointOffsetData) -> bool;
    fn op3_tuning_module_msgs__msg__JointOffsetData__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<JointOffsetData>, size: usize) -> bool;
    fn op3_tuning_module_msgs__msg__JointOffsetData__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<JointOffsetData>);
    fn op3_tuning_module_msgs__msg__JointOffsetData__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<JointOffsetData>, out_seq: *mut rosidl_runtime_rs::Sequence<JointOffsetData>) -> bool;
}

// Corresponds to op3_tuning_module_msgs__msg__JointOffsetData
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct JointOffsetData {

    // This member is not documented.
    #[allow(missing_docs)]
    pub joint_name: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_value: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub offset_value: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub p_gain: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub i_gain: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub d_gain: i32,

}



impl Default for JointOffsetData {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !op3_tuning_module_msgs__msg__JointOffsetData__init(&mut msg as *mut _) {
        panic!("Call to op3_tuning_module_msgs__msg__JointOffsetData__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for JointOffsetData {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { op3_tuning_module_msgs__msg__JointOffsetData__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { op3_tuning_module_msgs__msg__JointOffsetData__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { op3_tuning_module_msgs__msg__JointOffsetData__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for JointOffsetData {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for JointOffsetData where Self: Sized {
  const TYPE_NAME: &'static str = "op3_tuning_module_msgs/msg/JointOffsetData";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__op3_tuning_module_msgs__msg__JointOffsetData() }
  }
}


#[link(name = "op3_tuning_module_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__op3_tuning_module_msgs__msg__JointOffsetPositionData() -> *const std::ffi::c_void;
}

#[link(name = "op3_tuning_module_msgs__rosidl_generator_c")]
extern "C" {
    fn op3_tuning_module_msgs__msg__JointOffsetPositionData__init(msg: *mut JointOffsetPositionData) -> bool;
    fn op3_tuning_module_msgs__msg__JointOffsetPositionData__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<JointOffsetPositionData>, size: usize) -> bool;
    fn op3_tuning_module_msgs__msg__JointOffsetPositionData__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<JointOffsetPositionData>);
    fn op3_tuning_module_msgs__msg__JointOffsetPositionData__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<JointOffsetPositionData>, out_seq: *mut rosidl_runtime_rs::Sequence<JointOffsetPositionData>) -> bool;
}

// Corresponds to op3_tuning_module_msgs__msg__JointOffsetPositionData
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct JointOffsetPositionData {

    // This member is not documented.
    #[allow(missing_docs)]
    pub joint_name: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_value: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub offset_value: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub present_value: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub p_gain: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub i_gain: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub d_gain: i32,

}



impl Default for JointOffsetPositionData {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !op3_tuning_module_msgs__msg__JointOffsetPositionData__init(&mut msg as *mut _) {
        panic!("Call to op3_tuning_module_msgs__msg__JointOffsetPositionData__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for JointOffsetPositionData {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { op3_tuning_module_msgs__msg__JointOffsetPositionData__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { op3_tuning_module_msgs__msg__JointOffsetPositionData__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { op3_tuning_module_msgs__msg__JointOffsetPositionData__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for JointOffsetPositionData {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for JointOffsetPositionData where Self: Sized {
  const TYPE_NAME: &'static str = "op3_tuning_module_msgs/msg/JointOffsetPositionData";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__op3_tuning_module_msgs__msg__JointOffsetPositionData() }
  }
}


#[link(name = "op3_tuning_module_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__op3_tuning_module_msgs__msg__JointTorqueOnOff() -> *const std::ffi::c_void;
}

#[link(name = "op3_tuning_module_msgs__rosidl_generator_c")]
extern "C" {
    fn op3_tuning_module_msgs__msg__JointTorqueOnOff__init(msg: *mut JointTorqueOnOff) -> bool;
    fn op3_tuning_module_msgs__msg__JointTorqueOnOff__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<JointTorqueOnOff>, size: usize) -> bool;
    fn op3_tuning_module_msgs__msg__JointTorqueOnOff__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<JointTorqueOnOff>);
    fn op3_tuning_module_msgs__msg__JointTorqueOnOff__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<JointTorqueOnOff>, out_seq: *mut rosidl_runtime_rs::Sequence<JointTorqueOnOff>) -> bool;
}

// Corresponds to op3_tuning_module_msgs__msg__JointTorqueOnOff
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct JointTorqueOnOff {

    // This member is not documented.
    #[allow(missing_docs)]
    pub joint_name: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub torque_enable: bool,

}



impl Default for JointTorqueOnOff {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !op3_tuning_module_msgs__msg__JointTorqueOnOff__init(&mut msg as *mut _) {
        panic!("Call to op3_tuning_module_msgs__msg__JointTorqueOnOff__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for JointTorqueOnOff {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { op3_tuning_module_msgs__msg__JointTorqueOnOff__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { op3_tuning_module_msgs__msg__JointTorqueOnOff__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { op3_tuning_module_msgs__msg__JointTorqueOnOff__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for JointTorqueOnOff {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for JointTorqueOnOff where Self: Sized {
  const TYPE_NAME: &'static str = "op3_tuning_module_msgs/msg/JointTorqueOnOff";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__op3_tuning_module_msgs__msg__JointTorqueOnOff() }
  }
}


#[link(name = "op3_tuning_module_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__op3_tuning_module_msgs__msg__JointTorqueOnOffArray() -> *const std::ffi::c_void;
}

#[link(name = "op3_tuning_module_msgs__rosidl_generator_c")]
extern "C" {
    fn op3_tuning_module_msgs__msg__JointTorqueOnOffArray__init(msg: *mut JointTorqueOnOffArray) -> bool;
    fn op3_tuning_module_msgs__msg__JointTorqueOnOffArray__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<JointTorqueOnOffArray>, size: usize) -> bool;
    fn op3_tuning_module_msgs__msg__JointTorqueOnOffArray__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<JointTorqueOnOffArray>);
    fn op3_tuning_module_msgs__msg__JointTorqueOnOffArray__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<JointTorqueOnOffArray>, out_seq: *mut rosidl_runtime_rs::Sequence<JointTorqueOnOffArray>) -> bool;
}

// Corresponds to op3_tuning_module_msgs__msg__JointTorqueOnOffArray
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct JointTorqueOnOffArray {

    // This member is not documented.
    #[allow(missing_docs)]
    pub torque_enable_data: rosidl_runtime_rs::Sequence<super::super::msg::rmw::JointTorqueOnOff>,

}



impl Default for JointTorqueOnOffArray {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !op3_tuning_module_msgs__msg__JointTorqueOnOffArray__init(&mut msg as *mut _) {
        panic!("Call to op3_tuning_module_msgs__msg__JointTorqueOnOffArray__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for JointTorqueOnOffArray {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { op3_tuning_module_msgs__msg__JointTorqueOnOffArray__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { op3_tuning_module_msgs__msg__JointTorqueOnOffArray__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { op3_tuning_module_msgs__msg__JointTorqueOnOffArray__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for JointTorqueOnOffArray {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for JointTorqueOnOffArray where Self: Sized {
  const TYPE_NAME: &'static str = "op3_tuning_module_msgs/msg/JointTorqueOnOffArray";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__op3_tuning_module_msgs__msg__JointTorqueOnOffArray() }
  }
}


