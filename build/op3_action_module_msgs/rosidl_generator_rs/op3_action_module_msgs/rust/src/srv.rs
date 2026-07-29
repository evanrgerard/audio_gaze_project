#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};




// Corresponds to op3_action_module_msgs__srv__IsRunning_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct IsRunning_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for IsRunning_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::IsRunning_Request::default())
  }
}

impl rosidl_runtime_rs::Message for IsRunning_Request {
  type RmwMsg = super::srv::rmw::IsRunning_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
    }
  }
}


// Corresponds to op3_action_module_msgs__srv__IsRunning_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct IsRunning_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub is_running: bool,

}



impl Default for IsRunning_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::IsRunning_Response::default())
  }
}

impl rosidl_runtime_rs::Message for IsRunning_Response {
  type RmwMsg = super::srv::rmw::IsRunning_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        is_running: msg.is_running,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      is_running: msg.is_running,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      is_running: msg.is_running,
    }
  }
}






#[link(name = "op3_action_module_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__op3_action_module_msgs__srv__IsRunning() -> *const std::ffi::c_void;
}

// Corresponds to op3_action_module_msgs__srv__IsRunning
#[allow(missing_docs, non_camel_case_types)]
pub struct IsRunning;

impl rosidl_runtime_rs::Service for IsRunning {
    type Request = IsRunning_Request;
    type Response = IsRunning_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__op3_action_module_msgs__srv__IsRunning() }
    }
}


