#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};




// Corresponds to op3_offset_tuner_msgs__srv__GetPresentJointOffsetData_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetPresentJointOffsetData_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for GetPresentJointOffsetData_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GetPresentJointOffsetData_Request::default())
  }
}

impl rosidl_runtime_rs::Message for GetPresentJointOffsetData_Request {
  type RmwMsg = super::srv::rmw::GetPresentJointOffsetData_Request;

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


// Corresponds to op3_offset_tuner_msgs__srv__GetPresentJointOffsetData_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetPresentJointOffsetData_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub present_data_array: Vec<super::msg::JointOffsetPositionData>,

}



impl Default for GetPresentJointOffsetData_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GetPresentJointOffsetData_Response::default())
  }
}

impl rosidl_runtime_rs::Message for GetPresentJointOffsetData_Response {
  type RmwMsg = super::srv::rmw::GetPresentJointOffsetData_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        present_data_array: msg.present_data_array
          .into_iter()
          .map(|elem| super::msg::JointOffsetPositionData::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        present_data_array: msg.present_data_array
          .iter()
          .map(|elem| super::msg::JointOffsetPositionData::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      present_data_array: msg.present_data_array
          .into_iter()
          .map(super::msg::JointOffsetPositionData::from_rmw_message)
          .collect(),
    }
  }
}






#[link(name = "op3_offset_tuner_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__op3_offset_tuner_msgs__srv__GetPresentJointOffsetData() -> *const std::ffi::c_void;
}

// Corresponds to op3_offset_tuner_msgs__srv__GetPresentJointOffsetData
#[allow(missing_docs, non_camel_case_types)]
pub struct GetPresentJointOffsetData;

impl rosidl_runtime_rs::Service for GetPresentJointOffsetData {
    type Request = GetPresentJointOffsetData_Request;
    type Response = GetPresentJointOffsetData_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__op3_offset_tuner_msgs__srv__GetPresentJointOffsetData() }
    }
}


