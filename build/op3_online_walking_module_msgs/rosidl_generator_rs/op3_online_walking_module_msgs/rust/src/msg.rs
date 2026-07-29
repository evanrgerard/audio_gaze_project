#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};



// Corresponds to op3_online_walking_module_msgs__msg__JointPose

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct JointPose {

    // This member is not documented.
    #[allow(missing_docs)]
    pub mov_time: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub pose: sensor_msgs::msg::JointState,

}



impl Default for JointPose {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::JointPose::default())
  }
}

impl rosidl_runtime_rs::Message for JointPose {
  type RmwMsg = super::msg::rmw::JointPose;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        mov_time: msg.mov_time,
        pose: sensor_msgs::msg::JointState::into_rmw_message(std::borrow::Cow::Owned(msg.pose)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      mov_time: msg.mov_time,
        pose: sensor_msgs::msg::JointState::into_rmw_message(std::borrow::Cow::Borrowed(&msg.pose)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      mov_time: msg.mov_time,
      pose: sensor_msgs::msg::JointState::from_rmw_message(msg.pose),
    }
  }
}


// Corresponds to op3_online_walking_module_msgs__msg__KinematicsPose

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct KinematicsPose {

    // This member is not documented.
    #[allow(missing_docs)]
    pub name: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub mov_time: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub pose: geometry_msgs::msg::Pose,

}



impl Default for KinematicsPose {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::KinematicsPose::default())
  }
}

impl rosidl_runtime_rs::Message for KinematicsPose {
  type RmwMsg = super::msg::rmw::KinematicsPose;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        name: msg.name.as_str().into(),
        mov_time: msg.mov_time,
        pose: geometry_msgs::msg::Pose::into_rmw_message(std::borrow::Cow::Owned(msg.pose)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        name: msg.name.as_str().into(),
      mov_time: msg.mov_time,
        pose: geometry_msgs::msg::Pose::into_rmw_message(std::borrow::Cow::Borrowed(&msg.pose)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      name: msg.name.to_string(),
      mov_time: msg.mov_time,
      pose: geometry_msgs::msg::Pose::from_rmw_message(msg.pose),
    }
  }
}


// Corresponds to op3_online_walking_module_msgs__msg__FootStepCommand

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct FootStepCommand {

    // This member is not documented.
    #[allow(missing_docs)]
    pub command: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub start_leg: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub step_num: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub step_time: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub step_length: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub side_length: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub step_angle: f64,

}



impl Default for FootStepCommand {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::FootStepCommand::default())
  }
}

impl rosidl_runtime_rs::Message for FootStepCommand {
  type RmwMsg = super::msg::rmw::FootStepCommand;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        command: msg.command.as_str().into(),
        start_leg: msg.start_leg.as_str().into(),
        step_num: msg.step_num,
        step_time: msg.step_time,
        step_length: msg.step_length,
        side_length: msg.side_length,
        step_angle: msg.step_angle,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        command: msg.command.as_str().into(),
        start_leg: msg.start_leg.as_str().into(),
      step_num: msg.step_num,
      step_time: msg.step_time,
      step_length: msg.step_length,
      side_length: msg.side_length,
      step_angle: msg.step_angle,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      command: msg.command.to_string(),
      start_leg: msg.start_leg.to_string(),
      step_num: msg.step_num,
      step_time: msg.step_time,
      step_length: msg.step_length,
      side_length: msg.side_length,
      step_angle: msg.step_angle,
    }
  }
}


// Corresponds to op3_online_walking_module_msgs__msg__FootStepArray

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct FootStepArray {

    // This member is not documented.
    #[allow(missing_docs)]
    pub moving_foot: Vec<i32>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub data: Vec<geometry_msgs::msg::Pose2D>,

}

impl FootStepArray {
    /// Left foot constant
    pub const LEFT_FOOT: i32 = 0;

    /// Right foot constant
    pub const RIGHT_FOOT: i32 = 1;

}


impl Default for FootStepArray {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::FootStepArray::default())
  }
}

impl rosidl_runtime_rs::Message for FootStepArray {
  type RmwMsg = super::msg::rmw::FootStepArray;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        moving_foot: msg.moving_foot.into(),
        data: msg.data
          .into_iter()
          .map(|elem| geometry_msgs::msg::Pose2D::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        moving_foot: msg.moving_foot.as_slice().into(),
        data: msg.data
          .iter()
          .map(|elem| geometry_msgs::msg::Pose2D::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      moving_foot: msg.moving_foot
          .into_iter()
          .collect(),
      data: msg.data
          .into_iter()
          .map(geometry_msgs::msg::Pose2D::from_rmw_message)
          .collect(),
    }
  }
}


// Corresponds to op3_online_walking_module_msgs__msg__PreviewRequest

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PreviewRequest {

    // This member is not documented.
    #[allow(missing_docs)]
    pub control_cycle: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub lipm_height: f64,

}



impl Default for PreviewRequest {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::PreviewRequest::default())
  }
}

impl rosidl_runtime_rs::Message for PreviewRequest {
  type RmwMsg = super::msg::rmw::PreviewRequest;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        control_cycle: msg.control_cycle,
        lipm_height: msg.lipm_height,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      control_cycle: msg.control_cycle,
      lipm_height: msg.lipm_height,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      control_cycle: msg.control_cycle,
      lipm_height: msg.lipm_height,
    }
  }
}


// Corresponds to op3_online_walking_module_msgs__msg__PreviewResponse

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PreviewResponse {

    // This member is not documented.
    #[allow(missing_docs)]
    pub k_row: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub k_col: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub k: Vec<f64>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub p_row: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub p_col: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub p: Vec<f64>,

}



impl Default for PreviewResponse {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::PreviewResponse::default())
  }
}

impl rosidl_runtime_rs::Message for PreviewResponse {
  type RmwMsg = super::msg::rmw::PreviewResponse;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        k_row: msg.k_row,
        k_col: msg.k_col,
        k: msg.k.into(),
        p_row: msg.p_row,
        p_col: msg.p_col,
        p: msg.p.into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      k_row: msg.k_row,
      k_col: msg.k_col,
        k: msg.k.as_slice().into(),
      p_row: msg.p_row,
      p_col: msg.p_col,
        p: msg.p.as_slice().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      k_row: msg.k_row,
      k_col: msg.k_col,
      k: msg.k
          .into_iter()
          .collect(),
      p_row: msg.p_row,
      p_col: msg.p_col,
      p: msg.p
          .into_iter()
          .collect(),
    }
  }
}


// Corresponds to op3_online_walking_module_msgs__msg__WalkingParam

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct WalkingParam {

    // This member is not documented.
    #[allow(missing_docs)]
    pub dsp_ratio: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub lipm_height: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub foot_height_max: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub zmp_offset_x: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub zmp_offset_y: f64,

}



impl Default for WalkingParam {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::WalkingParam::default())
  }
}

impl rosidl_runtime_rs::Message for WalkingParam {
  type RmwMsg = super::msg::rmw::WalkingParam;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        dsp_ratio: msg.dsp_ratio,
        lipm_height: msg.lipm_height,
        foot_height_max: msg.foot_height_max,
        zmp_offset_x: msg.zmp_offset_x,
        zmp_offset_y: msg.zmp_offset_y,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      dsp_ratio: msg.dsp_ratio,
      lipm_height: msg.lipm_height,
      foot_height_max: msg.foot_height_max,
      zmp_offset_x: msg.zmp_offset_x,
      zmp_offset_y: msg.zmp_offset_y,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      dsp_ratio: msg.dsp_ratio,
      lipm_height: msg.lipm_height,
      foot_height_max: msg.foot_height_max,
      zmp_offset_x: msg.zmp_offset_x,
      zmp_offset_y: msg.zmp_offset_y,
    }
  }
}


// Corresponds to op3_online_walking_module_msgs__msg__Step2D
/// 2D StepData

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Step2D {
    /// step pose as relative offset to last leg
    pub step2d: geometry_msgs::msg::Pose2D,

    /// which leg to be used (left/right/no, see below)
    pub moving_foot: u8,

}

impl Step2D {
    /// Left foot constant
    pub const LEFT_FOOT_SWING: u8 = 1;

    /// Right foot constant
    pub const RIGHT_FOOT_SWING: u8 = 2;

    /// Standing constant
    pub const STANDING: u8 = 3;

}


impl Default for Step2D {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::Step2D::default())
  }
}

impl rosidl_runtime_rs::Message for Step2D {
  type RmwMsg = super::msg::rmw::Step2D;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        step2d: geometry_msgs::msg::Pose2D::into_rmw_message(std::borrow::Cow::Owned(msg.step2d)).into_owned(),
        moving_foot: msg.moving_foot,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        step2d: geometry_msgs::msg::Pose2D::into_rmw_message(std::borrow::Cow::Borrowed(&msg.step2d)).into_owned(),
      moving_foot: msg.moving_foot,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      step2d: geometry_msgs::msg::Pose2D::from_rmw_message(msg.step2d),
      moving_foot: msg.moving_foot,
    }
  }
}


// Corresponds to op3_online_walking_module_msgs__msg__Step2DArray

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Step2DArray {

    // This member is not documented.
    #[allow(missing_docs)]
    pub step_time: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub footsteps_2d: Vec<super::msg::Step2D>,

}



impl Default for Step2DArray {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::Step2DArray::default())
  }
}

impl rosidl_runtime_rs::Message for Step2DArray {
  type RmwMsg = super::msg::rmw::Step2DArray;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        step_time: msg.step_time,
        footsteps_2d: msg.footsteps_2d
          .into_iter()
          .map(|elem| super::msg::Step2D::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      step_time: msg.step_time,
        footsteps_2d: msg.footsteps_2d
          .iter()
          .map(|elem| super::msg::Step2D::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      step_time: msg.step_time,
      footsteps_2d: msg.footsteps_2d
          .into_iter()
          .map(super::msg::Step2D::from_rmw_message)
          .collect(),
    }
  }
}


