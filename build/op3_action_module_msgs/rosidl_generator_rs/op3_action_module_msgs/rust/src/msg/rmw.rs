#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};


#[link(name = "op3_action_module_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__op3_action_module_msgs__msg__StartAction() -> *const std::ffi::c_void;
}

#[link(name = "op3_action_module_msgs__rosidl_generator_c")]
extern "C" {
    fn op3_action_module_msgs__msg__StartAction__init(msg: *mut StartAction) -> bool;
    fn op3_action_module_msgs__msg__StartAction__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<StartAction>, size: usize) -> bool;
    fn op3_action_module_msgs__msg__StartAction__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<StartAction>);
    fn op3_action_module_msgs__msg__StartAction__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<StartAction>, out_seq: *mut rosidl_runtime_rs::Sequence<StartAction>) -> bool;
}

// Corresponds to op3_action_module_msgs__msg__StartAction
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct StartAction {

    // This member is not documented.
    #[allow(missing_docs)]
    pub page_num: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub joint_name_array: rosidl_runtime_rs::Sequence<rosidl_runtime_rs::String>,

}



impl Default for StartAction {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !op3_action_module_msgs__msg__StartAction__init(&mut msg as *mut _) {
        panic!("Call to op3_action_module_msgs__msg__StartAction__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for StartAction {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { op3_action_module_msgs__msg__StartAction__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { op3_action_module_msgs__msg__StartAction__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { op3_action_module_msgs__msg__StartAction__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for StartAction {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for StartAction where Self: Sized {
  const TYPE_NAME: &'static str = "op3_action_module_msgs/msg/StartAction";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__op3_action_module_msgs__msg__StartAction() }
  }
}


