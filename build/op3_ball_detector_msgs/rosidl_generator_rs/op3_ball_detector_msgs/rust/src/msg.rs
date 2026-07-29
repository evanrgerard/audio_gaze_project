#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};



// Corresponds to op3_ball_detector_msgs__msg__BallDetectorParams
/// This represents the parameters of ball_detector

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::BallDetectorParams::default())
  }
}

impl rosidl_runtime_rs::Message for BallDetectorParams {
  type RmwMsg = super::msg::rmw::BallDetectorParams;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        gaussian_blur_size: msg.gaussian_blur_size,
        gaussian_blur_sigma: msg.gaussian_blur_sigma,
        canny_edge_th: msg.canny_edge_th,
        hough_accum_resolution: msg.hough_accum_resolution,
        hough_accum_th: msg.hough_accum_th,
        min_circle_dist: msg.min_circle_dist,
        min_radius: msg.min_radius,
        max_radius: msg.max_radius,
        filter_h_min: msg.filter_h_min,
        filter_h_max: msg.filter_h_max,
        filter_s_min: msg.filter_s_min,
        filter_s_max: msg.filter_s_max,
        filter_v_min: msg.filter_v_min,
        filter_v_max: msg.filter_v_max,
        ellipse_size: msg.ellipse_size,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      gaussian_blur_size: msg.gaussian_blur_size,
      gaussian_blur_sigma: msg.gaussian_blur_sigma,
      canny_edge_th: msg.canny_edge_th,
      hough_accum_resolution: msg.hough_accum_resolution,
      hough_accum_th: msg.hough_accum_th,
      min_circle_dist: msg.min_circle_dist,
      min_radius: msg.min_radius,
      max_radius: msg.max_radius,
      filter_h_min: msg.filter_h_min,
      filter_h_max: msg.filter_h_max,
      filter_s_min: msg.filter_s_min,
      filter_s_max: msg.filter_s_max,
      filter_v_min: msg.filter_v_min,
      filter_v_max: msg.filter_v_max,
      ellipse_size: msg.ellipse_size,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      gaussian_blur_size: msg.gaussian_blur_size,
      gaussian_blur_sigma: msg.gaussian_blur_sigma,
      canny_edge_th: msg.canny_edge_th,
      hough_accum_resolution: msg.hough_accum_resolution,
      hough_accum_th: msg.hough_accum_th,
      min_circle_dist: msg.min_circle_dist,
      min_radius: msg.min_radius,
      max_radius: msg.max_radius,
      filter_h_min: msg.filter_h_min,
      filter_h_max: msg.filter_h_max,
      filter_s_min: msg.filter_s_min,
      filter_s_max: msg.filter_s_max,
      filter_v_min: msg.filter_v_min,
      filter_v_max: msg.filter_v_max,
      ellipse_size: msg.ellipse_size,
    }
  }
}


// Corresponds to op3_ball_detector_msgs__msg__CircleSetStamped
/// This represents the set of detected circles

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CircleSetStamped {
    /// timestamp and frame id of the image frame
    pub header: std_msgs::msg::Header,

    /// set of detected circles:
    ///       (circles[i].x, circles[i].y) is the center point in image coordinates
    ///       circles[i].z is the circle radius
    pub circles: Vec<geometry_msgs::msg::Point>,

}



impl Default for CircleSetStamped {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::CircleSetStamped::default())
  }
}

impl rosidl_runtime_rs::Message for CircleSetStamped {
  type RmwMsg = super::msg::rmw::CircleSetStamped;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        circles: msg.circles
          .into_iter()
          .map(|elem| geometry_msgs::msg::Point::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
        circles: msg.circles
          .iter()
          .map(|elem| geometry_msgs::msg::Point::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      circles: msg.circles
          .into_iter()
          .map(geometry_msgs::msg::Point::from_rmw_message)
          .collect(),
    }
  }
}


