#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};




// Corresponds to robotis_controller_msgs__srv__GetJointModule_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetJointModule_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub joint_name: Vec<std::string::String>,

}



impl Default for GetJointModule_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GetJointModule_Request::default())
  }
}

impl rosidl_runtime_rs::Message for GetJointModule_Request {
  type RmwMsg = super::srv::rmw::GetJointModule_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        joint_name: msg.joint_name
          .into_iter()
          .map(|elem| elem.as_str().into())
          .collect(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        joint_name: msg.joint_name
          .iter()
          .map(|elem| elem.as_str().into())
          .collect(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      joint_name: msg.joint_name
          .into_iter()
          .map(|elem| elem.to_string())
          .collect(),
    }
  }
}


// Corresponds to robotis_controller_msgs__srv__GetJointModule_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetJointModule_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub joint_name: Vec<std::string::String>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub module_name: Vec<std::string::String>,

}



impl Default for GetJointModule_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GetJointModule_Response::default())
  }
}

impl rosidl_runtime_rs::Message for GetJointModule_Response {
  type RmwMsg = super::srv::rmw::GetJointModule_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        joint_name: msg.joint_name
          .into_iter()
          .map(|elem| elem.as_str().into())
          .collect(),
        module_name: msg.module_name
          .into_iter()
          .map(|elem| elem.as_str().into())
          .collect(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        joint_name: msg.joint_name
          .iter()
          .map(|elem| elem.as_str().into())
          .collect(),
        module_name: msg.module_name
          .iter()
          .map(|elem| elem.as_str().into())
          .collect(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      joint_name: msg.joint_name
          .into_iter()
          .map(|elem| elem.to_string())
          .collect(),
      module_name: msg.module_name
          .into_iter()
          .map(|elem| elem.to_string())
          .collect(),
    }
  }
}


// Corresponds to robotis_controller_msgs__srv__SetJointModule_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetJointModule_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub joint_name: Vec<std::string::String>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub module_name: Vec<std::string::String>,

}



impl Default for SetJointModule_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::SetJointModule_Request::default())
  }
}

impl rosidl_runtime_rs::Message for SetJointModule_Request {
  type RmwMsg = super::srv::rmw::SetJointModule_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        joint_name: msg.joint_name
          .into_iter()
          .map(|elem| elem.as_str().into())
          .collect(),
        module_name: msg.module_name
          .into_iter()
          .map(|elem| elem.as_str().into())
          .collect(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        joint_name: msg.joint_name
          .iter()
          .map(|elem| elem.as_str().into())
          .collect(),
        module_name: msg.module_name
          .iter()
          .map(|elem| elem.as_str().into())
          .collect(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      joint_name: msg.joint_name
          .into_iter()
          .map(|elem| elem.to_string())
          .collect(),
      module_name: msg.module_name
          .into_iter()
          .map(|elem| elem.to_string())
          .collect(),
    }
  }
}


// Corresponds to robotis_controller_msgs__srv__SetJointModule_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetJointModule_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub result: bool,

}



impl Default for SetJointModule_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::SetJointModule_Response::default())
  }
}

impl rosidl_runtime_rs::Message for SetJointModule_Response {
  type RmwMsg = super::srv::rmw::SetJointModule_Response;

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


// Corresponds to robotis_controller_msgs__srv__SetModule_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetModule_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub module_name: std::string::String,

}



impl Default for SetModule_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::SetModule_Request::default())
  }
}

impl rosidl_runtime_rs::Message for SetModule_Request {
  type RmwMsg = super::srv::rmw::SetModule_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        module_name: msg.module_name.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        module_name: msg.module_name.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      module_name: msg.module_name.to_string(),
    }
  }
}


// Corresponds to robotis_controller_msgs__srv__SetModule_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetModule_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub result: bool,

}



impl Default for SetModule_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::SetModule_Response::default())
  }
}

impl rosidl_runtime_rs::Message for SetModule_Response {
  type RmwMsg = super::srv::rmw::SetModule_Response;

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


// Corresponds to robotis_controller_msgs__srv__LoadOffset_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct LoadOffset_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub file_path: std::string::String,

}



impl Default for LoadOffset_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::LoadOffset_Request::default())
  }
}

impl rosidl_runtime_rs::Message for LoadOffset_Request {
  type RmwMsg = super::srv::rmw::LoadOffset_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        file_path: msg.file_path.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        file_path: msg.file_path.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      file_path: msg.file_path.to_string(),
    }
  }
}


// Corresponds to robotis_controller_msgs__srv__LoadOffset_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct LoadOffset_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub result: bool,

}



impl Default for LoadOffset_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::LoadOffset_Response::default())
  }
}

impl rosidl_runtime_rs::Message for LoadOffset_Response {
  type RmwMsg = super::srv::rmw::LoadOffset_Response;

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


