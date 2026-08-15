#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};


#[link(name = "audio_gaze_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__audio_gaze_msgs__msg__AudioCue() -> *const std::ffi::c_void;
}

#[link(name = "audio_gaze_msgs__rosidl_generator_c")]
extern "C" {
    fn audio_gaze_msgs__msg__AudioCue__init(msg: *mut AudioCue) -> bool;
    fn audio_gaze_msgs__msg__AudioCue__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<AudioCue>, size: usize) -> bool;
    fn audio_gaze_msgs__msg__AudioCue__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<AudioCue>);
    fn audio_gaze_msgs__msg__AudioCue__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<AudioCue>, out_seq: *mut rosidl_runtime_rs::Sequence<AudioCue>) -> bool;
}

// Corresponds to audio_gaze_msgs__msg__AudioCue
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// A single audio perception event: is someone speaking right now, and from
/// what direction relative to the robot's forward/camera-center axis?

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct AudioCue {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::rmw::Header,

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
    unsafe {
      let mut msg = std::mem::zeroed();
      if !audio_gaze_msgs__msg__AudioCue__init(&mut msg as *mut _) {
        panic!("Call to audio_gaze_msgs__msg__AudioCue__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for AudioCue {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { audio_gaze_msgs__msg__AudioCue__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { audio_gaze_msgs__msg__AudioCue__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { audio_gaze_msgs__msg__AudioCue__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for AudioCue {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for AudioCue where Self: Sized {
  const TYPE_NAME: &'static str = "audio_gaze_msgs/msg/AudioCue";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__audio_gaze_msgs__msg__AudioCue() }
  }
}


