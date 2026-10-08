"""
Launch the dashboard alongside your main algo_gaze_launch.py, or standalone.

Usage:
    ros2 launch gaze_dashboard dashboard.launch.py
    ros2 launch gaze_dashboard dashboard.launch.py mode:=sim audio_backend:=gcc_phat_sim \
        http_port:=8080

`mode` controls which /robotis/present_joint_states remap to use, mirroring
algo_gaze_launch.py's own remapping (sim/hybrid publish joint states on
/robotis_op3/joint_states instead) -- so the "Pan/Tilt (actual)" panel reads
the right topic without you having to remap it by hand. `audio_backend` is
purely informational, shown in the dashboard's Config panel.
"""
from launch import LaunchDescription
from launch.actions import DeclareLaunchArgument
from launch.conditions import IfCondition, UnlessCondition
from launch.substitutions import LaunchConfiguration, PythonExpression
from launch_ros.actions import Node


def generate_launch_description():
    mode_arg = DeclareLaunchArgument(
        'mode',
        default_value='',
        description="Same value you launched algo_gaze_launch.py with ('real'/'sim'/'hybrid') "
                    "-- picks the matching joint_states topic and is shown in the Config panel."
    )
    audio_backend_arg = DeclareLaunchArgument(
        'audio_backend',
        default_value='',
        description="Same audio_mode you launched algo_gaze_launch.py with -- purely "
                    "informational, shown in the Config panel."
    )
    http_port_arg = DeclareLaunchArgument(
        'http_port',
        default_value='8080',
        description='Port for the dashboard web page.'
    )

    mode = LaunchConfiguration('mode')
    audio_backend = LaunchConfiguration('audio_backend')
    http_port = LaunchConfiguration('http_port')
    needs_remap = PythonExpression(["'", mode, "' == 'sim' or '", mode, "' == 'hybrid'"])

    common_params = [{'mode': mode, 'audio_backend': audio_backend, 'http_port': http_port}]

    dashboard_node_no_remap = Node(
        package='gaze_dashboard',
        executable='dashboard_node',
        name='gaze_dashboard_node',
        output='screen',
        condition=UnlessCondition(needs_remap),
        parameters=common_params,
    )

    dashboard_node_remapped = Node(
        package='gaze_dashboard',
        executable='dashboard_node',
        name='gaze_dashboard_node',
        output='screen',
        condition=IfCondition(needs_remap),
        parameters=common_params,
        remappings=[
            ('/robotis/present_joint_states', '/robotis_op3/joint_states'),
        ],
    )

    return LaunchDescription([
        mode_arg,
        audio_backend_arg,
        http_port_arg,
        dashboard_node_no_remap,
        dashboard_node_remapped,
    ])
