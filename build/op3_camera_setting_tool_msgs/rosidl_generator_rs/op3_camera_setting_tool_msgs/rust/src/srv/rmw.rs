#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};



#[link(name = "op3_camera_setting_tool_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__op3_camera_setting_tool_msgs__srv__GetParameters_Request() -> *const std::ffi::c_void;
}

#[link(name = "op3_camera_setting_tool_msgs__rosidl_generator_c")]
extern "C" {
    fn op3_camera_setting_tool_msgs__srv__GetParameters_Request__init(msg: *mut GetParameters_Request) -> bool;
    fn op3_camera_setting_tool_msgs__srv__GetParameters_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetParameters_Request>, size: usize) -> bool;
    fn op3_camera_setting_tool_msgs__srv__GetParameters_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetParameters_Request>);
    fn op3_camera_setting_tool_msgs__srv__GetParameters_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetParameters_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<GetParameters_Request>) -> bool;
}

// Corresponds to op3_camera_setting_tool_msgs__srv__GetParameters_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetParameters_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for GetParameters_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !op3_camera_setting_tool_msgs__srv__GetParameters_Request__init(&mut msg as *mut _) {
        panic!("Call to op3_camera_setting_tool_msgs__srv__GetParameters_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetParameters_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { op3_camera_setting_tool_msgs__srv__GetParameters_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { op3_camera_setting_tool_msgs__srv__GetParameters_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { op3_camera_setting_tool_msgs__srv__GetParameters_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetParameters_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetParameters_Request where Self: Sized {
  const TYPE_NAME: &'static str = "op3_camera_setting_tool_msgs/srv/GetParameters_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__op3_camera_setting_tool_msgs__srv__GetParameters_Request() }
  }
}


#[link(name = "op3_camera_setting_tool_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__op3_camera_setting_tool_msgs__srv__GetParameters_Response() -> *const std::ffi::c_void;
}

#[link(name = "op3_camera_setting_tool_msgs__rosidl_generator_c")]
extern "C" {
    fn op3_camera_setting_tool_msgs__srv__GetParameters_Response__init(msg: *mut GetParameters_Response) -> bool;
    fn op3_camera_setting_tool_msgs__srv__GetParameters_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetParameters_Response>, size: usize) -> bool;
    fn op3_camera_setting_tool_msgs__srv__GetParameters_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetParameters_Response>);
    fn op3_camera_setting_tool_msgs__srv__GetParameters_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetParameters_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<GetParameters_Response>) -> bool;
}

// Corresponds to op3_camera_setting_tool_msgs__srv__GetParameters_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetParameters_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub returns: super::super::msg::rmw::CameraParams,

}



impl Default for GetParameters_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !op3_camera_setting_tool_msgs__srv__GetParameters_Response__init(&mut msg as *mut _) {
        panic!("Call to op3_camera_setting_tool_msgs__srv__GetParameters_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetParameters_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { op3_camera_setting_tool_msgs__srv__GetParameters_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { op3_camera_setting_tool_msgs__srv__GetParameters_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { op3_camera_setting_tool_msgs__srv__GetParameters_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetParameters_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetParameters_Response where Self: Sized {
  const TYPE_NAME: &'static str = "op3_camera_setting_tool_msgs/srv/GetParameters_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__op3_camera_setting_tool_msgs__srv__GetParameters_Response() }
  }
}


#[link(name = "op3_camera_setting_tool_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__op3_camera_setting_tool_msgs__srv__SetParameters_Request() -> *const std::ffi::c_void;
}

#[link(name = "op3_camera_setting_tool_msgs__rosidl_generator_c")]
extern "C" {
    fn op3_camera_setting_tool_msgs__srv__SetParameters_Request__init(msg: *mut SetParameters_Request) -> bool;
    fn op3_camera_setting_tool_msgs__srv__SetParameters_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SetParameters_Request>, size: usize) -> bool;
    fn op3_camera_setting_tool_msgs__srv__SetParameters_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SetParameters_Request>);
    fn op3_camera_setting_tool_msgs__srv__SetParameters_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SetParameters_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<SetParameters_Request>) -> bool;
}

// Corresponds to op3_camera_setting_tool_msgs__srv__SetParameters_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetParameters_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub params: super::super::msg::rmw::CameraParams,

}



impl Default for SetParameters_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !op3_camera_setting_tool_msgs__srv__SetParameters_Request__init(&mut msg as *mut _) {
        panic!("Call to op3_camera_setting_tool_msgs__srv__SetParameters_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SetParameters_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { op3_camera_setting_tool_msgs__srv__SetParameters_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { op3_camera_setting_tool_msgs__srv__SetParameters_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { op3_camera_setting_tool_msgs__srv__SetParameters_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SetParameters_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SetParameters_Request where Self: Sized {
  const TYPE_NAME: &'static str = "op3_camera_setting_tool_msgs/srv/SetParameters_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__op3_camera_setting_tool_msgs__srv__SetParameters_Request() }
  }
}


#[link(name = "op3_camera_setting_tool_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__op3_camera_setting_tool_msgs__srv__SetParameters_Response() -> *const std::ffi::c_void;
}

#[link(name = "op3_camera_setting_tool_msgs__rosidl_generator_c")]
extern "C" {
    fn op3_camera_setting_tool_msgs__srv__SetParameters_Response__init(msg: *mut SetParameters_Response) -> bool;
    fn op3_camera_setting_tool_msgs__srv__SetParameters_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SetParameters_Response>, size: usize) -> bool;
    fn op3_camera_setting_tool_msgs__srv__SetParameters_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SetParameters_Response>);
    fn op3_camera_setting_tool_msgs__srv__SetParameters_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SetParameters_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<SetParameters_Response>) -> bool;
}

// Corresponds to op3_camera_setting_tool_msgs__srv__SetParameters_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetParameters_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub returns: super::super::msg::rmw::CameraParams,

}



impl Default for SetParameters_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !op3_camera_setting_tool_msgs__srv__SetParameters_Response__init(&mut msg as *mut _) {
        panic!("Call to op3_camera_setting_tool_msgs__srv__SetParameters_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SetParameters_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { op3_camera_setting_tool_msgs__srv__SetParameters_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { op3_camera_setting_tool_msgs__srv__SetParameters_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { op3_camera_setting_tool_msgs__srv__SetParameters_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SetParameters_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SetParameters_Response where Self: Sized {
  const TYPE_NAME: &'static str = "op3_camera_setting_tool_msgs/srv/SetParameters_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__op3_camera_setting_tool_msgs__srv__SetParameters_Response() }
  }
}






#[link(name = "op3_camera_setting_tool_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__op3_camera_setting_tool_msgs__srv__GetParameters() -> *const std::ffi::c_void;
}

// Corresponds to op3_camera_setting_tool_msgs__srv__GetParameters
#[allow(missing_docs, non_camel_case_types)]
pub struct GetParameters;

impl rosidl_runtime_rs::Service for GetParameters {
    type Request = GetParameters_Request;
    type Response = GetParameters_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__op3_camera_setting_tool_msgs__srv__GetParameters() }
    }
}




#[link(name = "op3_camera_setting_tool_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__op3_camera_setting_tool_msgs__srv__SetParameters() -> *const std::ffi::c_void;
}

// Corresponds to op3_camera_setting_tool_msgs__srv__SetParameters
#[allow(missing_docs, non_camel_case_types)]
pub struct SetParameters;

impl rosidl_runtime_rs::Service for SetParameters {
    type Request = SetParameters_Request;
    type Response = SetParameters_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__op3_camera_setting_tool_msgs__srv__SetParameters() }
    }
}


