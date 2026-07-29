#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};




// Corresponds to op3_walking_module_msgs__srv__GetWalkingParam_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetWalkingParam_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub get_param: bool,

}



impl Default for GetWalkingParam_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GetWalkingParam_Request::default())
  }
}

impl rosidl_runtime_rs::Message for GetWalkingParam_Request {
  type RmwMsg = super::srv::rmw::GetWalkingParam_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        get_param: msg.get_param,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      get_param: msg.get_param,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      get_param: msg.get_param,
    }
  }
}


// Corresponds to op3_walking_module_msgs__srv__GetWalkingParam_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetWalkingParam_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub parameters: super::msg::WalkingParam,

}



impl Default for GetWalkingParam_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GetWalkingParam_Response::default())
  }
}

impl rosidl_runtime_rs::Message for GetWalkingParam_Response {
  type RmwMsg = super::srv::rmw::GetWalkingParam_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        parameters: super::msg::WalkingParam::into_rmw_message(std::borrow::Cow::Owned(msg.parameters)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        parameters: super::msg::WalkingParam::into_rmw_message(std::borrow::Cow::Borrowed(&msg.parameters)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      parameters: super::msg::WalkingParam::from_rmw_message(msg.parameters),
    }
  }
}


// Corresponds to op3_walking_module_msgs__srv__SetWalkingParam_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetWalkingParam_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub parameters: super::msg::WalkingParam,

}



impl Default for SetWalkingParam_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::SetWalkingParam_Request::default())
  }
}

impl rosidl_runtime_rs::Message for SetWalkingParam_Request {
  type RmwMsg = super::srv::rmw::SetWalkingParam_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        parameters: super::msg::WalkingParam::into_rmw_message(std::borrow::Cow::Owned(msg.parameters)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        parameters: super::msg::WalkingParam::into_rmw_message(std::borrow::Cow::Borrowed(&msg.parameters)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      parameters: super::msg::WalkingParam::from_rmw_message(msg.parameters),
    }
  }
}


// Corresponds to op3_walking_module_msgs__srv__SetWalkingParam_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetWalkingParam_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub result: bool,

}



impl Default for SetWalkingParam_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::SetWalkingParam_Response::default())
  }
}

impl rosidl_runtime_rs::Message for SetWalkingParam_Response {
  type RmwMsg = super::srv::rmw::SetWalkingParam_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        result: msg.result,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      result: msg.result,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      result: msg.result,
    }
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


