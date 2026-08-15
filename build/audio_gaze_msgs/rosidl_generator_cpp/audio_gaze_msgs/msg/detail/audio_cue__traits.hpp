// generated from rosidl_generator_cpp/resource/idl__traits.hpp.em
// with input from audio_gaze_msgs:msg/AudioCue.idl
// generated code does not contain a copyright notice

#ifndef AUDIO_GAZE_MSGS__MSG__DETAIL__AUDIO_CUE__TRAITS_HPP_
#define AUDIO_GAZE_MSGS__MSG__DETAIL__AUDIO_CUE__TRAITS_HPP_

#include <stdint.h>

#include <sstream>
#include <string>
#include <type_traits>

#include "audio_gaze_msgs/msg/detail/audio_cue__struct.hpp"
#include "rosidl_runtime_cpp/traits.hpp"

// Include directives for member types
// Member 'header'
#include "std_msgs/msg/detail/header__traits.hpp"

namespace audio_gaze_msgs
{

namespace msg
{

inline void to_flow_style_yaml(
  const AudioCue & msg,
  std::ostream & out)
{
  out << "{";
  // member: header
  {
    out << "header: ";
    to_flow_style_yaml(msg.header, out);
    out << ", ";
  }

  // member: is_speech
  {
    out << "is_speech: ";
    rosidl_generator_traits::value_to_yaml(msg.is_speech, out);
    out << ", ";
  }

  // member: direction_deg
  {
    out << "direction_deg: ";
    rosidl_generator_traits::value_to_yaml(msg.direction_deg, out);
    out << ", ";
  }

  // member: confidence
  {
    out << "confidence: ";
    rosidl_generator_traits::value_to_yaml(msg.confidence, out);
  }
  out << "}";
}  // NOLINT(readability/fn_size)

inline void to_block_style_yaml(
  const AudioCue & msg,
  std::ostream & out, size_t indentation = 0)
{
  // member: header
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "header:\n";
    to_block_style_yaml(msg.header, out, indentation + 2);
  }

  // member: is_speech
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "is_speech: ";
    rosidl_generator_traits::value_to_yaml(msg.is_speech, out);
    out << "\n";
  }

  // member: direction_deg
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "direction_deg: ";
    rosidl_generator_traits::value_to_yaml(msg.direction_deg, out);
    out << "\n";
  }

  // member: confidence
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "confidence: ";
    rosidl_generator_traits::value_to_yaml(msg.confidence, out);
    out << "\n";
  }
}  // NOLINT(readability/fn_size)

inline std::string to_yaml(const AudioCue & msg, bool use_flow_style = false)
{
  std::ostringstream out;
  if (use_flow_style) {
    to_flow_style_yaml(msg, out);
  } else {
    to_block_style_yaml(msg, out);
  }
  return out.str();
}

}  // namespace msg

}  // namespace audio_gaze_msgs

namespace rosidl_generator_traits
{

[[deprecated("use audio_gaze_msgs::msg::to_block_style_yaml() instead")]]
inline void to_yaml(
  const audio_gaze_msgs::msg::AudioCue & msg,
  std::ostream & out, size_t indentation = 0)
{
  audio_gaze_msgs::msg::to_block_style_yaml(msg, out, indentation);
}

[[deprecated("use audio_gaze_msgs::msg::to_yaml() instead")]]
inline std::string to_yaml(const audio_gaze_msgs::msg::AudioCue & msg)
{
  return audio_gaze_msgs::msg::to_yaml(msg);
}

template<>
inline const char * data_type<audio_gaze_msgs::msg::AudioCue>()
{
  return "audio_gaze_msgs::msg::AudioCue";
}

template<>
inline const char * name<audio_gaze_msgs::msg::AudioCue>()
{
  return "audio_gaze_msgs/msg/AudioCue";
}

template<>
struct has_fixed_size<audio_gaze_msgs::msg::AudioCue>
  : std::integral_constant<bool, has_fixed_size<std_msgs::msg::Header>::value> {};

template<>
struct has_bounded_size<audio_gaze_msgs::msg::AudioCue>
  : std::integral_constant<bool, has_bounded_size<std_msgs::msg::Header>::value> {};

template<>
struct is_message<audio_gaze_msgs::msg::AudioCue>
  : std::true_type {};

}  // namespace rosidl_generator_traits

#endif  // AUDIO_GAZE_MSGS__MSG__DETAIL__AUDIO_CUE__TRAITS_HPP_
