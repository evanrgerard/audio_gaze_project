"""Hardware-free GCC-PHAT + SRP-PHAT beamforming pipeline.

Brings up sim_mic_array_node (synthesizes a directional multi-channel mic
signal, no hardware/Webots-acoustics required) feeding gcc_phat_node (real
GCC-PHAT/SRP-PHAT DOA estimation + delay-and-sum beamforming), which
publishes the same audio_gaze_msgs/AudioCue on /audio/cue that
mock_audio_node does -- so algo_gaze's fusion logic runs completely
unchanged. This is the sim-mode counterpart to audio_mock.launch.py; select
between the two via algo_gaze_launch.py's `audio_mode` argument.

All tunable values (VAD threshold, mic array geometry, noise levels, ...)
live in config/sim_audio_params.yaml -- edit that file directly, no code
changes or --ros-args -p flags needed.

Trigger a simulated speaker exactly as with the mock node:
    ros2 topic pub --once /audio/mock_trigger std_msgs/msg/Float32 "{data: -25.0}"
"""
import os

from ament_index_python.packages import get_package_share_directory
from launch import LaunchDescription
from launch_ros.actions import Node


def generate_launch_description():
    params_yaml = os.path.join(
        get_package_share_directory('audio_gaze'), 'config', 'sim_audio_params.yaml')

    return LaunchDescription([
        Node(
            package='audio_gaze',
            executable='sim_mic_array_node',
            name='sim_mic_array_node',
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
