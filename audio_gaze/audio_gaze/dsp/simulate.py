"""Synthetic multi-mic plane-wave signal generator.

Used both by the offline DSP unit tests (ground-truth check that GCC-PHAT /
SRP-PHAT recover a known simulated angle) and by `sim_mic_array_node` (so the
whole GCC-PHAT + beamforming pipeline can be exercised end-to-end with no
physical mic-array hardware -- e.g. in Webots `sim`/`hybrid` mode, or
standalone).
"""
import numpy as np

from .array_geometry import (
    mic_positions, source_arrival_time, source_arrival_time_with_diffraction,
    height_offset_to_elevation_deg, direction_deg_to_math_angle, wrap180,
)
from .gcc_phat import fractional_delay


def head_shadow_attenuation_db(mic_pos, theta_deg, max_attenuation_db):
    """Coarse stress-test model of acoustic shadowing by whatever body the
    array is mounted on (e.g. the OP3's own head/shoulders) -- something the
    free-field simulation otherwise ignores entirely, and that a real mount
    can't avoid. NOT a physically exact HRTF/diffraction model: real
    shadowing is frequency-dependent (worse at high frequencies) and the true
    diffraction pattern is more complex than this smooth cosine taper. Good
    enough to answer "how much confidence margin do I actually have if some
    mics lose several dB to self-shadowing," not to predict exact real dB.

    mic_pos: this mic's (x, y) position in the array's own frame -- used only
        for its direction from the array center (mic's own "outward normal").
    theta_deg: source azimuth, math convention (see array_geometry).
    max_attenuation_db: attenuation when the source is directly behind this
        mic (full shadow). 0 attenuation when the source is directly in front.
    """
    mic_dir_deg = np.degrees(np.arctan2(mic_pos[1], mic_pos[0]))
    angle_diff = wrap180(theta_deg - mic_dir_deg)
    shadow_frac = (1.0 - np.cos(np.radians(angle_diff))) / 2.0  # 0 (facing source) .. 1 (facing away)
    return -max_attenuation_db * shadow_frac


def simulate_plane_wave_frame(
        source, fs, num_mics, mic_radius_m, direction_deg, speed_of_sound_mps=343.0,
        mic_start_angle_deg=0.0, yaw_offset_deg=0.0, noise_std=0.0, rng=None,
        head_shadow_max_db=0.0, head_diffraction_radius_m=None,
        mic_height_offset_m=None, source_distance_m=None):
    """source: (num_samples,) 1-D waveform, treated as the wavefront at the
    array's geometric origin. Returns an (num_samples, num_mics) frame where
    each channel is `source` delayed per the array geometry for a far-field
    plane wave arriving from `direction_deg` (AudioCue convention), plus
    independent sensor noise per channel.

    head_shadow_max_db (default 0.0, i.e. off): if > 0, applies
    head_shadow_attenuation_db per channel -- see that function's docstring.
    Amplitude-only; found to barely affect GCC-PHAT (it's phase-based, so
    amplitude loss alone matters little as long as a channel stays above the
    noise floor).

    head_diffraction_radius_m (default None, i.e. off): if set, uses
    source_arrival_time_with_diffraction instead of the plain straight-line
    source_arrival_time -- models the REAL effect a naive free-field DOA
    estimator can't see: shadowed mics hearing the source measurably later
    than straight-line geometry predicts. This is the one that actually
    matters; use it to see how much bias/confidence loss a diffraction-naive
    estimator (like srp_phat_doa, unchanged) suffers against a more
    realistic signal, ahead of touching real hardware.

    mic_height_offset_m / source_distance_m (default None, i.e. off): set
    BOTH to simulate the array and the sound source (e.g. a person's mouth)
    NOT being in the same horizontal plane -- e.g. the mic array mounted on
    the torso while the camera (and the speaker's mouth) is at head height.
    Converted internally to an elevation angle via
    array_geometry.height_offset_to_elevation_deg, then applied as a
    cos(elevation) delay scaling (see
    source_arrival_time_with_elevation) -- the geometric foreshortening
    effect a horizontal-plane-only estimator like srp_phat_doa (unchanged,
    always assumes elevation=0) cannot see. Distinct from and stacked
    (multiplicatively) with head_diffraction_radius_m if both are set: this
    is an approximation for a stress test, not a claim that the two effects
    combine exactly this way physically.
    """
    if rng is None:
        rng = np.random.default_rng()
    positions = mic_positions(num_mics, mic_radius_m, mic_start_angle_deg)
    theta = direction_deg_to_math_angle(direction_deg, yaw_offset_deg)

    elevation_deg = None
    if mic_height_offset_m is not None and source_distance_m is not None:
        elevation_deg = height_offset_to_elevation_deg(mic_height_offset_m, source_distance_m)
        elevation_scale = np.cos(np.radians(elevation_deg))

    frame = np.zeros((source.shape[0], num_mics), dtype=np.float64)
    for m in range(num_mics):
        if head_diffraction_radius_m is not None:
            tau = source_arrival_time_with_diffraction(
                positions[m], theta, speed_of_sound_mps, head_diffraction_radius_m)
        else:
            tau = source_arrival_time(positions[m], theta, speed_of_sound_mps)
        if elevation_deg is not None:
            tau *= elevation_scale
        channel = fractional_delay(source, tau * fs)
        if head_shadow_max_db > 0.0:
            atten_db = head_shadow_attenuation_db(positions[m], theta, head_shadow_max_db)
            channel = channel * (10.0 ** (atten_db / 20.0))
        frame[:, m] = channel
        if noise_std > 0.0:
            frame[:, m] += rng.normal(0.0, noise_std, size=source.shape[0])
    return frame
