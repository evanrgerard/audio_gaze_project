# algo_gaze / BRONE Audio-Visual Attention Project — Checkpoint

**Date:** August 3, 2026
**Workspace:** `~/Documents/BRONE_audio_gaze_project`
**Environment:** ROS 2 Humble, Webots R2025a (`ros-humble-webots-ros2` 2025.0.0)

---

## 1. Baseline system (starting point)

A ROS 2 workspace built around the ROBOTIS OP3 humanoid robot stack, with two
custom vision-based attention packages layered on top:

- **`algo_gaze`** — main/production node. YOLOv11 (person detection) + MediaPipe
  Holistic (face/hand/pose landmarks) + a `skfuzzy` fuzzy-logic controller that
  scores every visible person on: proximity, angle-from-center, pointing gesture,
  waving gesture, body orientation, direct gaze, and speech status (originally a
  **visual proxy** — lip-landmark distance variance). Highest-scoring person
  becomes the tracking target; the robot's head pan/tilt servos smoothly track
  them, with a "nod" gesture triggered after 2s of stable lock-on.
- **`brone_gaze`** — earlier experimental variants (linear-model and fuzzy-model
  scoring), not wired to head control, kept for reference/comparison.

---

## 2. Infrastructure fixes (getting it running at all)

- Fixed a hardcoded model path in `main.py` (was pointing to the original
  author's home directory) — now resolves via ROS parameter + package share dir.
- Fixed stale `build/`/`install/` CMake caches after the workspace was moved
  between machines/paths (`CMakeCache.txt` path mismatches) — resolved by wiping
  and doing clean rebuilds.
- Fixed a Jazzy→Humble C++ API drift: `cv_bridge/cv_bridge.hpp` doesn't exist on
  Humble (only `.h`) — patched in `face_detection` and `op3_ball_detector`.
- Fixed a Humble-specific `rclcpp::Time`/`Duration` addition operator error in
  `op3_localization` (needed explicit `rclcpp::Time(...)` wrapping).

---

## 3. Three run modes (one unified launch file)

`algo_gaze/launch/algo_gaze_launch.py`, selected via `mode:=`:

| Mode | What runs | Use case |
|---|---|---|
| `real` | `usb_cam` + `algo_gaze`, `simulation_mode:=false` | Physical OP3 hardware |
| `sim` | Webots (`op3_webots_ros2`) + `algo_gaze`, image/joint-state topics remapped to Webots' naming, `simulation_mode:=true` | No hardware needed, but YOLO sees Webots' CGI-rendered scene (domain-gap risk for vision accuracy) |
| `hybrid` | Webots (physics/head movement only) + **real laptop webcam** for vision + `algo_gaze` | Best of both — real-world detection accuracy, simulated robot body, no hardware required |

```bash
ros2 launch algo_gaze algo_gaze_launch.py mode:=sim      # or real / hybrid
ros2 launch algo_gaze algo_gaze_launch.py mode:=real video_device:=/dev/video1
```

Key fixes made for `sim`/`hybrid` to actually work correctly:
- `op3_webots_ros2`'s extern controller uses **different topic names/types** than
  real hardware (`/robotis_op3/camera/image_raw` vs `/image_raw`; per-joint
  `std_msgs/Float64` head commands vs one combined `JointState`) — handled via
  launch-time remaps for the input side, and a `publish_head_command()` helper
  in `main.py` that emits both formats when `simulation_mode:=true`.
- **Tilt direction was inverted in simulation**: the robot panned to the correct
  person but tilted the wrong way (looked down instead of up). Root cause: a
  `tilt_dir` sign multiplier tuned for the real robot's joint convention, wrong
  for Webots' joint convention. Fixed by auto-flipping it based on
  `simulation_mode`.
- Confirmed via Webots console (`extern controller: connected`) and
  `ros2 topic hz` that the full camera → detection → fuzzy decision → head
  movement loop is live end-to-end in both `sim` and `hybrid` modes.
- Added pedestrian(s) to the Webots world (`RobotisOp3.proto`'s scene) via
  `File → Save World As...` overwriting the actual source `.wbt` (not the
  temp file Webots regenerates on each launch) so they persist across runs.

---

## 4. Phase 1 of the new research direction: vision-audio fusion

**Thesis title:** *"Pengambilan Keputusan Atensi Dinamis Terhadap Kerumunan
Manusia pada Robot Sosial Melalui Fusi Multimodal Visi dan Audio"* — dynamic
crowd-attention decision-making via multimodal vision + audio fusion.

**Key framing:** the existing vision-only system (with its lip-variance speech
*proxy*) is the thesis's own baseline for an ablation study. The contribution is
replacing/augmenting that proxy with real audio: Voice Activity Detection (VAD)
+ Sound Source Localization (SSL), eventually from a multi-mic array (ReSpeaker
Mic Array v2.0 planned, not yet acquired).

**Phase 1 goal:** build and validate the full ROS interface + fusion logic
*before* the hardware arrives, using a controllable mock audio source.

### New packages built
- **`audio_gaze_msgs`** — custom interface package. `AudioCue.msg`:
  `header`, `bool is_speech`, `float32 direction_deg` (0=center, −=left, +=right),
  `float32 confidence`.
- **`audio_gaze`** — `mock_audio_node.py`. Publishes `AudioCue` on `/audio/cue`
  at 10 Hz. Listens on `/audio/mock_trigger` (`std_msgs/Float32`) — publishing a
  direction simulates a 3-second speech burst from that angle, then
  auto-returns to silence (mimics a real, finite utterance).

### Fusion logic added to `algo_gaze/main.py`
- Subscribes to `/audio/cue`; tracks staleness (`audio_cue_timeout_sec`, default
  0.5s) so old cues don't linger.
- `pixel_x_to_azimuth_deg()` converts each tracked person's screen position to
  an approximate azimuth using `camera_hfov_deg` (default 78°, tunable param).
- Per person, per frame: if the audio direction is within
  `audio_match_tolerance_deg` (default 15°) of that person's azimuth, their
  `speech_status` fuzzy input is confirmed via audio (`audio_speech` cue),
  fused with the original visual lip-variance proxy (`visual_speech` cue) —
  either one alone is enough to flag "may be speaking." Both signals are kept
  separately in `p['cues']` for later ablation analysis.
- **Off-camera speech detection** (the key new capability vision-only cannot
  have at all): if audio detects speech that matches no currently-tracked
  person — including when *nobody* is visually detected — it's logged via
  `[FUSION]`-tagged messages, for future reactive behavior (e.g. pan-search)
  and for the thesis's ablation writeup.
- CSV experiment logging (`/experiment/trigger`) extended with
  `Audio_Active`, `Audio_Direction_Deg`, `Audio_Matched_Person` columns.

### Verified working
- `ros2 topic echo /audio/cue` confirms mock triggers correctly flip
  `is_speech` true/false with the right direction and timing.
- `[FUSION]` log lines confirmed printing for both "matched to a visible
  person" and "no match" cases once tested with a trigger angle roughly
  matching the pedestrian's actual on-screen position.

---

## 5. Known limitations / open items

- **CGI domain gap**: YOLO trained on real photos may under-detect Webots'
  rendered pedestrians at the default `conf=0.5` threshold — `hybrid` mode
  (real webcam + simulated robot body) is the practical workaround.
- **Azimuth mapping is a simple linear FOV approximation**, not a
  lens-calibrated projection — fine for tolerance-based matching, not
  precise angle measurement.
- **`AudioCue` represents one speaker/direction at a time.** If multi-speaker
  SSL (e.g. MUSIC-based) is wanted later, this should become an `AudioCueArray`
  — not yet built, flagged as a future decision point.
- **No reactive behavior yet** for off-camera speech (e.g. auto pan-search
  toward an unmatched voice) — currently logged only, not acted on.
- **SSL method not yet chosen/implemented for real hardware** — mock node
  only. Because everything routes through the same `AudioCue` message and
  `/audio/cue` topic, swapping in a real method (ReSpeaker onboard DOA,
  GCC-PHAT, SRP-PHAT, MUSIC, etc.) later requires only a new node + one
  launch-file line change, no changes to `algo_gaze`'s fusion logic.

---

## 6. Quick reference: running everything

```bash
# Every new terminal:
source /opt/ros/humble/setup.bash
source ~/Documents/BRONE_audio_gaze_project/install/setup.bash

# Launch (mock audio node comes up automatically, disable with use_audio:=false):
cd ~/Documents/BRONE_audio_gaze_project
ros2 launch algo_gaze algo_gaze_launch.py mode:=sim      # or real / hybrid

# Simulate a speech event from a given angle (deg, 0=center, - =left, + =right):
ros2 topic pub --once /audio/mock_trigger std_msgs/msg/Float32 "{data: -20.0}"

# Watch it directly:
ros2 topic echo /audio/cue
ros2 run rqt_image_view rqt_image_view      # select /gaze_model/annotated_image

# Rebuild after any source change:
colcon build --symlink-install --packages-select <changed_package>
source install/setup.bash
```

**⚠️ Workspace path is fixed:** if the project folder is ever renamed or moved
again, `build/`, `install/`, and `log/` must be deleted and fully rebuilt —
colcon bakes absolute paths in, it doesn't use relative ones.