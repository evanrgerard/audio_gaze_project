// generated from rosidl_generator_cpp/resource/idl__builder.hpp.em
// with input from audio_gaze_msgs:msg/AudioCue.idl
// generated code does not contain a copyright notice

#ifndef AUDIO_GAZE_MSGS__MSG__DETAIL__AUDIO_CUE__BUILDER_HPP_
#define AUDIO_GAZE_MSGS__MSG__DETAIL__AUDIO_CUE__BUILDER_HPP_

#include <algorithm>
#include <utility>

#include "audio_gaze_msgs/msg/detail/audio_cue__struct.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


namespace audio_gaze_msgs
{

namespace msg
{

namespace builder
{

class Init_AudioCue_confidence
{
public:
  explicit Init_AudioCue_confidence(::audio_gaze_msgs::msg::AudioCue & msg)
  : msg_(msg)
  {}
  ::audio_gaze_msgs::msg::AudioCue confidence(::audio_gaze_msgs::msg::AudioCue::_confidence_type arg)
  {
    msg_.confidence = std::move(arg);
    return std::move(msg_);
  }

private:
  ::audio_gaze_msgs::msg::AudioCue msg_;
};

class Init_AudioCue_direction_deg
{
public:
  explicit Init_AudioCue_direction_deg(::audio_gaze_msgs::msg::AudioCue & msg)
  : msg_(msg)
  {}
  Init_AudioCue_confidence direction_deg(::audio_gaze_msgs::msg::AudioCue::_direction_deg_type arg)
  {
    msg_.direction_deg = std::move(arg);
    return Init_AudioCue_confidence(msg_);
  }

private:
  ::audio_gaze_msgs::msg::AudioCue msg_;
};

class Init_AudioCue_is_speech
{
public:
  explicit Init_AudioCue_is_speech(::audio_gaze_msgs::msg::AudioCue & msg)
  : msg_(msg)
  {}
  Init_AudioCue_direction_deg is_speech(::audio_gaze_msgs::msg::AudioCue::_is_speech_type arg)
  {
    msg_.is_speech = std::move(arg);
    return Init_AudioCue_direction_deg(msg_);
  }

private:
  ::audio_gaze_msgs::msg::AudioCue msg_;
};

class Init_AudioCue_header
{
public:
  Init_AudioCue_header()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_AudioCue_is_speech header(::audio_gaze_msgs::msg::AudioCue::_header_type arg)
  {
    msg_.header = std::move(arg);
    return Init_AudioCue_is_speech(msg_);
  }

private:
  ::audio_gaze_msgs::msg::AudioCue msg_;
};

}  // namespace builder

}  // namespace msg

template<typename MessageType>
auto build();

template<>
inline
auto build<::audio_gaze_msgs::msg::AudioCue>()
{
  return audio_gaze_msgs::msg::builder::Init_AudioCue_header();
}

}  // namespace audio_gaze_msgs

#endif  // AUDIO_GAZE_MSGS__MSG__DETAIL__AUDIO_CUE__BUILDER_HPP_
