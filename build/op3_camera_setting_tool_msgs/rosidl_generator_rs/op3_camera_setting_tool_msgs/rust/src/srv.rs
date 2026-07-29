#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};




// Corresponds to op3_camera_setting_tool_msgs__srv__GetParameters_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetParameters_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for GetParameters_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GetParameters_Request::default())
  }
}

impl rosidl_runtime_rs::Message for GetParameters_Request {
  type RmwMsg = super::srv::rmw::GetParameters_Request;

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


// Corresponds to op3_camera_setting_tool_msgs__srv__GetParameters_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetParameters_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub returns: super::msg::CameraParams,

}



impl Default for GetParameters_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GetParameters_Response::default())
  }
}

impl rosidl_runtime_rs::Message for GetParameters_Response {
  type RmwMsg = super::srv::rmw::GetParameters_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        returns: super::msg::CameraParams::into_rmw_message(std::borrow::Cow::Owned(msg.returns)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        returns: super::msg::CameraParams::into_rmw_message(std::borrow::Cow::Borrowed(&msg.returns)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      returns: super::msg::CameraParams::from_rmw_message(msg.returns),
    }
  }
}


// Corresponds to op3_camera_setting_tool_msgs__srv__SetParameters_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetParameters_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub params: super::msg::CameraParams,

}



impl Default for SetParameters_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::SetParameters_Request::default())
  }
}

impl rosidl_runtime_rs::Message for SetParameters_Request {
  type RmwMsg = super::srv::rmw::SetParameters_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        params: super::msg::CameraParams::into_rmw_message(std::borrow::Cow::Owned(msg.params)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        params: super::msg::CameraParams::into_rmw_message(std::borrow::Cow::Borrowed(&msg.params)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      params: super::msg::CameraParams::from_rmw_message(msg.params),
    }
  }
}


// Corresponds to op3_camera_setting_tool_msgs__srv__SetParameters_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetParameters_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub returns: super::msg::CameraParams,

}



impl Default for SetParameters_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::SetParameters_Response::default())
  }
}

impl rosidl_runtime_rs::Message for SetParameters_Response {
  type RmwMsg = super::srv::rmw::SetParameters_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        returns: super::msg::CameraParams::into_rmw_message(std::borrow::Cow::Owned(msg.returns)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        returns: super::msg::CameraParams::into_rmw_message(std::borrow::Cow::Borrowed(&msg.returns)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      returns: super::msg::CameraParams::from_rmw_message(msg.returns),
    }
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


