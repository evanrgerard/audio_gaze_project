// generated from rosidl_generator_cpp/resource/idl__struct.hpp.em
// with input from audio_gaze_msgs:msg/AudioCue.idl
// generated code does not contain a copyright notice

#ifndef AUDIO_GAZE_MSGS__MSG__DETAIL__AUDIO_CUE__STRUCT_HPP_
#define AUDIO_GAZE_MSGS__MSG__DETAIL__AUDIO_CUE__STRUCT_HPP_

#include <algorithm>
#include <array>
#include <cstdint>
#include <memory>
#include <string>
#include <vector>

#include "rosidl_runtime_cpp/bounded_vector.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


// Include directives for member types
// Member 'header'
#include "std_msgs/msg/detail/header__struct.hpp"

#ifndef _WIN32
# define DEPRECATED__audio_gaze_msgs__msg__AudioCue __attribute__((deprecated))
#else
# define DEPRECATED__audio_gaze_msgs__msg__AudioCue __declspec(deprecated)
#endif

namespace audio_gaze_msgs
{

namespace msg
{

// message struct
template<class ContainerAllocator>
struct AudioCue_
{
  using Type = AudioCue_<ContainerAllocator>;

  explicit AudioCue_(rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  : header(_init)
  {
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->is_speech = false;
      this->direction_deg = 0.0f;
      this->confidence = 0.0f;
    }
  }

  explicit AudioCue_(const ContainerAllocator & _alloc, rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  : header(_alloc, _init)
  {
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->is_speech = false;
      this->direction_deg = 0.0f;
      this->confidence = 0.0f;
    }
  }

  // field types and members
  using _header_type =
    std_msgs::msg::Header_<ContainerAllocator>;
  _header_type header;
  using _is_speech_type =
    bool;
  _is_speech_type is_speech;
  using _direction_deg_type =
    float;
  _direction_deg_type direction_deg;
  using _confidence_type =
    float;
  _confidence_type confidence;

  // setters for named parameter idiom
  Type & set__header(
    const std_msgs::msg::Header_<ContainerAllocator> & _arg)
  {
    this->header = _arg;
    return *this;
  }
  Type & set__is_speech(
    const bool & _arg)
  {
    this->is_speech = _arg;
    return *this;
  }
  Type & set__direction_deg(
    const float & _arg)
  {
    this->direction_deg = _arg;
    return *this;
  }
  Type & set__confidence(
    const float & _arg)
  {
    this->confidence = _arg;
    return *this;
  }

  // constant declarations

  // pointer types
  using RawPtr =
    audio_gaze_msgs::msg::AudioCue_<ContainerAllocator> *;
  using ConstRawPtr =
    const audio_gaze_msgs::msg::AudioCue_<ContainerAllocator> *;
  using SharedPtr =
    std::shared_ptr<audio_gaze_msgs::msg::AudioCue_<ContainerAllocator>>;
  using ConstSharedPtr =
    std::shared_ptr<audio_gaze_msgs::msg::AudioCue_<ContainerAllocator> const>;

  template<typename Deleter = std::default_delete<
      audio_gaze_msgs::msg::AudioCue_<ContainerAllocator>>>
  using UniquePtrWithDeleter =
    std::unique_ptr<audio_gaze_msgs::msg::AudioCue_<ContainerAllocator>, Deleter>;

  using UniquePtr = UniquePtrWithDeleter<>;

  template<typename Deleter = std::default_delete<
      audio_gaze_msgs::msg::AudioCue_<ContainerAllocator>>>
  using ConstUniquePtrWithDeleter =
    std::unique_ptr<audio_gaze_msgs::msg::AudioCue_<ContainerAllocator> const, Deleter>;
  using ConstUniquePtr = ConstUniquePtrWithDeleter<>;

  using WeakPtr =
    std::weak_ptr<audio_gaze_msgs::msg::AudioCue_<ContainerAllocator>>;
  using ConstWeakPtr =
    std::weak_ptr<audio_gaze_msgs::msg::AudioCue_<ContainerAllocator> const>;

  // pointer types similar to ROS 1, use SharedPtr / ConstSharedPtr instead
  // NOTE: Can't use 'using' here because GNU C++ can't parse attributes properly
  typedef DEPRECATED__audio_gaze_msgs__msg__AudioCue
    std::shared_ptr<audio_gaze_msgs::msg::AudioCue_<ContainerAllocator>>
    Ptr;
  typedef DEPRECATED__audio_gaze_msgs__msg__AudioCue
    std::shared_ptr<audio_gaze_msgs::msg::AudioCue_<ContainerAllocator> const>
    ConstPtr;

  // comparison operators
  bool operator==(const AudioCue_ & other) const
  {
    if (this->header != other.header) {
      return false;
    }
    if (this->is_speech != other.is_speech) {
      return false;
    }
    if (this->direction_deg != other.direction_deg) {
      return false;
    }
    if (this->confidence != other.confidence) {
      return false;
    }
    return true;
  }
  bool operator!=(const AudioCue_ & other) const
  {
    return !this->operator==(other);
  }
};  // struct AudioCue_

// alias to use template instance with default allocator
using AudioCue =
  audio_gaze_msgs::msg::AudioCue_<std::allocator<void>>;

// constant definitions

}  // namespace msg

}  // namespace audio_gaze_msgs

#endif  // AUDIO_GAZE_MSGS__MSG__DETAIL__AUDIO_CUE__STRUCT_HPP_
