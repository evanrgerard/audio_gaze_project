#!/usr/bin/env python3
"""Real multi-channel mic-array capture driver.

Opens a real audio input device (any device your OS/PulseAudio/ALSA exposes,
including a laptop's built-in multi-mic array -- many modern laptops have 2+
physical mic capsules for noise cancellation, which is enough to validate this
whole driver + gcc_phat_node pipeline against real captured audio, real room
reverberation, and real background noise -- *before* a purpose-built mic array
(e.g. ReSpeaker) is in hand) and publishes MicArrayFrame, exactly like
sim_mic_array_node does. Feed it straight into the existing, unmodified
gcc_phat_node -- no new DSP code needed, this only replaces the signal source.

List available devices and their channel counts:
    python3 -c "import sounddevice as sd; print(sd.query_devices())"

IMPORTANT -- mic_array_radius_m / mic_start_angle_deg / mic_array_yaw_offset_deg
default to the SAME placeholder numbers as sim_mic_array_node. These are almost
certainly wrong for your actual device: measure the real capsule spacing and
mounting orientation and override them, or every direction estimate will be
calibrated against a fictional geometry. For a 2-mic linear array specifically,
mic_positions()'s circular formula degenerates correctly to two mics 2*radius
apart on a line (angles 0 deg and 180 deg) -- just make sure mic_array_radius_m
is HALF your measured true mic spacing, and mic_start_angle_deg/yaw_offset_deg
match which physical mic is "channel 0" and which way the pair actually faces.

Publishes:
    /audio/mic_array_raw   (audio_gaze_msgs/MicArrayFrame)

Parameters:
    sample_rate (int, default 16000)
    frame_size (int, default 1024)
    num_channels (int, default 2) -- channels to open ON THE DEVICE. For a
        device that mixes raw mic channels with its own processed/beamformed
        output on the same stream (e.g. the ReSpeaker XVF3800's 6-channel
        firmware: ch0-1 = onboard-processed audio, ch2-5 = mic 0-3 raw), this
        must be the device's FULL channel count, not just the raw mic count
        -- use raw_channel_indices below to publish only the raw subset.
    raw_channel_indices (int[], default []) -- if non-empty, only these
        0-indexed columns of the num_channels-wide capture are published
        (MicArrayFrame.num_channels becomes len(raw_channel_indices)); the
        rest (e.g. the device's own processed/beamformed channels) are
        dropped before they ever reach gcc_phat_node. GCC-PHAT needs the
        unprocessed inter-mic phase relationship -- feeding it a device's own
        already-beamformed output as if it were one more raw mic corrupts
        the array geometry math. Leave empty (default) to publish every
        opened channel as-is, e.g. a plain 2-mic laptop array with no onboard
        processed channel to exclude.
    input_device (int, default -1) -- sounddevice device index, -1 = system default
"""
import queue

import numpy as np
import rclpy
import sounddevice as sd
from rcl_interfaces.msg import ParameterDescriptor, ParameterType
from rclpy.node import Node

from audio_gaze_msgs.msg import MicArrayFrame


class RealMicArrayNode(Node):

    def __init__(self):
        super().__init__('real_mic_array_node')

        self.declare_parameter('sample_rate', 16000)
        self.declare_parameter('frame_size', 1024)
        self.declare_parameter('num_channels', 2)
        self.declare_parameter(
            'raw_channel_indices', [],
            ParameterDescriptor(type=ParameterType.PARAMETER_INTEGER_ARRAY))
        self.declare_parameter('input_device', -1)

        self.sample_rate = int(self.get_parameter('sample_rate').value)
        self.frame_size = int(self.get_parameter('frame_size').value)
        self.num_channels = int(self.get_parameter('num_channels').value)
        self.raw_channel_indices = list(self.get_parameter('raw_channel_indices').value)
        input_device = int(self.get_parameter('input_device').value)

        if self.raw_channel_indices and max(self.raw_channel_indices) >= self.num_channels:
            self.get_logger().error(
                f'raw_channel_indices {self.raw_channel_indices} out of range for a '
                f'{self.num_channels}-channel device stream -- fix the config.')
            raise ValueError('raw_channel_indices out of range')

        self._audio_queue = queue.Queue()
        self.frame_pub = self.create_publisher(MicArrayFrame, '/audio/mic_array_raw', 10)

        device = None if input_device < 0 else input_device
        try:
            self.stream = sd.InputStream(
                samplerate=self.sample_rate, channels=self.num_channels, dtype='float32',
                blocksize=self.frame_size, device=device,
                callback=self._audio_callback)
            self.stream.start()
        except Exception as e:
            self.get_logger().error(
                f'Failed to open {self.num_channels}-channel input stream: {e}\n'
                'List devices with: python3 -c "import sounddevice as sd; '
                'print(sd.query_devices())" and check num_channels matches what '
                "the device actually offers (max_input_channels)."
            )
            raise

        timer_period = self.frame_size / float(self.sample_rate)
        self.timer = self.create_timer(timer_period, self._process_queue)

        published_channels = len(self.raw_channel_indices) if self.raw_channel_indices \
            else self.num_channels
        channel_note = (
            f', publishing only raw channels {self.raw_channel_indices}'
            if self.raw_channel_indices else ''
        )
        self.get_logger().info(
            f'Real mic array capture running on device '
            f'{"default" if device is None else device} @ {self.sample_rate} Hz, '
            f'{self.num_channels}-channel stream opened{channel_note} -> '
            f'/audio/mic_array_raw as {published_channels} channel(s). Point '
            f'gcc_phat_node at this (it already is, by default) -- but verify '
            f'mic_array_radius_m/mic_start_angle_deg/mic_array_yaw_offset_deg on '
            f'gcc_phat_node match your REAL measured geometry, not the simulation '
            f'defaults.'
        )

    def _audio_callback(self, indata, frames, time_info, status):
        if status:
            self.get_logger().warn(f'Audio input stream status: {status}')
        self._audio_queue.put(indata.copy())

    def _process_queue(self):
        while not self._audio_queue.empty():
            block = self._audio_queue.get()
            self._publish_frame(block)

    def _publish_frame(self, block: np.ndarray):
        if self.raw_channel_indices:
            block = block[:, self.raw_channel_indices]

        msg = MicArrayFrame()
        msg.header.stamp = self.get_clock().now().to_msg()
        msg.header.frame_id = 'mic_array_link'
        msg.sample_rate = self.sample_rate
        msg.num_channels = block.shape[1]
        msg.frame_size = block.shape[0]
        msg.data = np.clip(block, -1.0, 1.0).astype(np.float32).reshape(-1).tolist()
        self.frame_pub.publish(msg)

    def destroy_node(self):
        if hasattr(self, 'stream'):
            self.stream.stop()
            self.stream.close()
        super().destroy_node()


def main(args=None):
    rclpy.init(args=args)
    node = None
    try:
        node = RealMicArrayNode()
        rclpy.spin(node)
    except KeyboardInterrupt:
        pass
    finally:
        if node is not None:
            node.destroy_node()
        rclpy.try_shutdown()


if __name__ == '__main__':
    main()
