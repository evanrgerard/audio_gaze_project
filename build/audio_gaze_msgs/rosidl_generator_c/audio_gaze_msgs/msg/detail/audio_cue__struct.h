// generated from rosidl_generator_c/resource/idl__struct.h.em
// with input from audio_gaze_msgs:msg/AudioCue.idl
// generated code does not contain a copyright notice

#ifndef AUDIO_GAZE_MSGS__MSG__DETAIL__AUDIO_CUE__STRUCT_H_
#define AUDIO_GAZE_MSGS__MSG__DETAIL__AUDIO_CUE__STRUCT_H_

#ifdef __cplusplus
extern "C"
{
#endif

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>


// Constants defined in the message

// Include directives for member types
// Member 'header'
#include "std_msgs/msg/detail/header__struct.h"

/// Struct defined in msg/AudioCue in the package audio_gaze_msgs.
/**
  * A single audio perception event: is someone speaking right now, and from
  * what direction relative to the robot's forward/camera-center axis?
 */
typedef struct audio_gaze_msgs__msg__AudioCue
{
  std_msgs__msg__Header header;
  /// Voice Activity Detection result: true if speech energy detected
  bool is_speech;
  /// Direction of Arrival, degrees. 0 = straight ahead (camera center).
  /// Negative = to the robot's left, positive = to the robot's right.
  /// Range: -180 to 180 (mic arrays like ReSpeaker report 0-360;
  /// convert to this convention in the driver/adapter node).
  float direction_deg;
  /// 0.0-1.0, VAD/DOA confidence. Mock node can just send 1.0.
  float confidence;
} audio_gaze_msgs__msg__AudioCue;

// Struct for a sequence of audio_gaze_msgs__msg__AudioCue.
typedef struct audio_gaze_msgs__msg__AudioCue__Sequence
{
  audio_gaze_msgs__msg__AudioCue * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} audio_gaze_msgs__msg__AudioCue__Sequence;

#ifdef __cplusplus
}
#endif

#endif  // AUDIO_GAZE_MSGS__MSG__DETAIL__AUDIO_CUE__STRUCT_H_
