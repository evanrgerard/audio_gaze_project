# algo_gaze_project

A ROS 2 (Humble) workspace for a **social attention system on the ROBOTIS OP3 humanoid robot**.
The robot watches a scene through a camera, detects the people in it, decides *who* it
should look at right now (based on proximity, gestures, gaze direction, and speech), and
physically drives its head servos to track that person — including a "nod" gesture once
it's locked on for a couple of seconds.

The system can run against the **real OP3 hardware**, a **Webots simulation**, or a
**hybrid** of the two (simulated robot body + your real laptop webcam). A **Phase 2**
extension is in progress to fuse in real microphone-array audio (voice direction +
activity detection) alongside vision — see [Roadmap](#roadmap-audiovision-fusion) below.

---

## 1. Repository map — what's ROBOTIS stock code vs. custom project code

This workspace is a mix of **imported, unmodified ROBOTIS OP3 framework packages**
(the robot's "body" — drivers, kinematics, controllers) and a **small set of custom
packages that are the actual project** (the "brain" — what to look at, and why).

| Folder | What it is | Did we write it? |
|---|---|---|
| `ROBOTIS-OP3/` | Core robot stack: `op3_manager` (hardware bring-up), `robotis_controller`, `op3_head_control_module`, `op3_walking_module`, `op3_kinematics_dynamics`, `op3_localization`, etc. | **No** — stock ROBOTIS packages, ported to ROS 2. Only touched to fix ROS 2 Humble build errors (see [Known Fixes](#known-portingbuild-fixes-already-applied)). |
| `ROBOTIS-OP3-Demo/` | Demo packages bundled with the stock robot, e.g. `op3_ball_detector` (a color-blob ball tracker — **not** used by our gaze logic, but its message package `op3_ball_detector_msgs` is repurposed, see below). | No |
| `ROBOTIS-OP3-ETC/` | Misc. extras: `usb_cam` (camera driver), `face_detection` (a separate, unrelated Haar-cascade face tracker demo), audio players. | No |
| `ROBOTIS-OP3-Simulations/op3_webots_ros2/` | The Webots simulation bridge: spawns the OP3 in a 3D world and exposes its camera/joints/servos over ROS 2 topics as an "extern controller." | No (stock), only the **world file** (`worlds/robotis_op3_extern.wbt`) is edited by us to add pedestrian test subjects. |
| `DynamixelSDK/`, `robotis_framework_common/`, `robotis_math/`, msg packages (`*_msgs`) | Low-level servo communication and shared message/service types the framework depends on. | No |
| **`algo_gaze/`** | **★ The main project.** Vision detection (YOLO + MediaPipe), fuzzy-logic attention scoring, head-tracking control, nodding behavior, CSV experiment logging. | **Yes — this is the thesis/project code.** |
| **`brone_gaze/`** | Two earlier/experimental variants of the same idea: `linear_model_node.py` (simple weighted-sum scoring instead of fuzzy logic) and `fuzzy_model_node.py` (fuzzy scoring, but without head-servo output — detection/visualization only). Kept for comparison. | **Yes** |
| **`audio_gaze_msgs/`, `audio_gaze/`** | Phase 2 in progress: custom `AudioCue` message + a mock audio node (no mic hardware yet) for building/testing the vision-audio fusion logic ahead of the real microphone array arriving. | **Yes** |

**In one sentence:** everything under `ROBOTIS-OP3*` and `DynamixelSDK`/`robotis_*` is
"the robot's body," imported wholesale from ROBOTIS; `algo_gaze` (and its `brone_gaze`
siblings and the new `audio_gaze*` packages) is "the robot's decision-making," and is
the actual contribution of this project.

---

## 2. How the vision/attention algorithm works (`algo_gaze/algo_gaze/main.py`)

This is the file that matters most. Per camera frame, it does:

1. **Detect people** — YOLOv11 (`yolo11s.pt`, bundled in `algo_gaze/algo_gaze/models/`)
   runs person detection + tracking (`classes=0` = person only, `conf=0.5` confidence
   threshold) to get a bounding box + persistent ID per person in frame.
2. **Extract per-person cues** — for each detected person, MediaPipe Holistic (pose +
   face landmarks) estimates:
   - `proximity` — how large their bounding box is relative to the largest person in frame (closer = bigger = higher score)
   - `angle` — how far off-center they are horizontally
   - `pointing_gesture` — are they pointing at the robot/camera?
   - `waving_gesture` — are they waving?
   - `body_orientation` — are they facing the robot?
   - `direct_gaze` — are they looking at the camera?
   - `speech_status` — **currently a visual proxy**: variance of mouth-landmark distance over the last 10 frames (high variance ≈ mouth moving ≈ probably talking). This is the exact piece Phase 2 replaces/augments with real audio.
3. **Fuzzy inference** (`skfuzzy`) — all six cues feed a Mamdani fuzzy controller that
   outputs a single `priority` score per person. Scores are smoothed frame-to-frame
   (exponential moving average, `alpha_score`) to avoid target-switching on noisy
   single-frame spikes.
4. **Pick a target & track it** — the highest-scoring person becomes the gaze target.
   `track_face_smooth()` converts their pixel position into a pan/tilt correction
   (with a 10% dead-zone near center to avoid jitter), applies a smoothing gain, and
   calls `publish_head_command()` to send it to the servos.
5. **Nod on lock-on** — if the same target is held for ~2 seconds, `process_nodding_animation()`
   plays a sinusoidal nod gesture on the tilt joint.
6. **Publish results** — an annotated debug image (`/gaze_model/annotated_image`, boxes +
   face mesh + target label), and (repurposed) `op3_ball_detector_msgs/CircleSetStamped`
   messages broadcasting the target's pixel coordinates on `/ball_detector_node/circle_set`
   (this reuses the ball-detector's message type as a convenient existing "point of
   interest" message — it's not doing ball detection).
7. **Optional CSV logging** — toggled via `/experiment/trigger` (`std_msgs/Bool`), records
   per-frame latency, FPS, pixel error, on-target flag, and pan/tilt angle for later analysis.

### `head_pub` vs. Webots output (`simulation_mode`)
`publish_head_command()` always publishes the real-hardware message
(`sensor_msgs/JointState` on `/robotis/head_control/set_joint_states`). When the
`simulation_mode` ROS parameter is `true`, it **additionally** publishes two
`std_msgs/Float64` topics (`/robotis_op3/head_pan_position/command`,
`/robotis_op3/head_tilt_position/command`) — the format Webots' extern controller
expects, since it doesn't understand the combined `JointState` message the real
hardware's head-control module does.

### `brone_gaze` — the earlier experiments
- **`linear_model_node.py`** — same cues, but combined with a hand-tuned weighted sum
  instead of fuzzy logic. Useful as a simpler baseline to compare against.
- **`fuzzy_model_node.py`** — fuzzy scoring like `algo_gaze`, but only publishes an
  annotated debug image; it does **not** send head-servo commands, so running it
  alone won't move the robot.

---

## 3. Running the project

Three modes, selected by one launch argument (`mode:=real|sim|hybrid`), plus an
audio-perception node that runs alongside all of them.

```bash
source /opt/ros/humble/setup.bash
source ~/Documents/algo_gaze_project/install/setup.bash

ros2 launch algo_gaze algo_gaze_launch.py mode:=real     # physical OP3 + usb_cam
ros2 launch algo_gaze algo_gaze_launch.py mode:=sim       # Webots + Webots' own rendered camera
ros2 launch algo_gaze algo_gaze_launch.py mode:=hybrid    # Webots physics + your REAL laptop/USB camera
```

| Mode | Camera source | Head commands go to | Notes |
|---|---|---|---|
| `real` | `usb_cam` on `video_device` (default `/dev/video0`) | Real servos via `JointState` | Requires `op3_manager` running separately with the robot's serial connection configured — not started by this launch file. |
| `sim` | Webots' rendered camera (`/robotis_op3/camera/image_raw`) | Webots via `Float64` topics | **CGI domain-gap caveat**: YOLO was trained on real photos and may not confidently detect Webots' flat-shaded CGI pedestrians — good for testing the control loop, weaker for testing raw detection accuracy. |
| `hybrid` | Real `usb_cam` feed (`/image_raw`, unremapped) | Webots via `Float64` topics | Best of both — real-world detection accuracy, simulated robot body, no physical OP3 needed. Recommended for most development/testing. |

Extra launch args: `video_device:=/dev/videoN` (real/hybrid), `use_audio:=false`
(disable the audio node entirely).

### Viewing it live
```bash
ros2 run rqt_image_view rqt_image_view    # select /gaze_model/annotated_image
```

### Recording an experiment run
```bash
ros2 topic pub /experiment/trigger std_msgs/msg/Bool "{data: true}"   # start
# ...
ros2 topic pub /experiment/trigger std_msgs/msg/Bool "{data: false}"  # stop, saves CSV to cwd
```

---

## 4. Known porting/build fixes already applied

This workspace was originally authored against **ROS 2 Jazzy**; running it on
**ROS 2 Humble** required a few source-level fixes (already applied in this repo):

1. **`cv_bridge/cv_bridge.hpp` → `cv_bridge/cv_bridge.h`** in `face_detection` and
   `op3_ball_detector` — Humble's `cv_bridge` doesn't ship the `.hpp`-suffixed header
   that later distros introduced.
2. **`op3_localization`**: `pelvis_pose_.header.stamp + transform_tolerance` needed an
   explicit `rclcpp::Time(...)` wrap — Humble doesn't implicitly convert a
   `builtin_interfaces::msg::Time` to `rclcpp::Time` in that arithmetic context.
3. **Hardcoded YOLO model path** in `algo_gaze/main.py` (was hardcoded to the original
   author's home directory) — now resolves via the `yolo_model_path` ROS parameter,
   falling back to the package's bundled `models/` directory.
4. **`tilt_dir` sign flip for simulation** — the real OP3's `head_tilt` joint needs the
   opposite sign convention from Webots' simulated one. `tilt_dir` now auto-flips based
   on `simulation_mode` (see `main.py`).

If you rebuild this on yet another ROS distro, these four spots are the most likely
places to hit new API drift.

---

## 5. Known limitations

- **CGI domain gap** (`sim` mode): YOLO may under-detect Webots' rendered humans. Use
  `hybrid` mode if you need reliable detection while still using the simulated robot body.
- **Pixel dead-zone**: small movements near frame-center intentionally don't move the
  head (prevents jitter) — expect no response to tiny motions there.
- **`speech_status` is currently visual-only** (lip-movement variance), not real audio —
  this is exactly what the audio-fusion work in progress addresses.
- **Webots world edits don't persist automatically** — `robot_launch.py` copies the world
  file to a temp path on every launch (visible in Webots' title bar as something like
  `/tmp/tmpXXXXX_end_world_with_URDF_robot.wbt`). To keep scene changes (e.g. added
  pedestrians), use Webots' **File → Save World As...** and overwrite the actual source
  file at `ROBOTIS-OP3-Simulations/op3_webots_ros2/worlds/robotis_op3_extern.wbt`, then
  rebuild `op3_webots_ros2`.

---

## 6. Roadmap: audio/vision fusion

**Goal:** replace/augment the visual lip-movement `speech_status` proxy with real
audio — Voice Activity Detection (VAD) + Sound Source Localization (SSL/DOA) — fused
with the existing vision cues, so the robot can (a) confirm speech more reliably than
lip-motion alone, and (b) react to speakers **outside the camera's field of view**,
which the vision-only baseline cannot do at all.

- **Phase 1 (in progress, no hardware needed)** — `audio_gaze_msgs/AudioCue` message
  (`is_speech`, `direction_deg`, `confidence`) + `audio_gaze/mock_audio_node` (publishes
  controllable fake audio events via `/audio/mock_trigger`) + fusion logic in
  `algo_gaze/main.py` that matches audio direction to each tracked person's visual angle
  (`camera_hfov_deg`, `audio_match_tolerance_deg` params), with CSV logging extended to
  record audio state per frame for later ablation analysis.
- **Phase 2** — swap the mock node for a real mic-array driver (e.g. ReSpeaker Mic Array
  v2.0 + `respeaker_ros`), publishing to the same `/audio/cue` topic/message — no changes
  needed elsewhere.
- **Phase 3** — evaluation: compare vision-only vs. vision+audio fusion using the existing
  CSV recording pipeline (attention-switch latency, correct-speaker-identification
  accuracy, off-camera-speech reaction).
