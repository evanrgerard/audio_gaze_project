#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};



#[link(name = "op3_offset_tuner_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__op3_offset_tuner_msgs__srv__GetPresentJointOffsetData_Request() -> *const std::ffi::c_void;
}

#[link(name = "op3_offset_tuner_msgs__rosidl_generator_c")]
extern "C" {
    fn op3_offset_tuner_msgs__srv__GetPresentJointOffsetData_Request__init(msg: *mut GetPresentJointOffsetData_Request) -> bool;
    fn op3_offset_tuner_msgs__srv__GetPresentJointOffsetData_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetPresentJointOffsetData_Request>, size: usize) -> bool;
    fn op3_offset_tuner_msgs__srv__GetPresentJointOffsetData_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetPresentJointOffsetData_Request>);
    fn op3_offset_tuner_msgs__srv__GetPresentJointOffsetData_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetPresentJointOffsetData_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<GetPresentJointOffsetData_Request>) -> bool;
}

// Corresponds to op3_offset_tuner_msgs__srv__GetPresentJointOffsetData_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetPresentJointOffsetData_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for GetPresentJointOffsetData_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !op3_offset_tuner_msgs__srv__GetPresentJointOffsetData_Request__init(&mut msg as *mut _) {
        panic!("Call to op3_offset_tuner_msgs__srv__GetPresentJointOffsetData_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetPresentJointOffsetData_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { op3_offset_tuner_msgs__srv__GetPresentJointOffsetData_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { op3_offset_tuner_msgs__srv__GetPresentJointOffsetData_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { op3_offset_tuner_msgs__srv__GetPresentJointOffsetData_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetPresentJointOffsetData_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetPresentJointOffsetData_Request where Self: Sized {
  const TYPE_NAME: &'static str = "op3_offset_tuner_msgs/srv/GetPresentJointOffsetData_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__op3_offset_tuner_msgs__srv__GetPresentJointOffsetData_Request() }
  }
}


#[link(name = "op3_offset_tuner_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__op3_offset_tuner_msgs__srv__GetPresentJointOffsetData_Response() -> *const std::ffi::c_void;
}

#[link(name = "op3_offset_tuner_msgs__rosidl_generator_c")]
extern "C" {
    fn op3_offset_tuner_msgs__srv__GetPresentJointOffsetData_Response__init(msg: *mut GetPresentJointOffsetData_Response) -> bool;
    fn op3_offset_tuner_msgs__srv__GetPresentJointOffsetData_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetPresentJointOffsetData_Response>, size: usize) -> bool;
    fn op3_offset_tuner_msgs__srv__GetPresentJointOffsetData_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetPresentJointOffsetData_Response>);
    fn op3_offset_tuner_msgs__srv__GetPresentJointOffsetData_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetPresentJointOffsetData_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<GetPresentJointOffsetData_Response>) -> bool;
}

// Corresponds to op3_offset_tuner_msgs__srv__GetPresentJointOffsetData_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetPresentJointOffsetData_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub present_data_array: rosidl_runtime_rs::Sequence<super::super::msg::rmw::JointOffsetPositionData>,

}



impl Default for GetPresentJointOffsetData_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !op3_offset_tuner_msgs__srv__GetPresentJointOffsetData_Response__init(&mut msg as *mut _) {
        panic!("Call to op3_offset_tuner_msgs__srv__GetPresentJointOffsetData_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetPresentJointOffsetData_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { op3_offset_tuner_msgs__srv__GetPresentJointOffsetData_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { op3_offset_tuner_msgs__srv__GetPresentJointOffsetData_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { op3_offset_tuner_msgs__srv__GetPresentJointOffsetData_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetPresentJointOffsetData_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetPresentJointOffsetData_Response where Self: Sized {
  const TYPE_NAME: &'static str = "op3_offset_tuner_msgs/srv/GetPresentJointOffsetData_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__op3_offset_tuner_msgs__srv__GetPresentJointOffsetData_Response() }
  }
}






#[link(name = "op3_offset_tuner_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__op3_offset_tuner_msgs__srv__GetPresentJointOffsetData() -> *const std::ffi::c_void;
}

// Corresponds to op3_offset_tuner_msgs__srv__GetPresentJointOffsetData
#[allow(missing_docs, non_camel_case_types)]
pub struct GetPresentJointOffsetData;

impl rosidl_runtime_rs::Service for GetPresentJointOffsetData {
    type Request = GetPresentJointOffsetData_Request;
    type Response = GetPresentJointOffsetData_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__op3_offset_tuner_msgs__srv__GetPresentJointOffsetData() }
    }
}


