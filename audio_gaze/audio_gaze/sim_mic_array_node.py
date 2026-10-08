#!/usr/bin/env python3
"""Hardware-free multi-channel mic-array simulator.

Lets the GCC-PHAT/SRP-PHAT beamforming pipeline (gcc_phat_node) run and be
tested end-to-end -- signal in, direction out -- with no physical mic array
and no acoustic simulation in Webots (which this project's world files don't
model). Generates a synthetic far-field plane wave with the correct
per-channel geometric delays for a triggered direction (using the exact same
array-geometry math gcc_phat_node uses to *estimate* direction, so this is a
genuine closed-loop DSP test, not a re-creation of mock_audio_node's
"just forge the answer" shortcut) plus independent per-channel sensor noise
and a continuous ambient noise bed.

Publishes:
    /audio/mic_array_raw   (audio_gaze_msgs/MicArrayFrame)
        Continuously, at sample_rate / frame_size Hz. Ambient noise only when
        idle; a directional broadband burst overlaid on top while "speaking".

Subscribes:
    /audio/mock_trigger   (std_msgs/Float32)
        Same interface as mock_audio_node: publish a direction in degrees
        (AudioCue convention: 0=center, -=left, +=right) to simulate a person
        at that angle starting to speak for `speech_burst_duration` seconds.

Example manual test:
    ros2 topic pub --once /audio/mock_trigger std_msgs/msg/Float32 "{data: -25.0}"
    ros2 topic echo /audio/cue   # published by gcc_phat_node, consuming this node's frames

Parameters:
    sample_rate (int, default 16000)
    frame_size (int, default 1024) -- samples per channel per published frame
    num_mics (int, default 4)
    mic_array_radius_m (float, default 0.07) -- must match gcc_phat_node's value
    mic_start_angle_deg (float, default 0.0)
    mic_array_yaw_offset_deg (float, default 0.0)
    speed_of_sound_mps (float, default 343.0)
    speech_burst_duration (float, default 3.0)
    sensor_noise_std (float, default 0.03) -- per-channel independent noise
        added on top of the delayed source, simulating mic self-noise.
    ambient_noise_std (float, default 0.01) -- background noise level
        published continuously, even with no active trigger.
    speech_amplitude (float, default 0.5) -- std-dev of the simulated
        broadband "speech" source signal.
"""
import numpy as np
import rclpy
from rclpy.node import Node
from std_msgs.msg import Float32

from audio_gaze_msgs.msg import MicArrayFrame

from .dsp.simulate import simulate_plane_wave_frame


class SimMicArrayNode(Node):

    def __init__(self):
        super().__init__('sim_mic_array_node')

        self.declare_parameter('sample_rate', 16000)
        self.declare_parameter('frame_size', 1024)
        self.declare_parameter('num_mics', 4)
        self.declare_parameter('mic_array_radius_m', 0.07)
        self.declare_parameter('mic_start_angle_deg', 0.0)
        self.declare_parameter('mic_array_yaw_offset_deg', 0.0)
        self.declare_parameter('speed_of_sound_mps', 343.0)
        self.declare_parameter('speech_burst_duration', 3.0)
        self.declare_parameter('sensor_noise_std', 0.03)
        self.declare_parameter('ambient_noise_std', 0.01)
        self.declare_parameter('speech_amplitude', 0.5)

        self.sample_rate = int(self.get_parameter('sample_rate').value)
        self.frame_size = int(self.get_parameter('frame_size').value)
        self.num_mics = int(self.get_parameter('num_mics').value)
        self.mic_array_radius_m = self.get_parameter('mic_array_radius_m').value
        self.mic_start_angle_deg = self.get_parameter('mic_start_angle_deg').value
        self.mic_array_yaw_offset_deg = self.get_parameter('mic_array_yaw_offset_deg').value
        self.speed_of_sound_mps = self.get_parameter('speed_of_sound_mps').value
        self.speech_burst_duration = self.get_parameter('speech_burst_duration').value
        self.sensor_noise_std = self.get_parameter('sensor_noise_std').value
        self.ambient_noise_std = self.get_parameter('ambient_noise_std').value
        self.speech_amplitude = self.get_parameter('speech_amplitude').value

        self._rng = np.random.default_rng()
        self._current_direction_deg = 0.0
        self._speech_end_time = 0.0

        self.frame_pub = self.create_publisher(MicArrayFrame, '/audio/mic_array_raw', 10)
        self.trigger_sub = self.create_subscription(
            Float32, '/audio/mock_trigger', self.trigger_callback, 10)

        timer_period = self.frame_size / float(self.sample_rate)
        self.timer = self.create_timer(timer_period, self.publish_frame)

        self.get_logger().info(
            'Simulated mic-array node running (no hardware, no Webots acoustics needed). '
            'Trigger a fake speech event with:\n'
            "  ros2 topic pub --once /audio/mock_trigger std_msgs/msg/Float32 \"{data: -25.0}\""
        )

    def _now_sec(self):
        return self.get_clock().now().nanoseconds / 1e9

    def trigger_callback(self, msg: Float32):
        self._current_direction_deg = float(msg.data)
        self._speech_end_time = self._now_sec() + self.speech_burst_duration
        self.get_logger().info(
            f'SIM: simulating a speaker at {self._current_direction_deg:.1f} deg '
            f'for {self.speech_burst_duration:.1f}s'
        )

    def publish_frame(self):
        is_speech = self._now_sec() < self._speech_end_time

        if is_speech:
            source = self._rng.normal(0.0, self.speech_amplitude, size=self.frame_size)
            frame = simulate_plane_wave_frame(
                source, self.sample_rate, self.num_mics, self.mic_array_radius_m,
                self._current_direction_deg, self.speed_of_sound_mps,
                mic_start_angle_deg=self.mic_start_angle_deg,
                yaw_offset_deg=self.mic_array_yaw_offset_deg,
                noise_std=self.sensor_noise_std, rng=self._rng)
        else:
            frame = self._rng.normal(
                0.0, self.ambient_noise_std, size=(self.frame_size, self.num_mics))

        frame = np.clip(frame, -1.0, 1.0).astype(np.float32)

        msg = MicArrayFrame()
        msg.header.stamp = self.get_clock().now().to_msg()
        msg.header.frame_id = 'mic_array_link'
        msg.sample_rate = self.sample_rate
        msg.num_channels = self.num_mics
        msg.frame_size = self.frame_size
        msg.data = frame.reshape(-1).tolist()
        self.frame_pub.publish(msg)


def main(args=None):
    rclpy.init(args=args)
    node = SimMicArrayNode()
    try:
        rclpy.spin(node)
    except KeyboardInterrupt:
        pass
    finally:
        node.destroy_node()
        rclpy.try_shutdown()


if __name__ == '__main__':
    main()
