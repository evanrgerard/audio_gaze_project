"""Shared microphone-array geometry helpers for GCC-PHAT / beamforming.

Mic positions live in the array's own 2D horizontal-plane frame, with angles
measured counter-clockwise (standard math convention) from the array's local
+x axis. `direction_deg`, used everywhere else in this project (`AudioCue`,
`mock_audio_node`), follows the opposite convention: 0 = forward, negative =
left, positive = right. `math_angle_to_direction_deg` / `direction_deg_to_math_angle`
convert between the two; `yaw_offset_deg` corrects for how the array is
physically mounted relative to the robot's forward axis.
"""
import numpy as np


def wrap180(angle_deg):
    """Wrap an angle in degrees to (-180, 180]."""
    return (angle_deg + 180.0) % 360.0 - 180.0


def mic_positions(num_mics, radius_m, start_angle_deg=0.0):
    """Evenly-spaced circular array, (num_mics, 2) array of (x, y) meters."""
    angles_deg = start_angle_deg + np.arange(num_mics) * (360.0 / num_mics)
    angles_rad = np.deg2rad(angles_deg)
    return np.stack(
        [radius_m * np.cos(angles_rad), radius_m * np.sin(angles_rad)], axis=1)


def math_angle_to_direction_deg(theta_deg, yaw_offset_deg=0.0):
    return wrap180(yaw_offset_deg - theta_deg)


def direction_deg_to_math_angle(direction_deg, yaw_offset_deg=0.0):
    return wrap180(yaw_offset_deg - direction_deg)


def source_arrival_time(mic_pos, theta_deg, speed_of_sound_mps):
    """Arrival-time offset (seconds) at `mic_pos` for a far-field plane wave
    arriving from math-convention azimuth `theta_deg`, relative to the array
    origin. More negative = arrives earlier (mic sits closer to the source)."""
    u = np.array([np.cos(np.deg2rad(theta_deg)), np.sin(np.deg2rad(theta_deg))])
    return -float(np.asarray(mic_pos) @ u) / speed_of_sound_mps


def source_arrival_time_with_diffraction(mic_pos, theta_deg, speed_of_sound_mps, head_radius_m):
    """Like `source_arrival_time`, but modeling the extra path length sound
    must travel to diffract AROUND a rigid sphere (approximating the head/
    body the array is mounted on) to reach a mic in acoustic shadow -- the
    classic Woodworth (1938) geometric creeping-wave approximation, the same
    one used for interaural-time-difference spherical-head models, applied
    here to a general point on the sphere rather than just a pair of ears.

    Not physically exact (frequency-independent; treats the creeping wave as
    traveling at the free-space speed of sound), but captures the dominant
    real effect plain `source_arrival_time` ignores entirely: a mic on the
    far side of the mount hears the source measurably LATER than
    straight-line geometry alone predicts. Deliberately kept separate from
    `source_arrival_time` (used unchanged by `predicted_tdoa`/SRP-PHAT's DOA
    estimate) -- the point is testing how much a real, diffraction-naive
    estimator's accuracy degrades against a more realistic signal, not
    building a diffraction-aware estimator.

    `mic_pos` is used only for its direction from the array origin -- its
    distance is ignored; the mount is treated as a sphere of radius
    `head_radius_m` regardless of the array's own radius (a simplifying
    assumption for this coarse stress test, not a precise geometric model).
    """
    a = head_radius_m
    mic_dir_deg = np.degrees(np.arctan2(mic_pos[1], mic_pos[0]))
    alpha_deg = abs(wrap180(theta_deg - mic_dir_deg))
    alpha_rad = np.radians(alpha_deg)
    if alpha_deg <= 90.0:
        # Direct line of sight -- same straight-line prediction as source_arrival_time.
        return -a * np.cos(alpha_rad) / speed_of_sound_mps
    # In acoustic shadow: reaches the tangent point (90 deg from the source
    # direction, arriving in step with a wave reaching the sphere's center)
    # at time 0, then creeps the remaining arc (alpha - 90 deg) along the
    # surface at speed_of_sound.
    return a * (alpha_rad - np.pi / 2.0) / speed_of_sound_mps


def source_arrival_time_with_elevation(mic_pos, theta_deg, elevation_deg, speed_of_sound_mps):
    """Like `source_arrival_time`, but for a source that is NOT in the
    array's own horizontal plane -- e.g. a person's mouth at head height when
    the array is mounted lower (or higher) on the robot. `elevation_deg` is
    the angle above (+) or below (-) the array's horizontal plane; use
    `height_offset_to_elevation_deg` to get it from a height gap and range.

    For an array with every mic in the z=0 plane, the far-field unit vector
    toward a source at (azimuth theta_deg, elevation elevation_deg) is
    (cos(theta)*cos(phi), sin(theta)*cos(phi), sin(phi)); since every mic
    position has zero z-component, its dot product with that vector is just
    cos(phi) times the dot product with the flat (phi=0) unit vector. So the
    elevation's entire effect on a coplanar array's delays is a single
    multiplicative cos(elevation_deg) scaling of the flat-model delay -- this
    is exact for the far-field/planar-array case, not an approximation.

    This models a DIFFERENT effect than `source_arrival_time_with_diffraction`:
    that one is near-field shadowing around a rigid mount; this one is the
    geometric foreshortening a horizontal-plane-only estimator (srp_phat_doa,
    unchanged) can't see because it always assumes elevation_deg=0.
    """
    flat_tau = source_arrival_time(mic_pos, theta_deg, speed_of_sound_mps)
    return flat_tau * np.cos(np.radians(elevation_deg))


def height_offset_to_elevation_deg(height_offset_m, distance_m):
    """Convert a camera/mic height gap and horizontal interaction distance
    into the source elevation angle (degrees) `source_arrival_time_with_elevation`
    expects. height_offset_m: source height above the array's plane (e.g.
    camera/head height minus mic-array mount height, if the array is mounted
    lower on the torso); negative if the array is mounted higher than the
    source. distance_m: horizontal (ground-plane) distance to the source."""
    return float(np.degrees(np.arctan2(height_offset_m, distance_m)))


def predicted_tdoa(mic_i_pos, mic_ref_pos, theta_deg, speed_of_sound_mps):
    """Predicted TDOA (seconds) of the channel at `mic_i_pos` relative to the
    channel at `mic_ref_pos`, for a plane wave from azimuth `theta_deg`.
    Positive = mic_i lags the reference mic. Matches the sign convention
    returned by `gcc_phat.gcc_phat(sig_i, sig_ref)`."""
    return (source_arrival_time(mic_i_pos, theta_deg, speed_of_sound_mps)
            - source_arrival_time(mic_ref_pos, theta_deg, speed_of_sound_mps))
