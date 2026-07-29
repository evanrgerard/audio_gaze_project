#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};



#[link(name = "robotis_controller_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__robotis_controller_msgs__srv__GetJointModule_Request() -> *const std::ffi::c_void;
}

#[link(name = "robotis_controller_msgs__rosidl_generator_c")]
extern "C" {
    fn robotis_controller_msgs__srv__GetJointModule_Request__init(msg: *mut GetJointModule_Request) -> bool;
    fn robotis_controller_msgs__srv__GetJointModule_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetJointModule_Request>, size: usize) -> bool;
    fn robotis_controller_msgs__srv__GetJointModule_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetJointModule_Request>);
    fn robotis_controller_msgs__srv__GetJointModule_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetJointModule_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<GetJointModule_Request>) -> bool;
}

// Corresponds to robotis_controller_msgs__srv__GetJointModule_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetJointModule_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub joint_name: rosidl_runtime_rs::Sequence<rosidl_runtime_rs::String>,

}



impl Default for GetJointModule_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !robotis_controller_msgs__srv__GetJointModule_Request__init(&mut msg as *mut _) {
        panic!("Call to robotis_controller_msgs__srv__GetJointModule_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetJointModule_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robotis_controller_msgs__srv__GetJointModule_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robotis_controller_msgs__srv__GetJointModule_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robotis_controller_msgs__srv__GetJointModule_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetJointModule_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetJointModule_Request where Self: Sized {
  const TYPE_NAME: &'static str = "robotis_controller_msgs/srv/GetJointModule_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__robotis_controller_msgs__srv__GetJointModule_Request() }
  }
}


#[link(name = "robotis_controller_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__robotis_controller_msgs__srv__GetJointModule_Response() -> *const std::ffi::c_void;
}

#[link(name = "robotis_controller_msgs__rosidl_generator_c")]
extern "C" {
    fn robotis_controller_msgs__srv__GetJointModule_Response__init(msg: *mut GetJointModule_Response) -> bool;
    fn robotis_controller_msgs__srv__GetJointModule_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetJointModule_Response>, size: usize) -> bool;
    fn robotis_controller_msgs__srv__GetJointModule_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetJointModule_Response>);
    fn robotis_controller_msgs__srv__GetJointModule_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetJointModule_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<GetJointModule_Response>) -> bool;
}

// Corresponds to robotis_controller_msgs__srv__GetJointModule_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetJointModule_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub joint_name: rosidl_runtime_rs::Sequence<rosidl_runtime_rs::String>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub module_name: rosidl_runtime_rs::Sequence<rosidl_runtime_rs::String>,

}



impl Default for GetJointModule_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !robotis_controller_msgs__srv__GetJointModule_Response__init(&mut msg as *mut _) {
        panic!("Call to robotis_controller_msgs__srv__GetJointModule_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetJointModule_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robotis_controller_msgs__srv__GetJointModule_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robotis_controller_msgs__srv__GetJointModule_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robotis_controller_msgs__srv__GetJointModule_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetJointModule_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetJointModule_Response where Self: Sized {
  const TYPE_NAME: &'static str = "robotis_controller_msgs/srv/GetJointModule_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__robotis_controller_msgs__srv__GetJointModule_Response() }
  }
}


#[link(name = "robotis_controller_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__robotis_controller_msgs__srv__SetJointModule_Request() -> *const std::ffi::c_void;
}

#[link(name = "robotis_controller_msgs__rosidl_generator_c")]
extern "C" {
    fn robotis_controller_msgs__srv__SetJointModule_Request__init(msg: *mut SetJointModule_Request) -> bool;
    fn robotis_controller_msgs__srv__SetJointModule_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SetJointModule_Request>, size: usize) -> bool;
    fn robotis_controller_msgs__srv__SetJointModule_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SetJointModule_Request>);
    fn robotis_controller_msgs__srv__SetJointModule_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SetJointModule_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<SetJointModule_Request>) -> bool;
}

// Corresponds to robotis_controller_msgs__srv__SetJointModule_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetJointModule_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub joint_name: rosidl_runtime_rs::Sequence<rosidl_runtime_rs::String>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub module_name: rosidl_runtime_rs::Sequence<rosidl_runtime_rs::String>,

}



impl Default for SetJointModule_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !robotis_controller_msgs__srv__SetJointModule_Request__init(&mut msg as *mut _) {
        panic!("Call to robotis_controller_msgs__srv__SetJointModule_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SetJointModule_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robotis_controller_msgs__srv__SetJointModule_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robotis_controller_msgs__srv__SetJointModule_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robotis_controller_msgs__srv__SetJointModule_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SetJointModule_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SetJointModule_Request where Self: Sized {
  const TYPE_NAME: &'static str = "robotis_controller_msgs/srv/SetJointModule_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__robotis_controller_msgs__srv__SetJointModule_Request() }
  }
}


#[link(name = "robotis_controller_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__robotis_controller_msgs__srv__SetJointModule_Response() -> *const std::ffi::c_void;
}

#[link(name = "robotis_controller_msgs__rosidl_generator_c")]
extern "C" {
    fn robotis_controller_msgs__srv__SetJointModule_Response__init(msg: *mut SetJointModule_Response) -> bool;
    fn robotis_controller_msgs__srv__SetJointModule_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SetJointModule_Response>, size: usize) -> bool;
    fn robotis_controller_msgs__srv__SetJointModule_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SetJointModule_Response>);
    fn robotis_controller_msgs__srv__SetJointModule_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SetJointModule_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<SetJointModule_Response>) -> bool;
}

// Corresponds to robotis_controller_msgs__srv__SetJointModule_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetJointModule_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub result: bool,

}



impl Default for SetJointModule_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !robotis_controller_msgs__srv__SetJointModule_Response__init(&mut msg as *mut _) {
        panic!("Call to robotis_controller_msgs__srv__SetJointModule_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SetJointModule_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robotis_controller_msgs__srv__SetJointModule_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robotis_controller_msgs__srv__SetJointModule_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robotis_controller_msgs__srv__SetJointModule_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SetJointModule_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SetJointModule_Response where Self: Sized {
  const TYPE_NAME: &'static str = "robotis_controller_msgs/srv/SetJointModule_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__robotis_controller_msgs__srv__SetJointModule_Response() }
  }
}


#[link(name = "robotis_controller_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__robotis_controller_msgs__srv__SetModule_Request() -> *const std::ffi::c_void;
}

#[link(name = "robotis_controller_msgs__rosidl_generator_c")]
extern "C" {
    fn robotis_controller_msgs__srv__SetModule_Request__init(msg: *mut SetModule_Request) -> bool;
    fn robotis_controller_msgs__srv__SetModule_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SetModule_Request>, size: usize) -> bool;
    fn robotis_controller_msgs__srv__SetModule_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SetModule_Request>);
    fn robotis_controller_msgs__srv__SetModule_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SetModule_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<SetModule_Request>) -> bool;
}

// Corresponds to robotis_controller_msgs__srv__SetModule_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetModule_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub module_name: rosidl_runtime_rs::String,

}



impl Default for SetModule_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !robotis_controller_msgs__srv__SetModule_Request__init(&mut msg as *mut _) {
        panic!("Call to robotis_controller_msgs__srv__SetModule_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SetModule_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robotis_controller_msgs__srv__SetModule_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robotis_controller_msgs__srv__SetModule_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robotis_controller_msgs__srv__SetModule_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SetModule_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SetModule_Request where Self: Sized {
  const TYPE_NAME: &'static str = "robotis_controller_msgs/srv/SetModule_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__robotis_controller_msgs__srv__SetModule_Request() }
  }
}


#[link(name = "robotis_controller_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__robotis_controller_msgs__srv__SetModule_Response() -> *const std::ffi::c_void;
}

#[link(name = "robotis_controller_msgs__rosidl_generator_c")]
extern "C" {
    fn robotis_controller_msgs__srv__SetModule_Response__init(msg: *mut SetModule_Response) -> bool;
    fn robotis_controller_msgs__srv__SetModule_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SetModule_Response>, size: usize) -> bool;
    fn robotis_controller_msgs__srv__SetModule_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SetModule_Response>);
    fn robotis_controller_msgs__srv__SetModule_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SetModule_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<SetModule_Response>) -> bool;
}

// Corresponds to robotis_controller_msgs__srv__SetModule_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetModule_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub result: bool,

}



impl Default for SetModule_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !robotis_controller_msgs__srv__SetModule_Response__init(&mut msg as *mut _) {
        panic!("Call to robotis_controller_msgs__srv__SetModule_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SetModule_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robotis_controller_msgs__srv__SetModule_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robotis_controller_msgs__srv__SetModule_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robotis_controller_msgs__srv__SetModule_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SetModule_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SetModule_Response where Self: Sized {
  const TYPE_NAME: &'static str = "robotis_controller_msgs/srv/SetModule_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__robotis_controller_msgs__srv__SetModule_Response() }
  }
}


#[link(name = "robotis_controller_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__robotis_controller_msgs__srv__LoadOffset_Request() -> *const std::ffi::c_void;
}

#[link(name = "robotis_controller_msgs__rosidl_generator_c")]
extern "C" {
    fn robotis_controller_msgs__srv__LoadOffset_Request__init(msg: *mut LoadOffset_Request) -> bool;
    fn robotis_controller_msgs__srv__LoadOffset_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<LoadOffset_Request>, size: usize) -> bool;
    fn robotis_controller_msgs__srv__LoadOffset_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<LoadOffset_Request>);
    fn robotis_controller_msgs__srv__LoadOffset_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<LoadOffset_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<LoadOffset_Request>) -> bool;
}

// Corresponds to robotis_controller_msgs__srv__LoadOffset_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct LoadOffset_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub file_path: rosidl_runtime_rs::String,

}



impl Default for LoadOffset_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !robotis_controller_msgs__srv__LoadOffset_Request__init(&mut msg as *mut _) {
        panic!("Call to robotis_controller_msgs__srv__LoadOffset_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for LoadOffset_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robotis_controller_msgs__srv__LoadOffset_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robotis_controller_msgs__srv__LoadOffset_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robotis_controller_msgs__srv__LoadOffset_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for LoadOffset_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for LoadOffset_Request where Self: Sized {
  const TYPE_NAME: &'static str = "robotis_controller_msgs/srv/LoadOffset_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__robotis_controller_msgs__srv__LoadOffset_Request() }
  }
}


#[link(name = "robotis_controller_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__robotis_controller_msgs__srv__LoadOffset_Response() -> *const std::ffi::c_void;
}

#[link(name = "robotis_controller_msgs__rosidl_generator_c")]
extern "C" {
    fn robotis_controller_msgs__srv__LoadOffset_Response__init(msg: *mut LoadOffset_Response) -> bool;
    fn robotis_controller_msgs__srv__LoadOffset_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<LoadOffset_Response>, size: usize) -> bool;
    fn robotis_controller_msgs__srv__LoadOffset_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<LoadOffset_Response>);
    fn robotis_controller_msgs__srv__LoadOffset_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<LoadOffset_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<LoadOffset_Response>) -> bool;
}

// Corresponds to robotis_controller_msgs__srv__LoadOffset_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct LoadOffset_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub result: bool,

}



impl Default for LoadOffset_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !robotis_controller_msgs__srv__LoadOffset_Response__init(&mut msg as *mut _) {
        panic!("Call to robotis_controller_msgs__srv__LoadOffset_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for LoadOffset_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robotis_controller_msgs__srv__LoadOffset_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robotis_controller_msgs__srv__LoadOffset_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robotis_controller_msgs__srv__LoadOffset_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for LoadOffset_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for LoadOffset_Response where Self: Sized {
  const TYPE_NAME: &'static str = "robotis_controller_msgs/srv/LoadOffset_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__robotis_controller_msgs__srv__LoadOffset_Response() }
  }
}






#[link(name = "robotis_controller_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__robotis_controller_msgs__srv__GetJointModule() -> *const std::ffi::c_void;
}

// Corresponds to robotis_controller_msgs__srv__GetJointModule
#[allow(missing_docs, non_camel_case_types)]
pub struct GetJointModule;

impl rosidl_runtime_rs::Service for GetJointModule {
    type Request = GetJointModule_Request;
    type Response = GetJointModule_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__robotis_controller_msgs__srv__GetJointModule() }
    }
}




#[link(name = "robotis_controller_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__robotis_controller_msgs__srv__SetJointModule() -> *const std::ffi::c_void;
}

// Corresponds to robotis_controller_msgs__srv__SetJointModule
#[allow(missing_docs, non_camel_case_types)]
pub struct SetJointModule;

impl rosidl_runtime_rs::Service for SetJointModule {
    type Request = SetJointModule_Request;
    type Response = SetJointModule_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__robotis_controller_msgs__srv__SetJointModule() }
    }
}




#[link(name = "robotis_controller_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__robotis_controller_msgs__srv__SetModule() -> *const std::ffi::c_void;
}

// Corresponds to robotis_controller_msgs__srv__SetModule
#[allow(missing_docs, non_camel_case_types)]
pub struct SetModule;

impl rosidl_runtime_rs::Service for SetModule {
    type Request = SetModule_Request;
    type Response = SetModule_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__robotis_controller_msgs__srv__SetModule() }
    }
}




#[link(name = "robotis_controller_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__robotis_controller_msgs__srv__LoadOffset() -> *const std::ffi::c_void;
}

// Corresponds to robotis_controller_msgs__srv__LoadOffset
#[allow(missing_docs, non_camel_case_types)]
pub struct LoadOffset;

impl rosidl_runtime_rs::Service for LoadOffset {
    type Request = LoadOffset_Request;
    type Response = LoadOffset_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__robotis_controller_msgs__srv__LoadOffset() }
    }
}


