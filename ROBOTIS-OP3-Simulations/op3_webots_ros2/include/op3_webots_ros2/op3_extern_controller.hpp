#ifndef OP3_WEBOTS_ROS2_OP3_EXTERN_CONTROLLER_HPP
#define OP3_WEBOTS_ROS2_OP3_EXTERN_CONTROLLER_HPP

#include <rclcpp/rclcpp.hpp>

#include <thread>
#include <vector>
#include <string>

#include <std_msgs/msg/float64.hpp>
#include <std_msgs/msg/string.hpp>
#include <sensor_msgs/msg/joint_state.hpp>
#include <sensor_msgs/msg/imu.hpp>
#include <sensor_msgs/msg/camera_info.hpp>
#include <sensor_msgs/msg/image.hpp>
#include <geometry_msgs/msg/vector3.hpp>
#include <webots/Supervisor.hpp>

#include <yaml-cpp/yaml.h>

#define N_MOTORS (20)

namespace robotis_op
{

class OP3ExternController : public webots::Supervisor, public rclcpp::Node
{
public:
  OP3ExternController();
  virtual ~OP3ExternController();

  void run();

  void initialize(std::string gain_file_path);

  void process();

  void stepWebots();

  void setDesiredJointAngles();
  void getPresentJointAngles();
  void getPresentJointTorques();

  void getCameraInfo();
  
  void getCurrentRobotCOM();
  void getIMUOutput();

  void publishPresentJointStates();
  void publishIMUOutput();
  void publishCOMData();
  void publishCameraData();
  void publishWorldGroundTruth();

  void posCommandCallback(const std_msgs::msg::Float64::SharedPtr msg, const int &joint_idx);
  
  void queueThread();
  
  bool parsePIDGainYAML(std::string gain_file_path);


  int time_step_ms_;
  double time_step_sec_;

  double current_time_sec_; // control time

  // for motor angle
  double desired_joint_angle_rad_[N_MOTORS];
  double current_joint_angle_rad_[N_MOTORS];
  double current_joint_torque_Nm_[N_MOTORS];
  sensor_msgs::msg::JointState joint_state_msg_;

  // center of mass
  double current_com_m_[3];
  double previous_com_m_[3];
  double current_com_vel_mps_[3];
  geometry_msgs::msg::Vector3 com_m_;
  
  // imu
  double torso_xyz_m_[3];
  sensor_msgs::msg::Imu imu_data_;

  // camera
  sensor_msgs::msg::CameraInfo camera_info_msg_;
  sensor_msgs::msg::Image image_data_;

  // publishers
  rclcpp::Publisher<geometry_msgs::msg::Vector3>::SharedPtr com_data_publisher_;
  rclcpp::Publisher<sensor_msgs::msg::JointState>::SharedPtr present_joint_state_publisher_;
  rclcpp::Publisher<sensor_msgs::msg::Imu>::SharedPtr imu_data_publisher_;
  rclcpp::Publisher<sensor_msgs::msg::CameraInfo>::SharedPtr camera_info_publisher_;
  rclcpp::Publisher<sensor_msgs::msg::Image>::SharedPtr camera_image_publisher_;
  rclcpp::Publisher<std_msgs::msg::String>::SharedPtr world_ground_truth_publisher_;

  // World ground-truth (for gaze_dashboard's top-down map): populated in
  // initialize() via Supervisor::getFromDef() against DEF-labeled Pedestrian
  // nodes in the .wbt world. A null entry (DEF not found) is skipped, not
  // fatal -- lets the world file add/remove pedestrians without a rebuild.
  std::vector<webots::Node*> pedestrian_nodes_;
  std::vector<std::string> pedestrian_names_;

  // devices
  webots::Camera* camera_;
  webots::LED* head_led_;
  webots::LED* body_led_;
  webots::Speaker* speaker_;
  webots::Keyboard* key_board_; 
  
  webots::Gyro *gyro_;
  webots::Accelerometer *acc_;
  webots::InertialUnit *iu_;

  webots::Motor* motors_[N_MOTORS];
  webots::PositionSensor* encoders_[N_MOTORS];
  
  webots::Node *torso_node_;
  webots::Node *rf_node_, *lf_node_;

  bool desired_joint_angle_rcv_flag_[20];
  
  // thread
  std::thread queue_thread_;
};

}

#endif