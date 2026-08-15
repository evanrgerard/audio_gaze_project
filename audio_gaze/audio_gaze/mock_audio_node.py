#!/usr/bin/env python3
"""
Mock audio perception node for Phase 1 development -- no microphone hardware needed.

Simulates what a real Voice-Activity-Detection + Direction-of-Arrival (VAD/DOA)
node (e.g. a ReSpeaker mic array driver, in Phase 2) would publish, so the
vision-audio fusion logic in algo_gaze can be built and tested end-to-end now.

Publishes:
    /audio/cue   (audio_gaze_msgs/AudioCue)   at a fixed rate (default 10 Hz)

Subscribes:
    /audio/mock_trigger   (std_msgs/Float32)
        Publish a direction in degrees (-180..180, 0 = camera-center, negative =
        left, positive = right) to simulate "someone at this angle just started
        speaking". The mock node holds is_speech=True at that direction for
        `speech_burst_duration` seconds, then automatically returns to silence
        -- mimicking a real, finite speech utterance.

Example manual test:
    ros2 topic pub --once /audio/mock_trigger std_msgs/msg/Float32 "{data: -25.0}"
    # -> simulates a person 25 degrees to the LEFT starting to speak for a few seconds

Parameters:
    publish_rate_hz (float, default 10.0)
    speech_burst_duration (float, default 3.0) -- seconds the mock speech event lasts
"""
import time

import rclpy
from rclpy.node import Node
from std_msgs.msg import Float32
from audio_gaze_msgs.msg import AudioCue


class MockAudioNode(Node):
    def __init__(self):
        super().__init__('mock_audio_node')

        self.declare_parameter('publish_rate_hz', 10.0)
        self.declare_parameter('speech_burst_duration', 3.0)

        publish_rate_hz = self.get_parameter('publish_rate_hz').value
        self.speech_burst_duration = self.get_parameter('speech_burst_duration').value

        self.current_direction_deg = 0.0
        self.speech_end_time = 0.0  # monotonic time when the current mock speech burst ends

        self.publisher = self.create_publisher(AudioCue, '/audio/cue', 10)
        self.trigger_sub = self.create_subscription(
            Float32, '/audio/mock_trigger', self.trigger_callback, 10)

        timer_period = 1.0 / publish_rate_hz
        self.timer = self.create_timer(timer_period, self.publish_cue)

        self.get_logger().info(
            'Mock audio node running. Trigger a fake speech event with:\n'
            "  ros2 topic pub --once /audio/mock_trigger std_msgs/msg/Float32 \"{data: -25.0}\""
        )

    def trigger_callback(self, msg: Float32):
        self.current_direction_deg = float(msg.data)
        self.speech_end_time = time.monotonic() + self.speech_burst_duration
        self.get_logger().info(
            f'MOCK: simulating speech at {self.current_direction_deg:.1f} deg '
            f'for {self.speech_burst_duration:.1f}s'
        )

    def publish_cue(self):
        is_speech = time.monotonic() < self.speech_end_time

        cue = AudioCue()
        cue.header.stamp = self.get_clock().now().to_msg()
        cue.header.frame_id = 'mic_array_link'
        cue.is_speech = is_speech
        cue.direction_deg = self.current_direction_deg if is_speech else 0.0
        cue.confidence = 1.0 if is_speech else 0.0
        self.publisher.publish(cue)


def main(args=None):
    rclpy.init(args=args)
    node = MockAudioNode()
    try:
        rclpy.spin(node)
    except KeyboardInterrupt:
        pass
    finally:
        node.destroy_node()
        rclpy.try_shutdown()


if __name__ == '__main__':
    main()
