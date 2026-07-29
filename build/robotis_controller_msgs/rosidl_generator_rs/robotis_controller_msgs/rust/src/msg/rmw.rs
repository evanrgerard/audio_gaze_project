#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};


#[link(name = "robotis_controller_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__robotis_controller_msgs__msg__SyncWriteItem() -> *const std::ffi::c_void;
}

#[link(name = "robotis_controller_msgs__rosidl_generator_c")]
extern "C" {
    fn robotis_controller_msgs__msg__SyncWriteItem__init(msg: *mut SyncWriteItem) -> bool;
    fn robotis_controller_msgs__msg__SyncWriteItem__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SyncWriteItem>, size: usize) -> bool;
    fn robotis_controller_msgs__msg__SyncWriteItem__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SyncWriteItem>);
    fn robotis_controller_msgs__msg__SyncWriteItem__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SyncWriteItem>, out_seq: *mut rosidl_runtime_rs::Sequence<SyncWriteItem>) -> bool;
}

// Corresponds to robotis_controller_msgs__msg__SyncWriteItem
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SyncWriteItem {

    // This member is not documented.
    #[allow(missing_docs)]
    pub item_name: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub joint_name: rosidl_runtime_rs::Sequence<rosidl_runtime_rs::String>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub value: rosidl_runtime_rs::Sequence<u32>,

}



impl Default for SyncWriteItem {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !robotis_controller_msgs__msg__SyncWriteItem__init(&mut msg as *mut _) {
        panic!("Call to robotis_controller_msgs__msg__SyncWriteItem__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SyncWriteItem {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robotis_controller_msgs__msg__SyncWriteItem__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robotis_controller_msgs__msg__SyncWriteItem__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robotis_controller_msgs__msg__SyncWriteItem__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SyncWriteItem {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SyncWriteItem where Self: Sized {
  const TYPE_NAME: &'static str = "robotis_controller_msgs/msg/SyncWriteItem";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__robotis_controller_msgs__msg__SyncWriteItem() }
  }
}


#[link(name = "robotis_controller_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__robotis_controller_msgs__msg__JointCtrlModule() -> *const std::ffi::c_void;
}

#[link(name = "robotis_controller_msgs__rosidl_generator_c")]
extern "C" {
    fn robotis_controller_msgs__msg__JointCtrlModule__init(msg: *mut JointCtrlModule) -> bool;
    fn robotis_controller_msgs__msg__JointCtrlModule__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<JointCtrlModule>, size: usize) -> bool;
    fn robotis_controller_msgs__msg__JointCtrlModule__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<JointCtrlModule>);
    fn robotis_controller_msgs__msg__JointCtrlModule__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<JointCtrlModule>, out_seq: *mut rosidl_runtime_rs::Sequence<JointCtrlModule>) -> bool;
}

// Corresponds to robotis_controller_msgs__msg__JointCtrlModule
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct JointCtrlModule {

    // This member is not documented.
    #[allow(missing_docs)]
    pub joint_name: rosidl_runtime_rs::Sequence<rosidl_runtime_rs::String>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub module_name: rosidl_runtime_rs::Sequence<rosidl_runtime_rs::String>,

}



impl Default for JointCtrlModule {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !robotis_controller_msgs__msg__JointCtrlModule__init(&mut msg as *mut _) {
        panic!("Call to robotis_controller_msgs__msg__JointCtrlModule__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for JointCtrlModule {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robotis_controller_msgs__msg__JointCtrlModule__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robotis_controller_msgs__msg__JointCtrlModule__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robotis_controller_msgs__msg__JointCtrlModule__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for JointCtrlModule {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for JointCtrlModule where Self: Sized {
  const TYPE_NAME: &'static str = "robotis_controller_msgs/msg/JointCtrlModule";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__robotis_controller_msgs__msg__JointCtrlModule() }
  }
}


#[link(name = "robotis_controller_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__robotis_controller_msgs__msg__StatusMsg() -> *const std::ffi::c_void;
}

#[link(name = "robotis_controller_msgs__rosidl_generator_c")]
extern "C" {
    fn robotis_controller_msgs__msg__StatusMsg__init(msg: *mut StatusMsg) -> bool;
    fn robotis_controller_msgs__msg__StatusMsg__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<StatusMsg>, size: usize) -> bool;
    fn robotis_controller_msgs__msg__StatusMsg__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<StatusMsg>);
    fn robotis_controller_msgs__msg__StatusMsg__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<StatusMsg>, out_seq: *mut rosidl_runtime_rs::Sequence<StatusMsg>) -> bool;
}

// Corresponds to robotis_controller_msgs__msg__StatusMsg
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// Status Constants

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct StatusMsg {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::rmw::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub type_: u8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub module_name: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub status_msg: rosidl_runtime_rs::String,

}

impl StatusMsg {

    // This constant is not documented.
    #[allow(missing_docs)]
    pub const STATUS_UNKNOWN: u8 = 0;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const STATUS_INFO: u8 = 1;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const STATUS_WARN: u8 = 2;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const STATUS_ERROR: u8 = 3;

}


impl Default for StatusMsg {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !robotis_controller_msgs__msg__StatusMsg__init(&mut msg as *mut _) {
        panic!("Call to robotis_controller_msgs__msg__StatusMsg__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for StatusMsg {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robotis_controller_msgs__msg__StatusMsg__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robotis_controller_msgs__msg__StatusMsg__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robotis_controller_msgs__msg__StatusMsg__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for StatusMsg {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for StatusMsg where Self: Sized {
  const TYPE_NAME: &'static str = "robotis_controller_msgs/msg/StatusMsg";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__robotis_controller_msgs__msg__StatusMsg() }
  }
}


#[link(name = "robotis_controller_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__robotis_controller_msgs__msg__WriteControlTable() -> *const std::ffi::c_void;
}

#[link(name = "robotis_controller_msgs__rosidl_generator_c")]
extern "C" {
    fn robotis_controller_msgs__msg__WriteControlTable__init(msg: *mut WriteControlTable) -> bool;
    fn robotis_controller_msgs__msg__WriteControlTable__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<WriteControlTable>, size: usize) -> bool;
    fn robotis_controller_msgs__msg__WriteControlTable__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<WriteControlTable>);
    fn robotis_controller_msgs__msg__WriteControlTable__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<WriteControlTable>, out_seq: *mut rosidl_runtime_rs::Sequence<WriteControlTable>) -> bool;
}

// Corresponds to robotis_controller_msgs__msg__WriteControlTable
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct WriteControlTable {

    // This member is not documented.
    #[allow(missing_docs)]
    pub joint_name: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub start_item_name: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub data_length: u16,


    // This member is not documented.
    #[allow(missing_docs)]
    pub data: rosidl_runtime_rs::Sequence<u8>,

}



impl Default for WriteControlTable {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !robotis_controller_msgs__msg__WriteControlTable__init(&mut msg as *mut _) {
        panic!("Call to robotis_controller_msgs__msg__WriteControlTable__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for WriteControlTable {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robotis_controller_msgs__msg__WriteControlTable__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robotis_controller_msgs__msg__WriteControlTable__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robotis_controller_msgs__msg__WriteControlTable__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for WriteControlTable {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for WriteControlTable where Self: Sized {
  const TYPE_NAME: &'static str = "robotis_controller_msgs/msg/WriteControlTable";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__robotis_controller_msgs__msg__WriteControlTable() }
  }
}


