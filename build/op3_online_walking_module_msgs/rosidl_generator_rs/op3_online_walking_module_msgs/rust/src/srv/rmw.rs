#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};



#[link(name = "op3_online_walking_module_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__op3_online_walking_module_msgs__srv__GetJointPose_Request() -> *const std::ffi::c_void;
}

#[link(name = "op3_online_walking_module_msgs__rosidl_generator_c")]
extern "C" {
    fn op3_online_walking_module_msgs__srv__GetJointPose_Request__init(msg: *mut GetJointPose_Request) -> bool;
    fn op3_online_walking_module_msgs__srv__GetJointPose_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetJointPose_Request>, size: usize) -> bool;
    fn op3_online_walking_module_msgs__srv__GetJointPose_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetJointPose_Request>);
    fn op3_online_walking_module_msgs__srv__GetJointPose_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetJointPose_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<GetJointPose_Request>) -> bool;
}

// Corresponds to op3_online_walking_module_msgs__srv__GetJointPose_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetJointPose_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for GetJointPose_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !op3_online_walking_module_msgs__srv__GetJointPose_Request__init(&mut msg as *mut _) {
        panic!("Call to op3_online_walking_module_msgs__srv__GetJointPose_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetJointPose_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { op3_online_walking_module_msgs__srv__GetJointPose_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { op3_online_walking_module_msgs__srv__GetJointPose_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { op3_online_walking_module_msgs__srv__GetJointPose_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetJointPose_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetJointPose_Request where Self: Sized {
  const TYPE_NAME: &'static str = "op3_online_walking_module_msgs/srv/GetJointPose_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__op3_online_walking_module_msgs__srv__GetJointPose_Request() }
  }
}


#[link(name = "op3_online_walking_module_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__op3_online_walking_module_msgs__srv__GetJointPose_Response() -> *const std::ffi::c_void;
}

#[link(name = "op3_online_walking_module_msgs__rosidl_generator_c")]
extern "C" {
    fn op3_online_walking_module_msgs__srv__GetJointPose_Response__init(msg: *mut GetJointPose_Response) -> bool;
    fn op3_online_walking_module_msgs__srv__GetJointPose_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetJointPose_Response>, size: usize) -> bool;
    fn op3_online_walking_module_msgs__srv__GetJointPose_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetJointPose_Response>);
    fn op3_online_walking_module_msgs__srv__GetJointPose_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetJointPose_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<GetJointPose_Response>) -> bool;
}

// Corresponds to op3_online_walking_module_msgs__srv__GetJointPose_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetJointPose_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub pose: super::super::msg::rmw::JointPose,

}



impl Default for GetJointPose_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !op3_online_walking_module_msgs__srv__GetJointPose_Response__init(&mut msg as *mut _) {
        panic!("Call to op3_online_walking_module_msgs__srv__GetJointPose_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetJointPose_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { op3_online_walking_module_msgs__srv__GetJointPose_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { op3_online_walking_module_msgs__srv__GetJointPose_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { op3_online_walking_module_msgs__srv__GetJointPose_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetJointPose_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetJointPose_Response where Self: Sized {
  const TYPE_NAME: &'static str = "op3_online_walking_module_msgs/srv/GetJointPose_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__op3_online_walking_module_msgs__srv__GetJointPose_Response() }
  }
}


#[link(name = "op3_online_walking_module_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__op3_online_walking_module_msgs__srv__GetKinematicsPose_Request() -> *const std::ffi::c_void;
}

#[link(name = "op3_online_walking_module_msgs__rosidl_generator_c")]
extern "C" {
    fn op3_online_walking_module_msgs__srv__GetKinematicsPose_Request__init(msg: *mut GetKinematicsPose_Request) -> bool;
    fn op3_online_walking_module_msgs__srv__GetKinematicsPose_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetKinematicsPose_Request>, size: usize) -> bool;
    fn op3_online_walking_module_msgs__srv__GetKinematicsPose_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetKinematicsPose_Request>);
    fn op3_online_walking_module_msgs__srv__GetKinematicsPose_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetKinematicsPose_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<GetKinematicsPose_Request>) -> bool;
}

// Corresponds to op3_online_walking_module_msgs__srv__GetKinematicsPose_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetKinematicsPose_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub name: rosidl_runtime_rs::String,

}



impl Default for GetKinematicsPose_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !op3_online_walking_module_msgs__srv__GetKinematicsPose_Request__init(&mut msg as *mut _) {
        panic!("Call to op3_online_walking_module_msgs__srv__GetKinematicsPose_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetKinematicsPose_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { op3_online_walking_module_msgs__srv__GetKinematicsPose_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { op3_online_walking_module_msgs__srv__GetKinematicsPose_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { op3_online_walking_module_msgs__srv__GetKinematicsPose_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetKinematicsPose_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetKinematicsPose_Request where Self: Sized {
  const TYPE_NAME: &'static str = "op3_online_walking_module_msgs/srv/GetKinematicsPose_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__op3_online_walking_module_msgs__srv__GetKinematicsPose_Request() }
  }
}


#[link(name = "op3_online_walking_module_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__op3_online_walking_module_msgs__srv__GetKinematicsPose_Response() -> *const std::ffi::c_void;
}

#[link(name = "op3_online_walking_module_msgs__rosidl_generator_c")]
extern "C" {
    fn op3_online_walking_module_msgs__srv__GetKinematicsPose_Response__init(msg: *mut GetKinematicsPose_Response) -> bool;
    fn op3_online_walking_module_msgs__srv__GetKinematicsPose_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetKinematicsPose_Response>, size: usize) -> bool;
    fn op3_online_walking_module_msgs__srv__GetKinematicsPose_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetKinematicsPose_Response>);
    fn op3_online_walking_module_msgs__srv__GetKinematicsPose_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetKinematicsPose_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<GetKinematicsPose_Response>) -> bool;
}

// Corresponds to op3_online_walking_module_msgs__srv__GetKinematicsPose_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetKinematicsPose_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub pose: super::super::msg::rmw::KinematicsPose,

}



impl Default for GetKinematicsPose_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !op3_online_walking_module_msgs__srv__GetKinematicsPose_Response__init(&mut msg as *mut _) {
        panic!("Call to op3_online_walking_module_msgs__srv__GetKinematicsPose_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetKinematicsPose_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { op3_online_walking_module_msgs__srv__GetKinematicsPose_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { op3_online_walking_module_msgs__srv__GetKinematicsPose_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { op3_online_walking_module_msgs__srv__GetKinematicsPose_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetKinematicsPose_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetKinematicsPose_Response where Self: Sized {
  const TYPE_NAME: &'static str = "op3_online_walking_module_msgs/srv/GetKinematicsPose_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__op3_online_walking_module_msgs__srv__GetKinematicsPose_Response() }
  }
}


#[link(name = "op3_online_walking_module_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__op3_online_walking_module_msgs__srv__GetPreviewMatrix_Request() -> *const std::ffi::c_void;
}

#[link(name = "op3_online_walking_module_msgs__rosidl_generator_c")]
extern "C" {
    fn op3_online_walking_module_msgs__srv__GetPreviewMatrix_Request__init(msg: *mut GetPreviewMatrix_Request) -> bool;
    fn op3_online_walking_module_msgs__srv__GetPreviewMatrix_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetPreviewMatrix_Request>, size: usize) -> bool;
    fn op3_online_walking_module_msgs__srv__GetPreviewMatrix_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetPreviewMatrix_Request>);
    fn op3_online_walking_module_msgs__srv__GetPreviewMatrix_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetPreviewMatrix_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<GetPreviewMatrix_Request>) -> bool;
}

// Corresponds to op3_online_walking_module_msgs__srv__GetPreviewMatrix_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetPreviewMatrix_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub req: super::super::msg::rmw::PreviewRequest,

}



impl Default for GetPreviewMatrix_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !op3_online_walking_module_msgs__srv__GetPreviewMatrix_Request__init(&mut msg as *mut _) {
        panic!("Call to op3_online_walking_module_msgs__srv__GetPreviewMatrix_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetPreviewMatrix_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { op3_online_walking_module_msgs__srv__GetPreviewMatrix_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { op3_online_walking_module_msgs__srv__GetPreviewMatrix_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { op3_online_walking_module_msgs__srv__GetPreviewMatrix_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetPreviewMatrix_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetPreviewMatrix_Request where Self: Sized {
  const TYPE_NAME: &'static str = "op3_online_walking_module_msgs/srv/GetPreviewMatrix_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__op3_online_walking_module_msgs__srv__GetPreviewMatrix_Request() }
  }
}


#[link(name = "op3_online_walking_module_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__op3_online_walking_module_msgs__srv__GetPreviewMatrix_Response() -> *const std::ffi::c_void;
}

#[link(name = "op3_online_walking_module_msgs__rosidl_generator_c")]
extern "C" {
    fn op3_online_walking_module_msgs__srv__GetPreviewMatrix_Response__init(msg: *mut GetPreviewMatrix_Response) -> bool;
    fn op3_online_walking_module_msgs__srv__GetPreviewMatrix_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetPreviewMatrix_Response>, size: usize) -> bool;
    fn op3_online_walking_module_msgs__srv__GetPreviewMatrix_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetPreviewMatrix_Response>);
    fn op3_online_walking_module_msgs__srv__GetPreviewMatrix_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetPreviewMatrix_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<GetPreviewMatrix_Response>) -> bool;
}

// Corresponds to op3_online_walking_module_msgs__srv__GetPreviewMatrix_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetPreviewMatrix_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: super::super::msg::rmw::PreviewResponse,

}



impl Default for GetPreviewMatrix_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !op3_online_walking_module_msgs__srv__GetPreviewMatrix_Response__init(&mut msg as *mut _) {
        panic!("Call to op3_online_walking_module_msgs__srv__GetPreviewMatrix_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetPreviewMatrix_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { op3_online_walking_module_msgs__srv__GetPreviewMatrix_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { op3_online_walking_module_msgs__srv__GetPreviewMatrix_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { op3_online_walking_module_msgs__srv__GetPreviewMatrix_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetPreviewMatrix_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetPreviewMatrix_Response where Self: Sized {
  const TYPE_NAME: &'static str = "op3_online_walking_module_msgs/srv/GetPreviewMatrix_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__op3_online_walking_module_msgs__srv__GetPreviewMatrix_Response() }
  }
}






#[link(name = "op3_online_walking_module_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__op3_online_walking_module_msgs__srv__GetJointPose() -> *const std::ffi::c_void;
}

// Corresponds to op3_online_walking_module_msgs__srv__GetJointPose
#[allow(missing_docs, non_camel_case_types)]
pub struct GetJointPose;

impl rosidl_runtime_rs::Service for GetJointPose {
    type Request = GetJointPose_Request;
    type Response = GetJointPose_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__op3_online_walking_module_msgs__srv__GetJointPose() }
    }
}




#[link(name = "op3_online_walking_module_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__op3_online_walking_module_msgs__srv__GetKinematicsPose() -> *const std::ffi::c_void;
}

// Corresponds to op3_online_walking_module_msgs__srv__GetKinematicsPose
#[allow(missing_docs, non_camel_case_types)]
pub struct GetKinematicsPose;

impl rosidl_runtime_rs::Service for GetKinematicsPose {
    type Request = GetKinematicsPose_Request;
    type Response = GetKinematicsPose_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__op3_online_walking_module_msgs__srv__GetKinematicsPose() }
    }
}




#[link(name = "op3_online_walking_module_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__op3_online_walking_module_msgs__srv__GetPreviewMatrix() -> *const std::ffi::c_void;
}

// Corresponds to op3_online_walking_module_msgs__srv__GetPreviewMatrix
#[allow(missing_docs, non_camel_case_types)]
pub struct GetPreviewMatrix;

impl rosidl_runtime_rs::Service for GetPreviewMatrix {
    type Request = GetPreviewMatrix_Request;
    type Response = GetPreviewMatrix_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__op3_online_walking_module_msgs__srv__GetPreviewMatrix() }
    }
}


