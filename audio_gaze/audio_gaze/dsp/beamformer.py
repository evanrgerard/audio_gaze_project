"""Beamforming: SRP-PHAT direction-of-arrival scan + delay-and-sum enhancement.

SRP-PHAT ("Steered Response Power - PHAT") is the beamforming counterpart to
plain pairwise GCC-PHAT: instead of estimating one TDOA per mic pair and
solving for the angle algebraically (fragile -- any one bad pair estimate
throws off the whole result), it steers a virtual beam through a grid of
candidate angles and scores each one by how well ALL mic pairs' GCC-PHAT
correlation functions agree with that angle's geometrically-predicted delays.
The angle with the highest total agreement is the DOA -- beamforming used as
a direction finder. `delay_and_sum` then steers a real beam at the winning
angle to produce an enhanced (denoised) mono signal, e.g. for VAD or future
ASR use.
"""
import numpy as np

from .array_geometry import (
    direction_deg_to_math_angle,
    math_angle_to_direction_deg,
    mic_positions,
    predicted_tdoa,
)
from .gcc_phat import fractional_delay, gcc_phat


def srp_phat_doa(
        frame, fs, mic_radius_m, speed_of_sound_mps, grid_step_deg=2.0,
        max_tau_margin=1.2, mic_start_angle_deg=0.0, yaw_offset_deg=0.0):
    """Estimate direction of arrival for a multi-channel frame.

    frame: (num_samples, num_mics) array, channel 0 used as the GCC-PHAT
    reference. Returns `(direction_deg, confidence, theta_grid_deg, score)`,
    where `confidence` in [0, 1] is the normalized gap between the best and
    second-best candidate angle (a sharp, unambiguous peak scores near 1;
    a flat/noisy response scores near 0).
    """
    num_mics = frame.shape[1]
    positions = mic_positions(num_mics, mic_radius_m, mic_start_angle_deg)
    max_tau = max_tau_margin * (2.0 * mic_radius_m) / speed_of_sound_mps
    interp = 16

    ref = frame[:, 0]
    n = frame.shape[0] * 2
    max_shift = min(int(interp * fs * max_tau), interp * n // 2)

    theta_grid = np.arange(-180.0, 180.0, grid_step_deg)
    score = np.zeros_like(theta_grid)

    for m in range(1, num_mics):
        _, cc = gcc_phat(frame[:, m], ref, fs, max_tau=max_tau, interp=interp)
        taus_pred = np.array([
            predicted_tdoa(positions[m], positions[0], theta, speed_of_sound_mps)
            for theta in theta_grid
        ])
        idx = np.clip(
            np.round(taus_pred * fs * interp).astype(int) + max_shift,
            0, len(cc) - 1)
        score += cc[idx]

    best_k = int(np.argmax(score))
    best_theta = float(theta_grid[best_k])

    # Confidence = how many standard deviations the peak sits above the mean
    # of the whole steered-response curve. For an array this small, the
    # mainlobe is many degrees wide (coarse spatial aperture), so comparing
    # the peak to its immediate runner-up would always read as "no
    # confidence" even for a clean detection -- a z-score against the full
    # curve is robust to that and, empirically (see test_gcc_phat_dsp.py),
    # separates a real directional source (z ~ 3-4) from pure noise (z ~ 2,
    # an artifact of taking the max of ~360 correlated samples) reasonably
    # well. z=2 -> 0 confidence, z=4 -> full confidence.
    peak = float(score[best_k])
    z = (peak - float(np.mean(score))) / (float(np.std(score)) + 1e-9)
    confidence = float(np.clip((z - 2.0) / 2.0, 0.0, 1.0))

    direction_deg = math_angle_to_direction_deg(best_theta, yaw_offset_deg)
    return direction_deg, confidence, theta_grid, score


def delay_and_sum(
        frame, fs, mic_radius_m, direction_deg, speed_of_sound_mps,
        mic_start_angle_deg=0.0, yaw_offset_deg=0.0):
    """Steer a delay-and-sum beam at `direction_deg` (AudioCue convention,
    0=forward/-=left/+=right) and return the enhanced mono signal: each
    channel's relative delay for that direction is removed (time-aligned back
    to channel 0), then averaged, so in-beam signal adds coherently while
    off-axis noise/interference partially cancels."""
    num_mics = frame.shape[1]
    positions = mic_positions(num_mics, mic_radius_m, mic_start_angle_deg)
    theta = direction_deg_to_math_angle(direction_deg, yaw_offset_deg)

    aligned = np.zeros(frame.shape[0], dtype=np.float64)
    for m in range(num_mics):
        tau = predicted_tdoa(positions[m], positions[0], theta, speed_of_sound_mps)
        aligned += fractional_delay(frame[:, m], -tau * fs)
    return (aligned / num_mics).astype(np.float32)
