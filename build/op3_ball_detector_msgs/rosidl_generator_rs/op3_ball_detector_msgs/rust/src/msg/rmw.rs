#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};


#[link(name = "op3_ball_detector_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__op3_ball_detector_msgs__msg__BallDetectorParams() -> *const std::ffi::c_void;
}

#[link(name = "op3_ball_detector_msgs__rosidl_generator_c")]
extern "C" {
    fn op3_ball_detector_msgs__msg__BallDetectorParams__init(msg: *mut BallDetectorParams) -> bool;
    fn op3_ball_detector_msgs__msg__BallDetectorParams__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<BallDetectorParams>, size: usize) -> bool;
    fn op3_ball_detector_msgs__msg__BallDetectorParams__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<BallDetectorParams>);
    fn op3_ball_detector_msgs__msg__BallDetectorParams__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<BallDetectorParams>, out_seq: *mut rosidl_runtime_rs::Sequence<BallDetectorParams>) -> bool;
}

// Corresponds to op3_ball_detector_msgs__msg__BallDetectorParams
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// This represents the parameters of ball_detector

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BallDetectorParams {
    /// only odd number, 1 - 11
    pub gaussian_blur_size: u32,

    /// 1 - 5
    pub gaussian_blur_sigma: f32,

    /// 50 - 200
    pub canny_edge_th: f32,

    /// 1 - 8
    pub hough_accum_resolution: f32,

    /// 10 - 200
    pub hough_accum_th: f32,

    /// 10 - 200
    pub min_circle_dist: f32,

    /// 10 - 200
    pub min_radius: u32,

    /// 100 - 600
    pub max_radius: u32,

    /// 0 - 359
    pub filter_h_min: u32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub filter_h_max: u32,

    /// 0 - 255
    pub filter_s_min: u32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub filter_s_max: u32,

    /// 0 - 255
    pub filter_v_min: u32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub filter_v_max: u32,

    /// 1 - 9
    pub ellipse_size: u32,

}



impl Default for BallDetectorParams {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !op3_ball_detector_msgs__msg__BallDetectorParams__init(&mut msg as *mut _) {
        panic!("Call to op3_ball_detector_msgs__msg__BallDetectorParams__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for BallDetectorParams {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { op3_ball_detector_msgs__msg__BallDetectorParams__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { op3_ball_detector_msgs__msg__BallDetectorParams__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { op3_ball_detector_msgs__msg__BallDetectorParams__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for BallDetectorParams {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for BallDetectorParams where Self: Sized {
  const TYPE_NAME: &'static str = "op3_ball_detector_msgs/msg/BallDetectorParams";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__op3_ball_detector_msgs__msg__BallDetectorParams() }
  }
}


#[link(name = "op3_ball_detector_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__op3_ball_detector_msgs__msg__CircleSetStamped() -> *const std::ffi::c_void;
}

#[link(name = "op3_ball_detector_msgs__rosidl_generator_c")]
extern "C" {
    fn op3_ball_detector_msgs__msg__CircleSetStamped__init(msg: *mut CircleSetStamped) -> bool;
    fn op3_ball_detector_msgs__msg__CircleSetStamped__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<CircleSetStamped>, size: usize) -> bool;
    fn op3_ball_detector_msgs__msg__CircleSetStamped__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<CircleSetStamped>);
    fn op3_ball_detector_msgs__msg__CircleSetStamped__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<CircleSetStamped>, out_seq: *mut rosidl_runtime_rs::Sequence<CircleSetStamped>) -> bool;
}

// Corresponds to op3_ball_detector_msgs__msg__CircleSetStamped
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// This represents the set of detected circles

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CircleSetStamped {
    /// timestamp and frame id of the image frame
    pub header: std_msgs::msg::rmw::Header,

    /// set of detected circles:
    ///       (circles[i].x, circles[i].y) is the center point in image coordinates
    ///       circles[i].z is the circle radius
    pub circles: rosidl_runtime_rs::Sequence<geometry_msgs::msg::rmw::Point>,

}



impl Default for CircleSetStamped {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !op3_ball_detector_msgs__msg__CircleSetStamped__init(&mut msg as *mut _) {
        panic!("Call to op3_ball_detector_msgs__msg__CircleSetStamped__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for CircleSetStamped {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { op3_ball_detector_msgs__msg__CircleSetStamped__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { op3_ball_detector_msgs__msg__CircleSetStamped__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { op3_ball_detector_msgs__msg__CircleSetStamped__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for CircleSetStamped {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for CircleSetStamped where Self: Sized {
  const TYPE_NAME: &'static str = "op3_ball_detector_msgs/msg/CircleSetStamped";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__op3_ball_detector_msgs__msg__CircleSetStamped() }
  }
}


