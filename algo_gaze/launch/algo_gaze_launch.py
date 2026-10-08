"""
Unified launch file for algo_gaze — works with real OP3 hardware, the Webots
simulation, or a hybrid of the two, selected via the `mode` launch argument.

Usage:
    ros2 launch algo_gaze algo_gaze_launch.py mode:=real
    ros2 launch algo_gaze algo_gaze_launch.py mode:=sim
    ros2 launch algo_gaze algo_gaze_launch.py mode:=hybrid

    ros2 launch algo_gaze algo_gaze_launch.py mode:=real video_device:=/dev/video1
    ros2 launch algo_gaze algo_gaze_launch.py mode:=hybrid video_device:=/dev/video1

    # Real 2+ channel mic array, one launch instead of two:
    ros2 launch algo_gaze algo_gaze_launch.py mode:=hybrid audio_mode:=real_mic_array

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
    # Central tuning file -- edit algo_gaze/config/gaze_params.yaml directly to
    # change thresholds/timings/tolerances, no code changes or --ros-args -p
    # flags needed. Loaded as the FIRST parameters source below so the
    # mode-specific dict overrides (simulation_mode, the sim CGI-domain-gap
    # thresholds) still take precedence over it where they need to.
    gaze_params_yaml = os.path.join(
        get_package_share_directory('algo_gaze'), 'config', 'gaze_params.yaml')
    audio_sim_params_yaml = os.path.join(
        get_package_share_directory('audio_gaze'), 'config', 'sim_audio_params.yaml')
    audio_real_params_yaml = os.path.join(
        get_package_share_directory('audio_gaze'), 'config', 'real_audio_params.yaml')

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
    use_audio_arg = DeclareLaunchArgument(
        'use_audio',
        default_value='true',
        description="Whether to launch the audio perception pipeline at all."
    )
    audio_mode_arg = DeclareLaunchArgument(
        'audio_mode',
        default_value='mock',
        description="Audio perception backend: 'mock' (manual /audio/mock_trigger, no DSP), "
                    "'gcc_phat_sim' (real GCC-PHAT/SRP-PHAT DOA + delay-and-sum beamforming, "
                    "run against a synthetic mic-array signal -- no hardware needed), "
                    "'real_mono_mic' (real VAD from your actual laptop/USB mic -- direction_deg "
                    "is always 0.0, a single channel cannot measure direction of arrival), or "
                    "'real_mic_array' (real GCC-PHAT/SRP-PHAT DOA from a real 2+ channel device --"
                    " geometry/device settings come from real_audio_params.yaml, "
                    "edit that file before trusting any direction output, see README.md 4f). "
                    "All four publish the same audio_gaze_msgs/AudioCue on /audio/cue, so "
                    "algo_gaze itself needs no changes either way."
    )

    mode = LaunchConfiguration('mode')
    video_device = LaunchConfiguration('video_device')
    use_audio = LaunchConfiguration('use_audio')
    audio_mode = LaunchConfiguration('audio_mode')
    is_audio_mock = IfCondition(PythonExpression(
        ["'", use_audio, "' == 'true' and '", audio_mode, "' == 'mock'"]))
    is_audio_gcc_phat_sim = IfCondition(PythonExpression(
        ["'", use_audio, "' == 'true' and '", audio_mode, "' == 'gcc_phat_sim'"]))
    is_audio_real_mono_mic = IfCondition(PythonExpression(
        ["'", use_audio, "' == 'true' and '", audio_mode, "' == 'real_mono_mic'"]))
    is_audio_real_mic_array = IfCondition(PythonExpression(
        ["'", use_audio, "' == 'true' and '", audio_mode, "' == 'real_mic_array'"]))

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
        parameters=[gaze_params_yaml, {'simulation_mode': False}],
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
        # Lower detection thresholds than the real/hybrid default (0.5): Webots' CGI-rendered
        # pedestrians look visually different from the real photos YOLO/MediaPipe were trained
        # on (documented CGI domain-gap limitation in README.md section 5).
        parameters=[gaze_params_yaml, {
            'simulation_mode': True,
            'yolo_conf_threshold': 0.3,
            'mediapipe_min_detection_confidence': 0.3,
        }],
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
        parameters=[gaze_params_yaml, {'simulation_mode': True}],
        remappings=[
            # NOTE: image_raw is intentionally NOT remapped here -- usb_cam already
            # publishes the real camera feed on /image_raw, which is exactly what
            # algo_gaze subscribes to by default.
            ('/robotis/present_joint_states', '/robotis_op3/joint_states'),
        ]
    )

    # --- AUDIO PATH (hardware-independent of vision mode) ---
    # 'mock': manual /audio/mock_trigger -> forged AudioCue, no signal processing.
    # 'gcc_phat_sim': real GCC-PHAT/SRP-PHAT DOA + delay-and-sum beamforming, run
    # against sim_mic_array_node's synthetic mic-array signal (no hardware, no
    # Webots acoustics needed) -- same /audio/mock_trigger UX to fire a test event.
    # Both paths publish the same AudioCue on /audio/cue.
    mock_audio_node = Node(
        package='audio_gaze',
        executable='mock_audio_node',
        name='mock_audio_node',
        output='screen',
        condition=is_audio_mock,
    )

    sim_mic_array_node = Node(
        package='audio_gaze',
        executable='sim_mic_array_node',
        name='sim_mic_array_node',
        output='screen',
        condition=is_audio_gcc_phat_sim,
        parameters=[audio_sim_params_yaml],
    )

    gcc_phat_node = Node(
        package='audio_gaze',
        executable='gcc_phat_node',
        name='gcc_phat_node',
        output='screen',
        condition=is_audio_gcc_phat_sim,
        parameters=[audio_sim_params_yaml],
    )

    # 'real_mono_mic': real VAD from your actual mic, no DOA (see module docstring
    # in real_mono_mic_node.py for why -- one channel physically cannot measure
    # direction of arrival). Publishes AudioCue directly, no gcc_phat_node needed.
    real_mono_mic_node = Node(
        package='audio_gaze',
        executable='real_mono_mic_node',
        name='real_mono_mic_node',
        output='screen',
        condition=is_audio_real_mono_mic,
    )

    # 'real_mic_array': real GCC-PHAT/SRP-PHAT DOA from a real 2+ channel device --
    # same gcc_phat_node as gcc_phat_sim, only the signal source changes (see
    # README.md 4f). mic_array_radius_m/mic_start_angle_deg/mic_array_yaw_offset_deg
    # in real_audio_params.yaml are placeholders until you measure your real device.
    real_mic_array_node = Node(
        package='audio_gaze',
        executable='real_mic_array_node',
        name='real_mic_array_node',
        output='screen',
        condition=is_audio_real_mic_array,
        parameters=[audio_real_params_yaml],
    )

    gcc_phat_node_real = Node(
        package='audio_gaze',
        executable='gcc_phat_node',
        name='gcc_phat_node',
        output='screen',
        condition=is_audio_real_mic_array,
        parameters=[audio_real_params_yaml],
    )

    return LaunchDescription([
        mode_arg,
        video_device_arg,
        use_audio_arg,
        audio_mode_arg,
        usb_cam_node,
        gaze_node_real,
        webots_sim,
        gaze_node_sim,
        webots_sim_hybrid,
        gaze_node_hybrid,
        mock_audio_node,
        sim_mic_array_node,
        gcc_phat_node,
        real_mono_mic_node,
        real_mic_array_node,
        gcc_phat_node_real,
    ])
