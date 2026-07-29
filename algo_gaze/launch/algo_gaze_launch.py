"""
Unified launch file for algo_gaze — works with real OP3 hardware, the Webots
simulation, or a hybrid of the two, selected via the `mode` launch argument.

Usage:
    ros2 launch algo_gaze algo_gaze_launch.py mode:=real
    ros2 launch algo_gaze algo_gaze_launch.py mode:=sim
    ros2 launch algo_gaze algo_gaze_launch.py mode:=hybrid

    ros2 launch algo_gaze algo_gaze_launch.py mode:=real video_device:=/dev/video1
    ros2 launch algo_gaze algo_gaze_launch.py mode:=hybrid video_device:=/dev/video1

'hybrid' mode runs Webots (so the OP3's head physically moves in sim) but feeds
the vision pipeline from your real laptop/USB camera instead of Webots' rendered
camera feed -- avoids the YOLO-vs-CGI domain-gap issue while still testing the
full head-tracking control loop on the simulated robot.

Place this file at: algo_gaze/launch/algo_gaze_launch.py
(and register it in algo_gaze/setup.py's data_files launch list)
"""
import os
from launch import LaunchDescription
from launch.actions import DeclareLaunchArgument, IncludeLaunchDescription
from launch.conditions import IfCondition, UnlessCondition
from launch.launch_description_sources import PythonLaunchDescriptionSource
from launch.substitutions import LaunchConfiguration, PythonExpression
from launch_ros.actions import Node
from ament_index_python.packages import get_package_share_directory


def generate_launch_description():
    mode_arg = DeclareLaunchArgument(
        'mode',
        default_value='real',
        description="Run mode: 'real' (physical OP3 + usb_cam), 'sim' (Webots + Webots camera), "
                    "or 'hybrid' (Webots physics + your real laptop/USB camera)."
    )
    video_device_arg = DeclareLaunchArgument(
        'video_device',
        default_value='/dev/video0',
        description="Camera device path, used in 'real' and 'hybrid' modes."
    )

    mode = LaunchConfiguration('mode')
    video_device = LaunchConfiguration('video_device')

    is_real = IfCondition(PythonExpression(["'", mode, "' == 'real'"]))
    is_sim = IfCondition(PythonExpression(["'", mode, "' == 'sim'"]))
    is_hybrid = IfCondition(PythonExpression(["'", mode, "' == 'hybrid'"]))
    needs_real_cam = IfCondition(PythonExpression(["'", mode, "' == 'real' or '", mode, "' == 'hybrid'"]))

    # --- REAL HARDWARE PATH ---
    # Same as the original algo_gaze.py: bring up usb_cam, no topic remaps needed,
    # simulation_mode left False so only the real-robot JointState head command is sent.
    # (op3_manager itself is NOT launched here — start it separately if using real hardware,
    #  since it needs robot-specific config/serial-port setup.)
    #
    # --- HYBRID PATH ---
    # Also needs usb_cam (real camera -> /image_raw, same as real mode), but sends head
    # commands to Webots instead of real servos (simulation_mode=True), and remaps only
    # the joint_states feedback topic to Webots' -- NOT the image topic, since we want the
    # real webcam feed, not Webots' rendered camera.
    usb_cam_node = Node(
        package='usb_cam',
        executable='usb_cam_node_exe',
        name='usb_cam_node',
        output='screen',
        condition=needs_real_cam,
        parameters=[{
            'video_device': video_device,
            'framerate': 30.0,
            'image_width': 640,
            'image_height': 480,
            'pixel_format': 'yuyv'
        }]
    )

    gaze_node_real = Node(
        package='algo_gaze',
        executable='algo_gaze',
        name='algo_gaze_node',
        output='screen',
        condition=is_real,
        parameters=[{'simulation_mode': False}],
    )

    # --- SIMULATION PATH ---
    # Brings up Webots + remaps image/joint-state topics to match the sim's naming.
    # simulation_mode=True makes the node ALSO publish head commands as the
    # std_msgs/Float64 topics Webots expects (see main.py's publish_head_command()).
    webots_launch_dir = get_package_share_directory('op3_webots_ros2')
    webots_sim = IncludeLaunchDescription(
        PythonLaunchDescriptionSource(
            os.path.join(webots_launch_dir, 'launch', 'robot_launch.py')
        ),
        condition=is_sim,
    )
    webots_sim_hybrid = IncludeLaunchDescription(
        PythonLaunchDescriptionSource(
            os.path.join(webots_launch_dir, 'launch', 'robot_launch.py')
        ),
        condition=is_hybrid,
    )

    gaze_node_sim = Node(
        package='algo_gaze',
        executable='algo_gaze',
        name='algo_gaze_node',
        output='screen',
        condition=is_sim,
        parameters=[{'simulation_mode': True}],
        remappings=[
            ('/image_raw', '/robotis_op3/camera/image_raw'),
            ('/robotis/present_joint_states', '/robotis_op3/joint_states'),
        ]
    )

    gaze_node_hybrid = Node(
        package='algo_gaze',
        executable='algo_gaze',
        name='algo_gaze_node',
        output='screen',
        condition=is_hybrid,
        parameters=[{'simulation_mode': True}],
        remappings=[
            # NOTE: image_raw is intentionally NOT remapped here -- usb_cam already
            # publishes the real camera feed on /image_raw, which is exactly what
            # algo_gaze subscribes to by default.
            ('/robotis/present_joint_states', '/robotis_op3/joint_states'),
        ]
    )

    return LaunchDescription([
        mode_arg,
        video_device_arg,
        usb_cam_node,
        gaze_node_real,
        webots_sim,
        gaze_node_sim,
        webots_sim_hybrid,
        gaze_node_hybrid,
    ])