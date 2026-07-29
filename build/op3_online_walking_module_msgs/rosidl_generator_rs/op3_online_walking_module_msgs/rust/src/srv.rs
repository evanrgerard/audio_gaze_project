#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};




// Corresponds to op3_online_walking_module_msgs__srv__GetJointPose_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetJointPose_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for GetJointPose_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GetJointPose_Request::default())
  }
}

impl rosidl_runtime_rs::Message for GetJointPose_Request {
  type RmwMsg = super::srv::rmw::GetJointPose_Request;

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


// Corresponds to op3_online_walking_module_msgs__srv__GetJointPose_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetJointPose_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub pose: super::msg::JointPose,

}



impl Default for GetJointPose_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GetJointPose_Response::default())
  }
}

impl rosidl_runtime_rs::Message for GetJointPose_Response {
  type RmwMsg = super::srv::rmw::GetJointPose_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        pose: super::msg::JointPose::into_rmw_message(std::borrow::Cow::Owned(msg.pose)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        pose: super::msg::JointPose::into_rmw_message(std::borrow::Cow::Borrowed(&msg.pose)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      pose: super::msg::JointPose::from_rmw_message(msg.pose),
    }
  }
}


// Corresponds to op3_online_walking_module_msgs__srv__GetKinematicsPose_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetKinematicsPose_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub name: std::string::String,

}



impl Default for GetKinematicsPose_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GetKinematicsPose_Request::default())
  }
}

impl rosidl_runtime_rs::Message for GetKinematicsPose_Request {
  type RmwMsg = super::srv::rmw::GetKinematicsPose_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        name: msg.name.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        name: msg.name.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      name: msg.name.to_string(),
    }
  }
}


// Corresponds to op3_online_walking_module_msgs__srv__GetKinematicsPose_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetKinematicsPose_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub pose: super::msg::KinematicsPose,

}



impl Default for GetKinematicsPose_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GetKinematicsPose_Response::default())
  }
}

impl rosidl_runtime_rs::Message for GetKinematicsPose_Response {
  type RmwMsg = super::srv::rmw::GetKinematicsPose_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        pose: super::msg::KinematicsPose::into_rmw_message(std::borrow::Cow::Owned(msg.pose)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        pose: super::msg::KinematicsPose::into_rmw_message(std::borrow::Cow::Borrowed(&msg.pose)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      pose: super::msg::KinematicsPose::from_rmw_message(msg.pose),
    }
  }
}


// Corresponds to op3_online_walking_module_msgs__srv__GetPreviewMatrix_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetPreviewMatrix_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub req: super::msg::PreviewRequest,

}



impl Default for GetPreviewMatrix_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GetPreviewMatrix_Request::default())
  }
}

impl rosidl_runtime_rs::Message for GetPreviewMatrix_Request {
  type RmwMsg = super::srv::rmw::GetPreviewMatrix_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        req: super::msg::PreviewRequest::into_rmw_message(std::borrow::Cow::Owned(msg.req)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        req: super::msg::PreviewRequest::into_rmw_message(std::borrow::Cow::Borrowed(&msg.req)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      req: super::msg::PreviewRequest::from_rmw_message(msg.req),
    }
  }
}


// Corresponds to op3_online_walking_module_msgs__srv__GetPreviewMatrix_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetPreviewMatrix_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: super::msg::PreviewResponse,

}



impl Default for GetPreviewMatrix_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GetPreviewMatrix_Response::default())
  }
}

impl rosidl_runtime_rs::Message for GetPreviewMatrix_Response {
  type RmwMsg = super::srv::rmw::GetPreviewMatrix_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        res: super::msg::PreviewResponse::into_rmw_message(std::borrow::Cow::Owned(msg.res)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        res: super::msg::PreviewResponse::into_rmw_message(std::borrow::Cow::Borrowed(&msg.res)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      res: super::msg::PreviewResponse::from_rmw_message(msg.res),
    }
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


