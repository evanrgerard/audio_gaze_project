#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};



#[link(name = "op3_walking_module_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__op3_walking_module_msgs__srv__GetWalkingParam_Request() -> *const std::ffi::c_void;
}

#[link(name = "op3_walking_module_msgs__rosidl_generator_c")]
extern "C" {
    fn op3_walking_module_msgs__srv__GetWalkingParam_Request__init(msg: *mut GetWalkingParam_Request) -> bool;
    fn op3_walking_module_msgs__srv__GetWalkingParam_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetWalkingParam_Request>, size: usize) -> bool;
    fn op3_walking_module_msgs__srv__GetWalkingParam_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetWalkingParam_Request>);
    fn op3_walking_module_msgs__srv__GetWalkingParam_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetWalkingParam_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<GetWalkingParam_Request>) -> bool;
}

// Corresponds to op3_walking_module_msgs__srv__GetWalkingParam_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetWalkingParam_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub get_param: bool,

}



impl Default for GetWalkingParam_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !op3_walking_module_msgs__srv__GetWalkingParam_Request__init(&mut msg as *mut _) {
        panic!("Call to op3_walking_module_msgs__srv__GetWalkingParam_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetWalkingParam_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { op3_walking_module_msgs__srv__GetWalkingParam_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { op3_walking_module_msgs__srv__GetWalkingParam_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { op3_walking_module_msgs__srv__GetWalkingParam_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetWalkingParam_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetWalkingParam_Request where Self: Sized {
  const TYPE_NAME: &'static str = "op3_walking_module_msgs/srv/GetWalkingParam_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__op3_walking_module_msgs__srv__GetWalkingParam_Request() }
  }
}


#[link(name = "op3_walking_module_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__op3_walking_module_msgs__srv__GetWalkingParam_Response() -> *const std::ffi::c_void;
}

#[link(name = "op3_walking_module_msgs__rosidl_generator_c")]
extern "C" {
    fn op3_walking_module_msgs__srv__GetWalkingParam_Response__init(msg: *mut GetWalkingParam_Response) -> bool;
    fn op3_walking_module_msgs__srv__GetWalkingParam_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetWalkingParam_Response>, size: usize) -> bool;
    fn op3_walking_module_msgs__srv__GetWalkingParam_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetWalkingParam_Response>);
    fn op3_walking_module_msgs__srv__GetWalkingParam_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetWalkingParam_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<GetWalkingParam_Response>) -> bool;
}

// Corresponds to op3_walking_module_msgs__srv__GetWalkingParam_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetWalkingParam_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub parameters: super::super::msg::rmw::WalkingParam,

}



impl Default for GetWalkingParam_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !op3_walking_module_msgs__srv__GetWalkingParam_Response__init(&mut msg as *mut _) {
        panic!("Call to op3_walking_module_msgs__srv__GetWalkingParam_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetWalkingParam_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { op3_walking_module_msgs__srv__GetWalkingParam_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { op3_walking_module_msgs__srv__GetWalkingParam_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { op3_walking_module_msgs__srv__GetWalkingParam_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetWalkingParam_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetWalkingParam_Response where Self: Sized {
  const TYPE_NAME: &'static str = "op3_walking_module_msgs/srv/GetWalkingParam_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__op3_walking_module_msgs__srv__GetWalkingParam_Response() }
  }
}


#[link(name = "op3_walking_module_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__op3_walking_module_msgs__srv__SetWalkingParam_Request() -> *const std::ffi::c_void;
}

#[link(name = "op3_walking_module_msgs__rosidl_generator_c")]
extern "C" {
    fn op3_walking_module_msgs__srv__SetWalkingParam_Request__init(msg: *mut SetWalkingParam_Request) -> bool;
    fn op3_walking_module_msgs__srv__SetWalkingParam_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SetWalkingParam_Request>, size: usize) -> bool;
    fn op3_walking_module_msgs__srv__SetWalkingParam_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SetWalkingParam_Request>);
    fn op3_walking_module_msgs__srv__SetWalkingParam_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SetWalkingParam_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<SetWalkingParam_Request>) -> bool;
}

// Corresponds to op3_walking_module_msgs__srv__SetWalkingParam_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetWalkingParam_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub parameters: super::super::msg::rmw::WalkingParam,

}



impl Default for SetWalkingParam_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !op3_walking_module_msgs__srv__SetWalkingParam_Request__init(&mut msg as *mut _) {
        panic!("Call to op3_walking_module_msgs__srv__SetWalkingParam_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SetWalkingParam_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { op3_walking_module_msgs__srv__SetWalkingParam_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { op3_walking_module_msgs__srv__SetWalkingParam_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { op3_walking_module_msgs__srv__SetWalkingParam_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SetWalkingParam_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SetWalkingParam_Request where Self: Sized {
  const TYPE_NAME: &'static str = "op3_walking_module_msgs/srv/SetWalkingParam_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__op3_walking_module_msgs__srv__SetWalkingParam_Request() }
  }
}


#[link(name = "op3_walking_module_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__op3_walking_module_msgs__srv__SetWalkingParam_Response() -> *const std::ffi::c_void;
}

#[link(name = "op3_walking_module_msgs__rosidl_generator_c")]
extern "C" {
    fn op3_walking_module_msgs__srv__SetWalkingParam_Response__init(msg: *mut SetWalkingParam_Response) -> bool;
    fn op3_walking_module_msgs__srv__SetWalkingParam_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SetWalkingParam_Response>, size: usize) -> bool;
    fn op3_walking_module_msgs__srv__SetWalkingParam_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SetWalkingParam_Response>);
    fn op3_walking_module_msgs__srv__SetWalkingParam_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SetWalkingParam_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<SetWalkingParam_Response>) -> bool;
}

// Corresponds to op3_walking_module_msgs__srv__SetWalkingParam_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetWalkingParam_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub result: bool,

}



impl Default for SetWalkingParam_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !op3_walking_module_msgs__srv__SetWalkingParam_Response__init(&mut msg as *mut _) {
        panic!("Call to op3_walking_module_msgs__srv__SetWalkingParam_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SetWalkingParam_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { op3_walking_module_msgs__srv__SetWalkingParam_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { op3_walking_module_msgs__srv__SetWalkingParam_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { op3_walking_module_msgs__srv__SetWalkingParam_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SetWalkingParam_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SetWalkingParam_Response where Self: Sized {
  const TYPE_NAME: &'static str = "op3_walking_module_msgs/srv/SetWalkingParam_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__op3_walking_module_msgs__srv__SetWalkingParam_Response() }
  }
}






#[link(name = "op3_walking_module_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__op3_walking_module_msgs__srv__GetWalkingParam() -> *const std::ffi::c_void;
}

// Corresponds to op3_walking_module_msgs__srv__GetWalkingParam
#[allow(missing_docs, non_camel_case_types)]
pub struct GetWalkingParam;

impl rosidl_runtime_rs::Service for GetWalkingParam {
    type Request = GetWalkingParam_Request;
    type Response = GetWalkingParam_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__op3_walking_module_msgs__srv__GetWalkingParam() }
    }
}




#[link(name = "op3_walking_module_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__op3_walking_module_msgs__srv__SetWalkingParam() -> *const std::ffi::c_void;
}

// Corresponds to op3_walking_module_msgs__srv__SetWalkingParam
#[allow(missing_docs, non_camel_case_types)]
pub struct SetWalkingParam;

impl rosidl_runtime_rs::Service for SetWalkingParam {
    type Request = SetWalkingParam_Request;
    type Response = SetWalkingParam_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__op3_walking_module_msgs__srv__SetWalkingParam() }
    }
}


