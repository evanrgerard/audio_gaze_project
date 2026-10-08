#!/usr/bin/env python3
"""Real single-microphone VAD node -- for a laptop/USB mic with only one channel.

IMPORTANT LIMITATION: direction-of-arrival is mathematically impossible to
compute from a single microphone (GCC-PHAT/SRP-PHAT need >=2 channels with
known geometry to measure a time difference -- see gcc_phat_node for the real
multi-mic version). This node gives you REAL voice-activity detection from
real audio instead of mock_audio_node's manual trigger, but always publishes
`direction_deg: 0.0` -- it is NOT measured, just a fixed placeholder so the
message is well-formed. Practical effect on algo_gaze's fusion logic: only a
person near screen-center will ever "match" this cue (audio_match_tolerance_deg
around 0 deg) -- it cannot confirm *which* on-screen person is actually
speaking the way the real multi-mic pipeline can.

Publishes:
    /audio/cue   (audio_gaze_msgs/AudioCue)   direction_deg always 0.0

Parameters:
    sample_rate (int, default 16000)
    frame_size (int, default 1024) -- samples per VAD update
    input_device (int, default -1) -- sounddevice device index, -1 = system default
    vad_snr_threshold_db (float, default 9.0) -- dB above the adaptive noise
        floor before a frame is flagged as speech (same semantics as
        gcc_phat_node's VAD, so behavior is comparable across both nodes).
    noise_floor_alpha (float, default 0.95)

Requires the `sounddevice` package (pip install sounddevice); uses whatever
input device your system's audio backend (PulseAudio/ALSA) treats as default,
or pick one explicitly with `input_device` (run
`python3 -c "import sounddevice as sd; print(sd.query_devices())"` to list).
"""
import queue

import numpy as np
import rclpy
import sounddevice as sd
from rclpy.node import Node

from audio_gaze_msgs.msg import AudioCue


class RealMonoMicNode(Node):

    def __init__(self):
        super().__init__('real_mono_mic_node')

        self.declare_parameter('sample_rate', 16000)
        self.declare_parameter('frame_size', 1024)
        self.declare_parameter('input_device', -1)
        self.declare_parameter('vad_snr_threshold_db', 9.0)
        self.declare_parameter('noise_floor_alpha', 0.95)

        self.sample_rate = int(self.get_parameter('sample_rate').value)
        self.frame_size = int(self.get_parameter('frame_size').value)
        input_device = int(self.get_parameter('input_device').value)
        self.vad_snr_threshold_db = self.get_parameter('vad_snr_threshold_db').value
        self.noise_floor_alpha = self.get_parameter('noise_floor_alpha').value

        self._noise_floor_rms = 1e-3
        self._audio_queue = queue.Queue()

        self.cue_pub = self.create_publisher(AudioCue, '/audio/cue', 10)

        device = None if input_device < 0 else input_device
        try:
            self.stream = sd.InputStream(
                samplerate=self.sample_rate, channels=1, dtype='float32',
                blocksize=self.frame_size, device=device,
                callback=self._audio_callback)
            self.stream.start()
        except Exception as e:
            self.get_logger().error(
                f'Failed to open microphone input stream: {e}\n'
                'List available devices with:\n'
                '  python3 -c "import sounddevice as sd; print(sd.query_devices())"\n'
                'then set the input_device parameter to the right index.'
            )
            raise

        timer_period = self.frame_size / float(self.sample_rate)
        self.timer = self.create_timer(timer_period, self._process_queue)

        self.get_logger().info(
            f'Real single-mic VAD node running on device '
            f'{"default" if device is None else device} @ {self.sample_rate} Hz. '
            'direction_deg will always read 0.0 -- a single mic cannot measure '
            'direction of arrival (see gcc_phat_node for real multi-mic DOA).'
        )

    def _audio_callback(self, indata, frames, time_info, status):
        if status:
            self.get_logger().warn(f'Audio input stream status: {status}')
        # Runs on a separate audio thread -- keep it to just handing the block
        # off, no processing here (avoids underruns in the audio driver).
        self._audio_queue.put(indata[:, 0].copy())

    def _process_queue(self):
        while not self._audio_queue.empty():
            block = self._audio_queue.get()
            self._handle_block(block)

    def _handle_block(self, block: np.ndarray):
        rms = float(np.sqrt(np.mean(block.astype(np.float64) ** 2)) + 1e-12)
        snr_db = 20.0 * np.log10(rms / self._noise_floor_rms)
        is_speech = snr_db > self.vad_snr_threshold_db

        if is_speech:
            confidence = float(np.clip(snr_db / (2.0 * self.vad_snr_threshold_db), 0.0, 1.0))
        else:
            self._noise_floor_rms = (
                self.noise_floor_alpha * self._noise_floor_rms
                + (1.0 - self.noise_floor_alpha) * rms)
            confidence = 0.0

        cue = AudioCue()
        cue.header.stamp = self.get_clock().now().to_msg()
        cue.header.frame_id = 'mic_link'
        cue.is_speech = bool(is_speech)
        cue.direction_deg = 0.0  # not measurable from a single mic -- see module docstring
        cue.confidence = confidence
        self.cue_pub.publish(cue)

    def destroy_node(self):
        if hasattr(self, 'stream'):
            self.stream.stop()
            self.stream.close()
        super().destroy_node()


def main(args=None):
    rclpy.init(args=args)
    node = None
    try:
        node = RealMonoMicNode()
        rclpy.spin(node)
    except KeyboardInterrupt:
        pass
    finally:
        if node is not None:
            node.destroy_node()
        rclpy.try_shutdown()


if __name__ == '__main__':
    main()
