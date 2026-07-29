#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};



// Corresponds to op3_action_module_msgs__msg__StartAction

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct StartAction {

    // This member is not documented.
    #[allow(missing_docs)]
    pub page_num: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub joint_name_array: Vec<std::string::String>,

}



impl Default for StartAction {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::StartAction::default())
  }
}

impl rosidl_runtime_rs::Message for StartAction {
  type RmwMsg = super::msg::rmw::StartAction;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        page_num: msg.page_num,
        joint_name_array: msg.joint_name_array
          .into_iter()
          .map(|elem| elem.as_str().into())
          .collect(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      page_num: msg.page_num,
        joint_name_array: msg.joint_name_array
          .iter()
          .map(|elem| elem.as_str().into())
          .collect(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      page_num: msg.page_num,
      joint_name_array: msg.joint_name_array
          .into_iter()
          .map(|elem| elem.to_string())
          .collect(),
    }
  }
}


