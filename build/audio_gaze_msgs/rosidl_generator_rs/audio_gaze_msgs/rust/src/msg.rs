#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};



// Corresponds to audio_gaze_msgs__msg__AudioCue
/// A single audio perception event: is someone speaking right now, and from
/// what direction relative to the robot's forward/camera-center axis?

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct AudioCue {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::Header,

    /// Voice Activity Detection result: true if speech energy detected
    pub is_speech: bool,

    /// Direction of Arrival, degrees. 0 = straight ahead (camera center).
    /// Negative = to the robot's left, positive = to the robot's right.
    /// Range: -180 to 180 (mic arrays like ReSpeaker report 0-360;
    /// convert to this convention in the driver/adapter node).
    pub direction_deg: f32,

    /// 0.0-1.0, VAD/DOA confidence. Mock node can just send 1.0.
    pub confidence: f32,

}



impl Default for AudioCue {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::AudioCue::default())
  }
}

impl rosidl_runtime_rs::Message for AudioCue {
  type RmwMsg = super::msg::rmw::AudioCue;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        is_speech: msg.is_speech,
        direction_deg: msg.direction_deg,
        confidence: msg.confidence,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
      is_speech: msg.is_speech,
      direction_deg: msg.direction_deg,
      confidence: msg.confidence,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      is_speech: msg.is_speech,
      direction_deg: msg.direction_deg,
      confidence: msg.confidence,
    }
  }
}


