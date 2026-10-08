"""Real GCC-PHAT + SRP-PHAT beamforming pipeline, on real captured audio.

Brings up real_mic_array_node (opens a real multi-channel input device) feeding
the same unmodified gcc_phat_node used in simulation -- only the signal source
changes. This is the sim-to-real de-risking path: point it at any real 2+
channel device (many laptops' built-in mic array included) to validate the
capture driver and DOA math against real room acoustics/noise, ahead of a
purpose-built mic array.

All tunable values (device index, channel count, mic array geometry, VAD
threshold, ...) live in config/real_audio_params.yaml -- edit that file
directly, no code changes needed. It defaults to placeholder simulation
geometry; measure your real device's mic spacing/orientation and update
mic_array_radius_m/mic_start_angle_deg/mic_array_yaw_offset_deg there before
trusting any direction_deg output (see README.md section 4f/5).

List available input devices first:
    python3 -c "import sounddevice as sd; print(sd.query_devices())"

Then:
    ros2 launch audio_gaze audio_gcc_phat_real.launch.py

Need a quick one-off override without editing the file? Append raw overrides:
    ros2 launch audio_gaze audio_gcc_phat_real.launch.py --ros-args -p /real_mic_array_node:input_device:=4
"""
import os

from ament_index_python.packages import get_package_share_directory
from launch import LaunchDescription
from launch_ros.actions import Node


def generate_launch_description():
    params_yaml = os.path.join(
        get_package_share_directory('audio_gaze'), 'config', 'real_audio_params.yaml')

    return LaunchDescription([
        Node(
            package='audio_gaze',
            executable='real_mic_array_node',
            name='real_mic_array_node',
            output='screen',
            parameters=[params_yaml],
        ),
        Node(
            package='audio_gaze',
            executable='gcc_phat_node',
            name='gcc_phat_node',
            output='screen',
            parameters=[params_yaml],
        ),
    ])
