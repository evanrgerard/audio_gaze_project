"""Offline correctness tests for the GCC-PHAT / SRP-PHAT / beamforming DSP.

No rclpy/ROS graph needed -- these exercise `audio_gaze.dsp` directly against
synthetic mic-array frames with a known, ground-truth direction, so the math
is validated independent of the simulation node or hardware.
"""
import numpy as np
import pytest

from audio_gaze.dsp.gcc_phat import gcc_phat, fractional_delay
from audio_gaze.dsp.beamformer import srp_phat_doa, delay_and_sum
from audio_gaze.dsp.simulate import simulate_plane_wave_frame

FS = 16000.0
SPEED_OF_SOUND = 343.0
MIC_RADIUS = 0.0463
NUM_MICS = 4


def _speech_like_source(num_samples, seed=0):
    """Broadband noise burst -- stands in for real speech energy, which is
    likewise broadband; GCC-PHAT/SRP-PHAT need broadband content to localize
    well (a pure tone is ambiguous)."""
    rng = np.random.default_rng(seed)
    return rng.normal(0.0, 1.0, size=num_samples)


def test_fractional_delay_matches_known_shift():
    source = _speech_like_source(4096)
    delayed = fractional_delay(source, 7.5)
    tau, _ = gcc_phat(delayed, source, FS, max_tau=0.01)
    assert tau == pytest.approx(7.5 / FS, abs=1.0 / FS)


@pytest.mark.parametrize('direction_deg', [-135, -90, -45, 0, 45, 90, 135, 179])
def test_srp_phat_doa_recovers_known_angle(direction_deg):
    source = _speech_like_source(2048, seed=int(direction_deg) + 1000)
    rng = np.random.default_rng(42)
    frame = simulate_plane_wave_frame(
        source, FS, NUM_MICS, MIC_RADIUS, direction_deg, SPEED_OF_SOUND,
        noise_std=0.01, rng=rng)

    estimated_deg, confidence, _, _ = srp_phat_doa(
        frame, FS, MIC_RADIUS, SPEED_OF_SOUND, grid_step_deg=1.0)

    diff = abs(((estimated_deg - direction_deg) + 180) % 360 - 180)
    assert diff <= 2.0, (
        f'expected ~{direction_deg} deg, got {estimated_deg} deg (diff {diff})')
    assert confidence > 0.3


def test_delay_and_sum_improves_snr_over_raw_channels():
    source = _speech_like_source(4096, seed=7)
    rng = np.random.default_rng(1)
    direction_deg = -30.0
    frame = simulate_plane_wave_frame(
        source, FS, NUM_MICS, MIC_RADIUS, direction_deg, SPEED_OF_SOUND,
        noise_std=0.2, rng=rng)

    enhanced = delay_and_sum(frame, FS, MIC_RADIUS, direction_deg, SPEED_OF_SOUND)

    def snr(sig):
        aligned_len = min(len(sig), len(source))
        noise = sig[:aligned_len] - source[:aligned_len]
        return 10 * np.log10(np.sum(source[:aligned_len] ** 2) / np.sum(noise ** 2))

    raw_snrs = [snr(frame[:, m]) for m in range(NUM_MICS)]
    assert snr(enhanced) > max(raw_snrs)


def test_srp_phat_doa_low_confidence_on_pure_noise():
    rng = np.random.default_rng(3)
    frame = rng.normal(0.0, 1.0, size=(2048, NUM_MICS))
    _, confidence, _, _ = srp_phat_doa(frame, FS, MIC_RADIUS, SPEED_OF_SOUND)
    assert confidence < 0.5
