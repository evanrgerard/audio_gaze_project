#!/usr/bin/env python3
"""Real GCC-PHAT + SRP-PHAT beamforming sound-source-localization node.

Subscribes:
    /audio/mic_array_raw   (audio_gaze_msgs/MicArrayFrame)
        Synchronized multi-channel audio, from either a real mic-array
        capture driver or, hardware-free, sim_mic_array_node.

Publishes:
    /audio/cue          (audio_gaze_msgs/AudioCue)       -- same message/topic
                        as mock_audio_node, so algo_gaze's fusion logic needs
                        zero changes to consume real SSL instead of a manual
                        trigger.
    /audio/beamformed   (audio_gaze_msgs/BeamformedAudio) -- optional, the
                        delay-and-sum-enhanced mono signal steered at the
                        estimated direction (future ASR/VAD use).

Per frame:
    1. VAD: RMS energy on the reference channel against an adaptively
       tracked noise floor (so it works across different background noise
       levels, not just a fixed absolute threshold).
    2. If speech-like energy is present, SRP-PHAT scans a grid of candidate
       azimuths using GCC-PHAT cross-correlations between every channel and
       the reference channel, weighted by the array's known geometry, and
       picks the best-scoring direction (see audio_gaze.dsp.beamformer).
    3. Delay-and-sum beamforming steers a real beam at that direction to
       produce the enhanced signal published on /audio/beamformed.

Parameters:
    mic_array_radius_m (float, default 0.07) -- bigger radius = better angular
        resolution = higher SRP-PHAT confidence ceiling (confidence is limited
        by array geometry, not noise level -- a tiny array stays fuzzy about
        direction even with a perfectly clean signal). Must match
        sim_mic_array_node's value when testing in simulation.
    mic_start_angle_deg (float, default 0.0) -- math-convention angle of
        channel 0 in the array's own frame.
    mic_array_yaw_offset_deg (float, default 0.0) -- how the array is
        mounted relative to the robot's forward axis; tune this to calibrate.
    speed_of_sound_mps (float, default 343.0)
    doa_grid_step_deg (float, default 2.0)
    vad_snr_threshold_db (float, default 9.0) -- dB above the tracked noise
        floor before a frame is flagged as speech.
    noise_floor_alpha (float, default 0.95) -- smoothing factor for the
        adaptive noise-floor estimate (closer to 1 = slower to adapt).
    min_doa_confidence (float, default 0.3) -- SRP-PHAT frames scoring below
        this are still flagged is_speech (VAD already said so) but reported
        at the last-known-good direction rather than a noisy new one.
    noise_floor_warmup_sec (float, default 1.5) -- frames within this long of
        startup never trigger is_speech, no matter their SNR -- they only let
        the adaptive noise floor converge to the real ambient level first.
        Without this, the floor's initial seed value is arbitrary and frames
        right after launch can spuriously read as "speech" against it.
    publish_beamformed_audio (bool, default true)
"""
import numpy as np
import rclpy
from rclpy.node import Node

from audio_gaze_msgs.msg import AudioCue, BeamformedAudio, MicArrayFrame

from .dsp.beamformer import delay_and_sum, srp_phat_doa


class GccPhatNode(Node):

    def __init__(self):
        super().__init__('gcc_phat_node')

        self.declare_parameter('mic_array_radius_m', 0.07)
        self.declare_parameter('mic_start_angle_deg', 0.0)
        self.declare_parameter('mic_array_yaw_offset_deg', 0.0)
        self.declare_parameter('speed_of_sound_mps', 343.0)
        self.declare_parameter('doa_grid_step_deg', 2.0)
        self.declare_parameter('vad_snr_threshold_db', 9.0)
        self.declare_parameter('noise_floor_alpha', 0.95)
        self.declare_parameter('min_doa_confidence', 0.3)
        self.declare_parameter('noise_floor_warmup_sec', 1.5)
        self.declare_parameter('publish_beamformed_audio', True)

        self.mic_array_radius_m = self.get_parameter('mic_array_radius_m').value
        self.mic_start_angle_deg = self.get_parameter('mic_start_angle_deg').value
        self.mic_array_yaw_offset_deg = self.get_parameter('mic_array_yaw_offset_deg').value
        self.speed_of_sound_mps = self.get_parameter('speed_of_sound_mps').value
        self.doa_grid_step_deg = self.get_parameter('doa_grid_step_deg').value
        self.vad_snr_threshold_db = self.get_parameter('vad_snr_threshold_db').value
        self.noise_floor_alpha = self.get_parameter('noise_floor_alpha').value
        self.min_doa_confidence = self.get_parameter('min_doa_confidence').value
        self.noise_floor_warmup_sec = self.get_parameter('noise_floor_warmup_sec').value
        self.publish_beamformed_audio = self.get_parameter('publish_beamformed_audio').value

        self._noise_floor_rms = 1e-3  # arbitrary seed; converges to the real ambient level below
        self._last_direction_deg = 0.0
        self._start_time = self.get_clock().now()

        self.cue_pub = self.create_publisher(AudioCue, '/audio/cue', 10)
        self.beamformed_pub = self.create_publisher(
            BeamformedAudio, '/audio/beamformed', 10)
        self.frame_sub = self.create_subscription(
            MicArrayFrame, '/audio/mic_array_raw', self.frame_callback, 10)

        self.get_logger().info(
            'GCC-PHAT/SRP-PHAT SSL node running, listening on /audio/mic_array_raw. '
            'Feed it real capture-driver frames or run sim_mic_array_node for a '
            'hardware-free test.')

    def frame_callback(self, msg: MicArrayFrame):
        if msg.num_channels < 2:
            self.get_logger().warn(
                f'Need >=2 channels for GCC-PHAT, got {msg.num_channels}. Dropping frame.')
            return

        frame = np.asarray(msg.data, dtype=np.float64).reshape(
            msg.frame_size, msg.num_channels)

        rms = float(np.sqrt(np.mean(frame[:, 0] ** 2)) + 1e-12)
        snr_db = 20.0 * np.log10(rms / self._noise_floor_rms)
        warming_up = (self.get_clock().now() - self._start_time).nanoseconds / 1e9 < self.noise_floor_warmup_sec
        is_speech = (not warming_up) and snr_db > self.vad_snr_threshold_db

        if is_speech:
            direction_deg, confidence, _, _ = srp_phat_doa(
                frame, msg.sample_rate, self.mic_array_radius_m,
                self.speed_of_sound_mps, grid_step_deg=self.doa_grid_step_deg,
                mic_start_angle_deg=self.mic_start_angle_deg,
                yaw_offset_deg=self.mic_array_yaw_offset_deg)

            if confidence >= self.min_doa_confidence:
                self._last_direction_deg = direction_deg
            else:
                # Keep VAD's "someone is speaking" but don't trust this
                # frame's noisy DOA estimate over the last good one.
                direction_deg = self._last_direction_deg
        else:
            # Only track the noise floor during silence, so a sustained loud
            # voice doesn't get slowly absorbed into the "floor" and stop
            # tripping the VAD.
            self._noise_floor_rms = (
                self.noise_floor_alpha * self._noise_floor_rms
                + (1.0 - self.noise_floor_alpha) * rms)
            direction_deg = 0.0
            confidence = 0.0

        cue = AudioCue()
        cue.header = msg.header
        cue.header.frame_id = 'mic_array_link'
        cue.is_speech = bool(is_speech)
        cue.direction_deg = float(direction_deg) if is_speech else 0.0
        cue.confidence = float(confidence) if is_speech else 0.0
        self.cue_pub.publish(cue)

        if self.publish_beamformed_audio:
            steer_deg = direction_deg if is_speech else self._last_direction_deg
            enhanced = delay_and_sum(
                frame, msg.sample_rate, self.mic_array_radius_m, steer_deg,
                self.speed_of_sound_mps, mic_start_angle_deg=self.mic_start_angle_deg,
                yaw_offset_deg=self.mic_array_yaw_offset_deg)
            bf = BeamformedAudio()
            bf.header = msg.header
            bf.sample_rate = msg.sample_rate
            bf.direction_deg = float(steer_deg)
            bf.samples = enhanced.tolist()
            self.beamformed_pub.publish(bf)


def main(args=None):
    rclpy.init(args=args)
    node = GccPhatNode()
    try:
        rclpy.spin(node)
    except KeyboardInterrupt:
        pass
    finally:
        node.destroy_node()
        rclpy.try_shutdown()


if __name__ == '__main__':
    main()
