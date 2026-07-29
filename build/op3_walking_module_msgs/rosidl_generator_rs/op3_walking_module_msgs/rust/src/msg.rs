#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};



// Corresponds to op3_walking_module_msgs__msg__WalkingParam
/// walking init pose #######

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct WalkingParam {

    // This member is not documented.
    #[allow(missing_docs)]
    pub init_x_offset: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub init_y_offset: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub init_z_offset: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub init_roll_offset: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub init_pitch_offset: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub init_yaw_offset: f32,

    /// time parameter #####
    pub period_time: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub dsp_ratio: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub step_fb_ratio: f32,

    /// walking parameter ########
    pub x_move_amplitude: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub y_move_amplitude: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub z_move_amplitude: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub angle_move_amplitude: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub move_aim_on: bool,

    /// balance parameter ##########
    pub balance_enable: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub balance_hip_roll_gain: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub balance_knee_gain: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub balance_ankle_roll_gain: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub balance_ankle_pitch_gain: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub y_swap_amplitude: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub z_swap_amplitude: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub arm_swing_gain: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub pelvis_offset: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub hip_pitch_offset: f32,

    /// gain parameter ##########
    pub p_gain: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub i_gain: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub d_gain: i32,

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
        init_x_offset: msg.init_x_offset,
        init_y_offset: msg.init_y_offset,
        init_z_offset: msg.init_z_offset,
        init_roll_offset: msg.init_roll_offset,
        init_pitch_offset: msg.init_pitch_offset,
        init_yaw_offset: msg.init_yaw_offset,
        period_time: msg.period_time,
        dsp_ratio: msg.dsp_ratio,
        step_fb_ratio: msg.step_fb_ratio,
        x_move_amplitude: msg.x_move_amplitude,
        y_move_amplitude: msg.y_move_amplitude,
        z_move_amplitude: msg.z_move_amplitude,
        angle_move_amplitude: msg.angle_move_amplitude,
        move_aim_on: msg.move_aim_on,
        balance_enable: msg.balance_enable,
        balance_hip_roll_gain: msg.balance_hip_roll_gain,
        balance_knee_gain: msg.balance_knee_gain,
        balance_ankle_roll_gain: msg.balance_ankle_roll_gain,
        balance_ankle_pitch_gain: msg.balance_ankle_pitch_gain,
        y_swap_amplitude: msg.y_swap_amplitude,
        z_swap_amplitude: msg.z_swap_amplitude,
        arm_swing_gain: msg.arm_swing_gain,
        pelvis_offset: msg.pelvis_offset,
        hip_pitch_offset: msg.hip_pitch_offset,
        p_gain: msg.p_gain,
        i_gain: msg.i_gain,
        d_gain: msg.d_gain,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      init_x_offset: msg.init_x_offset,
      init_y_offset: msg.init_y_offset,
      init_z_offset: msg.init_z_offset,
      init_roll_offset: msg.init_roll_offset,
      init_pitch_offset: msg.init_pitch_offset,
      init_yaw_offset: msg.init_yaw_offset,
      period_time: msg.period_time,
      dsp_ratio: msg.dsp_ratio,
      step_fb_ratio: msg.step_fb_ratio,
      x_move_amplitude: msg.x_move_amplitude,
      y_move_amplitude: msg.y_move_amplitude,
      z_move_amplitude: msg.z_move_amplitude,
      angle_move_amplitude: msg.angle_move_amplitude,
      move_aim_on: msg.move_aim_on,
      balance_enable: msg.balance_enable,
      balance_hip_roll_gain: msg.balance_hip_roll_gain,
      balance_knee_gain: msg.balance_knee_gain,
      balance_ankle_roll_gain: msg.balance_ankle_roll_gain,
      balance_ankle_pitch_gain: msg.balance_ankle_pitch_gain,
      y_swap_amplitude: msg.y_swap_amplitude,
      z_swap_amplitude: msg.z_swap_amplitude,
      arm_swing_gain: msg.arm_swing_gain,
      pelvis_offset: msg.pelvis_offset,
      hip_pitch_offset: msg.hip_pitch_offset,
      p_gain: msg.p_gain,
      i_gain: msg.i_gain,
      d_gain: msg.d_gain,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      init_x_offset: msg.init_x_offset,
      init_y_offset: msg.init_y_offset,
      init_z_offset: msg.init_z_offset,
      init_roll_offset: msg.init_roll_offset,
      init_pitch_offset: msg.init_pitch_offset,
      init_yaw_offset: msg.init_yaw_offset,
      period_time: msg.period_time,
      dsp_ratio: msg.dsp_ratio,
      step_fb_ratio: msg.step_fb_ratio,
      x_move_amplitude: msg.x_move_amplitude,
      y_move_amplitude: msg.y_move_amplitude,
      z_move_amplitude: msg.z_move_amplitude,
      angle_move_amplitude: msg.angle_move_amplitude,
      move_aim_on: msg.move_aim_on,
      balance_enable: msg.balance_enable,
      balance_hip_roll_gain: msg.balance_hip_roll_gain,
      balance_knee_gain: msg.balance_knee_gain,
      balance_ankle_roll_gain: msg.balance_ankle_roll_gain,
      balance_ankle_pitch_gain: msg.balance_ankle_pitch_gain,
      y_swap_amplitude: msg.y_swap_amplitude,
      z_swap_amplitude: msg.z_swap_amplitude,
      arm_swing_gain: msg.arm_swing_gain,
      pelvis_offset: msg.pelvis_offset,
      hip_pitch_offset: msg.hip_pitch_offset,
      p_gain: msg.p_gain,
      i_gain: msg.i_gain,
      d_gain: msg.d_gain,
    }
  }
}


