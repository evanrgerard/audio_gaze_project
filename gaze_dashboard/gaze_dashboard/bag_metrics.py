"""Summarize a recorded rosbag2 (mcap) experiment into the same style of
metrics the project's ad-hoc analyze_bag.py/analyze_exp*.py scripts have been
computing by hand per-scenario -- consolidated here as one reusable function
so the dashboard's Experiments panel and any future standalone script share
one implementation instead of re-deriving it a fifth time.

Expects the same two topics those scripts read:
    /experiment/gaze_metrics       (std_msgs/String, JSON -- see
        brone_gaze_node.py's `metrics` dict / algo_gaze's publish_telemetry)
    /robotis/present_joint_states  (sensor_msgs/JointState)
Both are optional -- a bag missing one just gets an empty section rather
than an error, since e.g. a pure-simulation run may not have real motor
joint_states, and an older/different pipeline may not publish gaze_metrics.
"""
import json
import math

from rclpy.serialization import deserialize_message
from rosbag2_py import ConverterOptions, SequentialReader, StorageOptions
from sensor_msgs.msg import JointState
from std_msgs.msg import String


def _stats(values):
    """min/max/avg for a list of numbers, or None if empty."""
    if not values:
        return None
    return {
        'min': min(values), 'max': max(values),
        'avg': sum(values) / len(values), 'n': len(values),
    }


def _rmse(a_vals, b_vals):
    """RMSE between two equal-length lists, or None if empty/mismatched."""
    if not a_vals or len(a_vals) != len(b_vals):
        return None
    sq = [(a - b) ** 2 for a, b in zip(a_vals, b_vals)]
    return math.sqrt(sum(sq) / len(sq))


def summarize_bag(bag_path):
    """Read a bag at `bag_path` (an mcap rosbag2 directory, e.g. what
    `ros2 bag record -s mcap -o <path> ...` produces) and return a
    JSON-serializable dict of summary statistics. Raises whatever
    rosbag2_py raises (e.g. the path doesn't exist / isn't a bag) --
    callers should catch and report that as a 404/400, not swallow it here.
    """
    reader = SequentialReader()
    storage_options = StorageOptions(uri=bag_path, storage_id='mcap')
    converter_options = ConverterOptions(
        input_serialization_format='cdr', output_serialization_format='cdr')
    reader.open(storage_options, converter_options)

    gaze_data = []
    joint_pans, joint_tilts = [], []
    t_start, t_end = None, None

    while reader.has_next():
        topic, data, t = reader.read_next()
        if t_start is None:
            t_start = t
        t_end = t

        if topic == '/experiment/gaze_metrics':
            msg = deserialize_message(data, String)
            try:
                gaze_data.append(json.loads(msg.data))
            except json.JSONDecodeError:
                pass
        elif topic == '/robotis/present_joint_states':
            msg = deserialize_message(data, JointState)
            if 'head_pan' in msg.name and 'head_tilt' in msg.name:
                joint_pans.append(msg.position[msg.name.index('head_pan')])
                joint_tilts.append(msg.position[msg.name.index('head_tilt')])

    result = {
        'bag_path': bag_path,
        'duration_sec': (t_end - t_start) / 1e9 if t_start is not None else 0.0,
        'gaze_metrics_frames': len(gaze_data),
        'joint_state_samples': len(joint_pans),
    }

    if gaze_data:
        fps = [d['fps'] for d in gaze_data if d.get('fps') is not None]
        latency = [d['ai_latency_ms'] for d in gaze_data if d.get('ai_latency_ms') is not None]
        error_px = [d['error_px'] for d in gaze_data if d.get('error_px') is not None]
        on_target = [bool(d.get('is_on_target')) for d in gaze_data]
        shift_ms = [d['gaze_shifting_time_ms'] for d in gaze_data
                    if d.get('gaze_shifting_time_ms') is not None]
        cmd_pan = [d['command_pan'] for d in gaze_data if d.get('command_pan') is not None]
        cmd_tilt = [d['command_tilt'] for d in gaze_data if d.get('command_tilt') is not None]
        act_pan = [d['actual_pan'] for d in gaze_data if d.get('actual_pan') is not None]
        act_tilt = [d['actual_tilt'] for d in gaze_data if d.get('actual_tilt') is not None]
        audio_active = [bool(d.get('audio_active')) for d in gaze_data]
        audio_matched = [bool(d.get('audio_matched_any')) for d in gaze_data]

        result['gaze'] = {
            'fps': _stats(fps),
            'latency_ms': _stats(latency),
            'error_px': _stats(error_px),
            'time_on_target_pct': 100.0 * sum(on_target) / len(on_target) if on_target else None,
            'gaze_shift_time_ms': _stats(shift_ms),
        }
        result['command_vs_actual'] = {
            'pan_rmse_rad': _rmse(cmd_pan, act_pan),
            'tilt_rmse_rad': _rmse(cmd_tilt, act_tilt),
            'command_pan': _stats(cmd_pan),
            'actual_pan': _stats(act_pan),
            'command_tilt': _stats(cmd_tilt),
            'actual_tilt': _stats(act_tilt),
        }
        result['audio'] = {
            'active_pct': 100.0 * sum(audio_active) / len(audio_active) if audio_active else None,
            'matched_pct': (
                100.0 * sum(audio_matched) / len(audio_matched) if audio_matched else None),
        }
        # audio_mode/mic_mount/simulation_mode are constant per-run on the real
        # node (brone_gaze_node.py) -- surface whichever the first frame says,
        # useful for labeling which experimental condition this bag is.
        first = gaze_data[0]
        result['condition'] = {
            k: first.get(k) for k in ('simulation_mode', 'audio_mode', 'mic_mount')
            if k in first
        }

    if joint_pans:
        result['motor'] = {
            'pan_rad': _stats(joint_pans),
            'tilt_rad': _stats(joint_tilts),
        }

    return result
