from launch import LaunchDescription
from launch_ros.actions import Node


def generate_launch_description():
    return LaunchDescription([
        Node(
            package='audio_gaze',
            executable='mock_audio_node',
            name='mock_audio_node',
            output='screen',
        )
    ])
