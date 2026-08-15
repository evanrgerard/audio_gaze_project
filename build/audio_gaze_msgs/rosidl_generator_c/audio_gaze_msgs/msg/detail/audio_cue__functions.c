// generated from rosidl_generator_c/resource/idl__functions.c.em
// with input from audio_gaze_msgs:msg/AudioCue.idl
// generated code does not contain a copyright notice
#include "audio_gaze_msgs/msg/detail/audio_cue__functions.h"

#include <assert.h>
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>

#include "rcutils/allocator.h"


// Include directives for member types
// Member `header`
#include "std_msgs/msg/detail/header__functions.h"

bool
audio_gaze_msgs__msg__AudioCue__init(audio_gaze_msgs__msg__AudioCue * msg)
{
  if (!msg) {
    return false;
  }
  // header
  if (!std_msgs__msg__Header__init(&msg->header)) {
    audio_gaze_msgs__msg__AudioCue__fini(msg);
    return false;
  }
  // is_speech
  // direction_deg
  // confidence
  return true;
}

void
audio_gaze_msgs__msg__AudioCue__fini(audio_gaze_msgs__msg__AudioCue * msg)
{
  if (!msg) {
    return;
  }
  // header
  std_msgs__msg__Header__fini(&msg->header);
  // is_speech
  // direction_deg
  // confidence
}

bool
audio_gaze_msgs__msg__AudioCue__are_equal(const audio_gaze_msgs__msg__AudioCue * lhs, const audio_gaze_msgs__msg__AudioCue * rhs)
{
  if (!lhs || !rhs) {
    return false;
  }
  // header
  if (!std_msgs__msg__Header__are_equal(
      &(lhs->header), &(rhs->header)))
  {
    return false;
  }
  // is_speech
  if (lhs->is_speech != rhs->is_speech) {
    return false;
  }
  // direction_deg
  if (lhs->direction_deg != rhs->direction_deg) {
    return false;
  }
  // confidence
  if (lhs->confidence != rhs->confidence) {
    return false;
  }
  return true;
}

bool
audio_gaze_msgs__msg__AudioCue__copy(
  const audio_gaze_msgs__msg__AudioCue * input,
  audio_gaze_msgs__msg__AudioCue * output)
{
  if (!input || !output) {
    return false;
  }
  // header
  if (!std_msgs__msg__Header__copy(
      &(input->header), &(output->header)))
  {
    return false;
  }
  // is_speech
  output->is_speech = input->is_speech;
  // direction_deg
  output->direction_deg = input->direction_deg;
  // confidence
  output->confidence = input->confidence;
  return true;
}

audio_gaze_msgs__msg__AudioCue *
audio_gaze_msgs__msg__AudioCue__create()
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  audio_gaze_msgs__msg__AudioCue * msg = (audio_gaze_msgs__msg__AudioCue *)allocator.allocate(sizeof(audio_gaze_msgs__msg__AudioCue), allocator.state);
  if (!msg) {
    return NULL;
  }
  memset(msg, 0, sizeof(audio_gaze_msgs__msg__AudioCue));
  bool success = audio_gaze_msgs__msg__AudioCue__init(msg);
  if (!success) {
    allocator.deallocate(msg, allocator.state);
    return NULL;
  }
  return msg;
}

void
audio_gaze_msgs__msg__AudioCue__destroy(audio_gaze_msgs__msg__AudioCue * msg)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  if (msg) {
    audio_gaze_msgs__msg__AudioCue__fini(msg);
  }
  allocator.deallocate(msg, allocator.state);
}


bool
audio_gaze_msgs__msg__AudioCue__Sequence__init(audio_gaze_msgs__msg__AudioCue__Sequence * array, size_t size)
{
  if (!array) {
    return false;
  }
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  audio_gaze_msgs__msg__AudioCue * data = NULL;

  if (size) {
    data = (audio_gaze_msgs__msg__AudioCue *)allocator.zero_allocate(size, sizeof(audio_gaze_msgs__msg__AudioCue), allocator.state);
    if (!data) {
      return false;
    }
    // initialize all array elements
    size_t i;
    for (i = 0; i < size; ++i) {
      bool success = audio_gaze_msgs__msg__AudioCue__init(&data[i]);
      if (!success) {
        break;
      }
    }
    if (i < size) {
      // if initialization failed finalize the already initialized array elements
      for (; i > 0; --i) {
        audio_gaze_msgs__msg__AudioCue__fini(&data[i - 1]);
      }
      allocator.deallocate(data, allocator.state);
      return false;
    }
  }
  array->data = data;
  array->size = size;
  array->capacity = size;
  return true;
}

void
audio_gaze_msgs__msg__AudioCue__Sequence__fini(audio_gaze_msgs__msg__AudioCue__Sequence * array)
{
  if (!array) {
    return;
  }
  rcutils_allocator_t allocator = rcutils_get_default_allocator();

  if (array->data) {
    // ensure that data and capacity values are consistent
    assert(array->capacity > 0);
    // finalize all array elements
    for (size_t i = 0; i < array->capacity; ++i) {
      audio_gaze_msgs__msg__AudioCue__fini(&array->data[i]);
    }
    allocator.deallocate(array->data, allocator.state);
    array->data = NULL;
    array->size = 0;
    array->capacity = 0;
  } else {
    // ensure that data, size, and capacity values are consistent
    assert(0 == array->size);
    assert(0 == array->capacity);
  }
}

audio_gaze_msgs__msg__AudioCue__Sequence *
audio_gaze_msgs__msg__AudioCue__Sequence__create(size_t size)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  audio_gaze_msgs__msg__AudioCue__Sequence * array = (audio_gaze_msgs__msg__AudioCue__Sequence *)allocator.allocate(sizeof(audio_gaze_msgs__msg__AudioCue__Sequence), allocator.state);
  if (!array) {
    return NULL;
  }
  bool success = audio_gaze_msgs__msg__AudioCue__Sequence__init(array, size);
  if (!success) {
    allocator.deallocate(array, allocator.state);
    return NULL;
  }
  return array;
}

void
audio_gaze_msgs__msg__AudioCue__Sequence__destroy(audio_gaze_msgs__msg__AudioCue__Sequence * array)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  if (array) {
    audio_gaze_msgs__msg__AudioCue__Sequence__fini(array);
  }
  allocator.deallocate(array, allocator.state);
}

bool
audio_gaze_msgs__msg__AudioCue__Sequence__are_equal(const audio_gaze_msgs__msg__AudioCue__Sequence * lhs, const audio_gaze_msgs__msg__AudioCue__Sequence * rhs)
{
  if (!lhs || !rhs) {
    return false;
  }
  if (lhs->size != rhs->size) {
    return false;
  }
  for (size_t i = 0; i < lhs->size; ++i) {
    if (!audio_gaze_msgs__msg__AudioCue__are_equal(&(lhs->data[i]), &(rhs->data[i]))) {
      return false;
    }
  }
  return true;
}

bool
audio_gaze_msgs__msg__AudioCue__Sequence__copy(
  const audio_gaze_msgs__msg__AudioCue__Sequence * input,
  audio_gaze_msgs__msg__AudioCue__Sequence * output)
{
  if (!input || !output) {
    return false;
  }
  if (output->capacity < input->size) {
    const size_t allocation_size =
      input->size * sizeof(audio_gaze_msgs__msg__AudioCue);
    rcutils_allocator_t allocator = rcutils_get_default_allocator();
    audio_gaze_msgs__msg__AudioCue * data =
      (audio_gaze_msgs__msg__AudioCue *)allocator.reallocate(
      output->data, allocation_size, allocator.state);
    if (!data) {
      return false;
    }
    // If reallocation succeeded, memory may or may not have been moved
    // to fulfill the allocation request, invalidating output->data.
    output->data = data;
    for (size_t i = output->capacity; i < input->size; ++i) {
      if (!audio_gaze_msgs__msg__AudioCue__init(&output->data[i])) {
        // If initialization of any new item fails, roll back
        // all previously initialized items. Existing items
        // in output are to be left unmodified.
        for (; i-- > output->capacity; ) {
          audio_gaze_msgs__msg__AudioCue__fini(&output->data[i]);
        }
        return false;
      }
    }
    output->capacity = input->size;
  }
  output->size = input->size;
  for (size_t i = 0; i < input->size; ++i) {
    if (!audio_gaze_msgs__msg__AudioCue__copy(
        &(input->data[i]), &(output->data[i])))
    {
      return false;
    }
  }
  return true;
}
