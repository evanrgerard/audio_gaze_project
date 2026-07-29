#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};



// Corresponds to robotis_controller_msgs__msg__SyncWriteItem

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SyncWriteItem {

    // This member is not documented.
    #[allow(missing_docs)]
    pub item_name: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub joint_name: Vec<std::string::String>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub value: Vec<u32>,

}



impl Default for SyncWriteItem {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::SyncWriteItem::default())
  }
}

impl rosidl_runtime_rs::Message for SyncWriteItem {
  type RmwMsg = super::msg::rmw::SyncWriteItem;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        item_name: msg.item_name.as_str().into(),
        joint_name: msg.joint_name
          .into_iter()
          .map(|elem| elem.as_str().into())
          .collect(),
        value: msg.value.into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        item_name: msg.item_name.as_str().into(),
        joint_name: msg.joint_name
          .iter()
          .map(|elem| elem.as_str().into())
          .collect(),
        value: msg.value.as_slice().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      item_name: msg.item_name.to_string(),
      joint_name: msg.joint_name
          .into_iter()
          .map(|elem| elem.to_string())
          .collect(),
      value: msg.value
          .into_iter()
          .collect(),
    }
  }
}


// Corresponds to robotis_controller_msgs__msg__JointCtrlModule

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct JointCtrlModule {

    // This member is not documented.
    #[allow(missing_docs)]
    pub joint_name: Vec<std::string::String>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub module_name: Vec<std::string::String>,

}



impl Default for JointCtrlModule {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::JointCtrlModule::default())
  }
}

impl rosidl_runtime_rs::Message for JointCtrlModule {
  type RmwMsg = super::msg::rmw::JointCtrlModule;

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


// Corresponds to robotis_controller_msgs__msg__StatusMsg
/// Status Constants

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct StatusMsg {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub type_: u8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub module_name: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub status_msg: std::string::String,

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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::StatusMsg::default())
  }
}

impl rosidl_runtime_rs::Message for StatusMsg {
  type RmwMsg = super::msg::rmw::StatusMsg;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        type_: msg.type_,
        module_name: msg.module_name.as_str().into(),
        status_msg: msg.status_msg.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
      type_: msg.type_,
        module_name: msg.module_name.as_str().into(),
        status_msg: msg.status_msg.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      type_: msg.type_,
      module_name: msg.module_name.to_string(),
      status_msg: msg.status_msg.to_string(),
    }
  }
}


// Corresponds to robotis_controller_msgs__msg__WriteControlTable

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct WriteControlTable {

    // This member is not documented.
    #[allow(missing_docs)]
    pub joint_name: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub start_item_name: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub data_length: u16,


    // This member is not documented.
    #[allow(missing_docs)]
    pub data: Vec<u8>,

}



impl Default for WriteControlTable {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::WriteControlTable::default())
  }
}

impl rosidl_runtime_rs::Message for WriteControlTable {
  type RmwMsg = super::msg::rmw::WriteControlTable;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        joint_name: msg.joint_name.as_str().into(),
        start_item_name: msg.start_item_name.as_str().into(),
        data_length: msg.data_length,
        data: msg.data.into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        joint_name: msg.joint_name.as_str().into(),
        start_item_name: msg.start_item_name.as_str().into(),
      data_length: msg.data_length,
        data: msg.data.as_slice().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      joint_name: msg.joint_name.to_string(),
      start_item_name: msg.start_item_name.to_string(),
      data_length: msg.data_length,
      data: msg.data
          .into_iter()
          .collect(),
    }
  }
}


