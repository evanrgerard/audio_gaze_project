#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};



// Corresponds to op3_tuning_module_msgs__msg__JointOffsetData

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct JointOffsetData {

    // This member is not documented.
    #[allow(missing_docs)]
    pub joint_name: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_value: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub offset_value: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub p_gain: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub i_gain: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub d_gain: i32,

}



impl Default for JointOffsetData {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::JointOffsetData::default())
  }
}

impl rosidl_runtime_rs::Message for JointOffsetData {
  type RmwMsg = super::msg::rmw::JointOffsetData;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        joint_name: msg.joint_name.as_str().into(),
        goal_value: msg.goal_value,
        offset_value: msg.offset_value,
        p_gain: msg.p_gain,
        i_gain: msg.i_gain,
        d_gain: msg.d_gain,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        joint_name: msg.joint_name.as_str().into(),
      goal_value: msg.goal_value,
      offset_value: msg.offset_value,
      p_gain: msg.p_gain,
      i_gain: msg.i_gain,
      d_gain: msg.d_gain,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      joint_name: msg.joint_name.to_string(),
      goal_value: msg.goal_value,
      offset_value: msg.offset_value,
      p_gain: msg.p_gain,
      i_gain: msg.i_gain,
      d_gain: msg.d_gain,
    }
  }
}


// Corresponds to op3_tuning_module_msgs__msg__JointOffsetPositionData

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct JointOffsetPositionData {

    // This member is not documented.
    #[allow(missing_docs)]
    pub joint_name: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_value: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub offset_value: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub present_value: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub p_gain: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub i_gain: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub d_gain: i32,

}



impl Default for JointOffsetPositionData {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::JointOffsetPositionData::default())
  }
}

impl rosidl_runtime_rs::Message for JointOffsetPositionData {
  type RmwMsg = super::msg::rmw::JointOffsetPositionData;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        joint_name: msg.joint_name.as_str().into(),
        goal_value: msg.goal_value,
        offset_value: msg.offset_value,
        present_value: msg.present_value,
        p_gain: msg.p_gain,
        i_gain: msg.i_gain,
        d_gain: msg.d_gain,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        joint_name: msg.joint_name.as_str().into(),
      goal_value: msg.goal_value,
      offset_value: msg.offset_value,
      present_value: msg.present_value,
      p_gain: msg.p_gain,
      i_gain: msg.i_gain,
      d_gain: msg.d_gain,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      joint_name: msg.joint_name.to_string(),
      goal_value: msg.goal_value,
      offset_value: msg.offset_value,
      present_value: msg.present_value,
      p_gain: msg.p_gain,
      i_gain: msg.i_gain,
      d_gain: msg.d_gain,
    }
  }
}


// Corresponds to op3_tuning_module_msgs__msg__JointTorqueOnOff

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct JointTorqueOnOff {

    // This member is not documented.
    #[allow(missing_docs)]
    pub joint_name: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub torque_enable: bool,

}



impl Default for JointTorqueOnOff {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::JointTorqueOnOff::default())
  }
}

impl rosidl_runtime_rs::Message for JointTorqueOnOff {
  type RmwMsg = super::msg::rmw::JointTorqueOnOff;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        joint_name: msg.joint_name.as_str().into(),
        torque_enable: msg.torque_enable,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        joint_name: msg.joint_name.as_str().into(),
      torque_enable: msg.torque_enable,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      joint_name: msg.joint_name.to_string(),
      torque_enable: msg.torque_enable,
    }
  }
}


// Corresponds to op3_tuning_module_msgs__msg__JointTorqueOnOffArray

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct JointTorqueOnOffArray {

    // This member is not documented.
    #[allow(missing_docs)]
    pub torque_enable_data: Vec<super::msg::JointTorqueOnOff>,

}



impl Default for JointTorqueOnOffArray {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::JointTorqueOnOffArray::default())
  }
}

impl rosidl_runtime_rs::Message for JointTorqueOnOffArray {
  type RmwMsg = super::msg::rmw::JointTorqueOnOffArray;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        torque_enable_data: msg.torque_enable_data
          .into_iter()
          .map(|elem| super::msg::JointTorqueOnOff::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        torque_enable_data: msg.torque_enable_data
          .iter()
          .map(|elem| super::msg::JointTorqueOnOff::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      torque_enable_data: msg.torque_enable_data
          .into_iter()
          .map(super::msg::JointTorqueOnOff::from_rmw_message)
          .collect(),
    }
  }
}


