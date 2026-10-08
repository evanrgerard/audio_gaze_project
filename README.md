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

## 4b. Phase 2: real GCC-PHAT + SRP-PHAT beamforming (still hardware-free)

Replaces the manual-trigger mock with an actual signal-processing pipeline,
runnable entirely in simulation — no ReSpeaker/mic-array hardware, and no
Webots acoustic model needed (Webots doesn't simulate sound propagation in
this project).

### New: `audio_gaze_msgs`
- **`MicArrayFrame.msg`** — a block of synchronized multi-channel audio
  (`sample_rate`, `num_channels`, `frame_size`, interleaved `float32[] data`).
- **`BeamformedAudio.msg`** — the delay-and-sum-enhanced mono signal + the
  steering direction used, for future VAD/ASR use (not consumed by the
  fusion logic today).

### New: `audio_gaze/dsp/` (pure numpy, no rclpy — independently testable)
- **`gcc_phat.py`** — `gcc_phat(sig, refsig, fs, max_tau)`: classic
  Generalized Cross-Correlation with Phase Transform, the standard TDOA
  estimator. `fractional_delay(sig, delay_samples)`: FFT phase-shift
  fractional-sample delay, used both by the beamformer (to time-align
  channels) and the simulator (to place a synthetic source at an exact
  geometric delay).
- **`array_geometry.py`** — mic array position/angle-convention helpers
  (shared between estimation and simulation, so both sides of the math agree).
- **`beamformer.py`** — `srp_phat_doa(...)`: SRP-PHAT (Steered Response
  Power - PHAT), which scans a grid of candidate angles and scores each by
  how well every mic pair's GCC-PHAT correlation agrees with that angle's
  geometrically-predicted delay — beamforming used as a direction finder,
  far more robust than solving TDOAs algebraically. `delay_and_sum(...)`:
  steers a real beam at the winning angle to produce an enhanced/denoised
  mono signal.
- **`simulate.py`** — `simulate_plane_wave_frame(...)`: synthesizes a
  correctly-delayed multi-channel far-field plane wave for a given direction
  + sensor noise. Shared by the offline unit tests (`audio_gaze/test/test_gcc_phat_dsp.py`,
  ground-truth angle recovery, confidence sanity checks, beamforming SNR
  gain) and by `sim_mic_array_node` below.

### New nodes (`audio_gaze`)
- **`sim_mic_array_node`** — hardware-free mic-array simulator. Same
  `/audio/mock_trigger` UX as `mock_audio_node` (publish a direction in
  degrees to simulate a speaker), but instead of forging the answer it
  synthesizes an actual multi-channel signal with the correct per-mic
  geometric delays + sensor noise, publishing `MicArrayFrame` on
  `/audio/mic_array_raw`.
- **`gcc_phat_node`** — the real SSL node. Adaptive-noise-floor VAD, then
  SRP-PHAT DOA + delay-and-sum beamforming per frame. Publishes the same
  `AudioCue` on `/audio/cue` that `mock_audio_node` does, so **`algo_gaze`'s
  fusion logic is completely unchanged** — this was the explicit design
  intent from Phase 1.

### Launch: `audio_mode` argument
```bash
ros2 launch algo_gaze algo_gaze_launch.py mode:=sim audio_mode:=gcc_phat_sim
# audio_mode:=mock (default) still works exactly as in Phase 1

# Trigger a simulated speaker exactly like the mock node:
ros2 topic pub --once /audio/mock_trigger std_msgs/msg/Float32 "{data: -25.0}"
ros2 topic echo /audio/cue          # now backed by real GCC-PHAT/SRP-PHAT
ros2 topic echo /audio/beamformed   # the enhanced mono signal (optional)
```
Or standalone, without the vision pipeline: `ros2 launch audio_gaze audio_gcc_phat_sim.launch.py`.

**Verified**: end-to-end run (sim_mic_array_node -> gcc_phat_node -> `/audio/cue`)
triggered at -40 deg reported `direction_deg: -40.0` for the entire 3s burst
at ~0.71-0.72 confidence; silence/no-signal frames correctly stayed near-zero
confidence. Offline DSP unit tests cover angle recovery at 8 angles around
the full circle, low confidence on pure noise, and SNR improvement from
delay-and-sum over any single raw channel.

---

## 4c. Detection & tracking robustness (simulation-specific fixes)

Getting real signal through the pipeline in Webots surfaced three distinct bugs/gaps, all fixed:

- **CGI domain gap tuning.** Webots' rendered people don't look like the real
  photos YOLO/MediaPipe were trained on, so the stock 0.5 confidence threshold
  under-detects them. New tunable parameters on `algo_gaze`:
  `yolo_conf_threshold` and `mediapipe_min_detection_confidence` (both default
  0.5, unchanged for `real`/`hybrid`). `mode:=sim` overrides both to **0.3** in
  `algo_gaze_launch.py`. **This needs to go back up for real deployment** — a
  low threshold tuned to compensate for unrealistic CGI will produce more
  false positives on a real, cluttered scene (see section 5).
- **`WARNING not enough matching points` spam + wasted CPU.** Ultralytics'
  default tracker (BoT-SORT) runs sparse-optical-flow camera-motion
  compensation every frame; Webots' flat-shaded, low-texture scenes don't have
  enough corner features for it to ever succeed. Fixed with a custom tracker
  config, `algo_gaze/config/botsort_no_gmc.yaml` (`gmc_method: none`), wired
  into the `.track()` call via `self.tracker_config_path`.
- **Head got stuck looking at nothing.** There was no logic at all for "nobody
  detected" — the head just stayed wherever it last was. Added
  `recenter_head_smooth()`: after `lost_target_recenter_delay_sec` (default
  1.5s) of continuous non-detection, the head slowly drifts back to center
  instead of staying locked on a target that's left frame.

---

## 4d. Multi-person Webots test world

`robotis_op3_extern.wbt` now has **three** Webots `Pedestrian` PROTOs (not
zero, as originally) for testing multi-person priority scoring and DOA
matching together:

| Pedestrian | Position (x, y) | Bearing (`direction_deg`) |
|---|---|---|
| `pedestrian(0)` | (2.5, 0) | **0.0°** (center) |
| `pedestrian(1)` | (2.5, 0.9) | **-19.8°** (left) |
| `pedestrian(2)` | (2.5, -0.9) | **+19.8°** (right) |

Each has a `rotation` set to face the robot exactly (Webots pedestrians face
`+X` by default — with all three placed along the robot's own `+X`, they'd
otherwise show their backs to the camera). Distinct `shirtColor`/`pantsColor`
per pedestrian for visual identification. Fixed a version-mismatch bug along
the way: the `Pedestrian` `EXTERNPROTO` must match the installed Webots
version (`R2025a`) — pinning it to `R2023a` (like the project's other,
version-stable `EXTERNPROTO`s) caused `Skipped unknown field 'solid'` errors
and broken rendering on the pedestrian sub-parts specifically.

Trigger a cue matching one of them:
```bash
ros2 topic pub --once /audio/mock_trigger std_msgs/msg/Float32 "{data: -19.8}"  # pedestrian(1)
```

---

## 4e. Live web dashboard (`gaze_dashboard`)

A single-page live view of the whole system — annotated video feed (MJPEG),
config, FPS/latency, commanded + actual pan/tilt, current audio cue + recent
history, per-person fuzzy scores, and a filtered live log — replacing the need
to juggle several `rqt` windows.

**Recovery note:** this package existed only as orphaned compiled `.pyc`
bytecode with no source files, no `package.xml`, never committed to git.
Rebuilt from that bytecode: the HTML/CSS/JS (`dashboard_page.py`) is recovered
**byte-for-byte** (it was a pure string constant); `dashboard_node.py` was
rewritten to match the recovered docstring/parameters/imports, backed by a
Flask app. `algo_gaze/main.py` gained one new piece the dashboard needed that
didn't exist before: a `/gaze_model/telemetry` JSON publisher (FPS, latency,
commanded pan/tilt, target ID, per-person fuzzy scores/cues).

```bash
# Terminal 1:
ros2 launch algo_gaze algo_gaze_launch.py mode:=sim audio_mode:=gcc_phat_sim
# Terminal 2, with matching mode/audio_backend so the Config panel is accurate:
ros2 launch gaze_dashboard dashboard.launch.py mode:=sim audio_backend:=gcc_phat_sim
```
Open **http://localhost:8080**. The top-down "ground truth map" panel now
works — see 4k below.

---

## 4f. Real audio hardware path

Two real (non-simulated) audio nodes, both drop-in `AudioCue` publishers —
`algo_gaze`'s fusion logic needs no changes for either:

- **`real_mono_mic_node`** — opens a real single-channel mic (`sounddevice`),
  real energy-based VAD (adaptive noise floor, same SNR-threshold logic as
  `gcc_phat_node`). **`direction_deg` is always `0.0`** — this is not a bug,
  a single channel cannot physically measure direction of arrival. Practical
  effect: only a person near screen-center will ever register as "matched."
- **`real_mic_array_node`** — opens a real **N-channel** device and publishes
  real `MicArrayFrame`, feeding the *existing, unmodified* `gcc_phat_node` —
  same DSP code as simulation, only the signal source changes. Verified live
  against a real 2-channel device (a laptop's built-in stereo mic array):
  driver → `gcc_phat_node` → `/audio/cue` confirmed working end-to-end on
  genuinely captured audio.
  ```bash
  python3 -c "import sounddevice as sd; print(sd.query_devices())"   # find your device index
  ros2 launch audio_gaze audio_gcc_phat_real.launch.py input_device:=<idx> num_channels:=<n>
  ```
  **`mic_array_radius_m`/`mic_start_angle_deg`/`mic_array_yaw_offset_deg`
  default to simulation placeholder values — measure your real array's
  geometry and override them, or every direction estimate is calibrated
  against a fictional array.**

  Or skip the standalone launch above entirely and get vision + head control
  + real mic-array DOA from one process, via `algo_gaze_launch.py`'s
  `audio_mode` argument:
  ```bash
  ros2 launch algo_gaze algo_gaze_launch.py mode:=hybrid audio_mode:=real_mic_array
  ```
  This runs the exact same `real_mic_array_node` → `gcc_phat_node` pair,
  parameterized from `audio_gaze/config/real_audio_params.yaml` instead of
  the launch arguments above — edit that file for device/geometry settings
  (`input_device`, `num_channels`, `mic_array_radius_m`, etc.) rather than
  passing them on the command line. `input_device:=<idx>` still has to be
  set correctly in the YAML first; there is no launch-argument override for
  it on this path. Use whichever `mode` (`real`/`sim`/`hybrid`) matches what
  you want vision to do — the audio path is independent of it either way.

Two real bugs found and fixed while wiring this up (both apply to simulation too):
- **Confidence was capped by array size, not signal quality.** Reducing
  simulated sensor noise changed nothing; SRP-PHAT confidence is fundamentally
  limited by angular resolution. Raised the default `mic_array_radius_m` from
  0.0463m to **0.07m** (must match between `sim_mic_array_node` and
  `gcc_phat_node`) — typical confidence went from ~0.55–0.72 to **~0.85–1.0**
  at the same test angles.
- **Startup false-positive VAD.** `gcc_phat_node`'s adaptive noise floor
  seeded at an arbitrary low value, so ordinary background noise looked like
  a huge SNR spike for the first second or two after launch — publishing
  `is_speech: true` with a near-zero, meaningless confidence. Fixed with a new
  `noise_floor_warmup_sec` parameter (default 1.5s): frames within that
  window after launch never trigger speech, they only let the floor converge
  to the real ambient level first.

---

## 4g. Sim-to-real risk analysis: self-occlusion / mount shadowing

Before trusting any of this on real hardware, two stress-test additions to
`audio_gaze/dsp/simulate.py` quantify a risk the free-field simulation
otherwise ignores entirely: the robot's own head/body partially blocking some
mics from the source once the array is actually mounted. Both are opt-in
(default off, zero effect on existing behavior/tests — 11/11 DSP unit tests
still pass unchanged).

- **`head_shadow_max_db`** — amplitude-only attenuation model (mic facing
  away from the source loses signal energy). Result: **barely matters.**
  GCC-PHAT is phase-based (that's what PHAT whitening buys you), so amplitude
  loss alone, even a severe 20dB, left confidence/accuracy essentially
  unchanged in testing.
- **`head_diffraction_radius_m`** — models the effect that actually matters:
  sound diffracting *around* a rigid sphere (approximating the mount) takes a
  longer path and arrives measurably **later** than straight-line geometry
  predicts, using the classic Woodworth (1938) spherical-head geometric
  approximation (the standard interaural-time-difference model, generalized
  here to an N-mic array). Because this directly corrupts the timing/phase
  values GCC-PHAT depends on, the effect is large: mean angle error 7–39°,
  **worst-case up to 168°**, at physically plausible mount radii
  (0.03–0.10m). Critically, confidence does **not** reliably flag this —
  one tested case stayed at 0.80 confidence (well above the `min_doa_confidence=0.3`
  distrust gate) while still averaging 7° of error and up to 87° worst-case.

**Practical implication for real hardware:** mounting placement (minimizing
how much of the robot's own mass sits in the array's near field) matters more
than any DSP fix here. Plan an actual calibration pass — play a source at
known angles around the real mounted array and compare — rather than trusting
the confidence gate alone once real.

---

## 4h. Tuning every parameter from one place

All the values covered above (detection thresholds, timing, VAD threshold,
mic geometry, ...) are ROS 2 parameters, previously only changeable by
editing Python or passing `--ros-args -p` flags on every launch. Moved them
into plain YAML files, loaded automatically by the launch files — **edit the
file, relaunch, done.** No rebuild needed (they're data files, not compiled).

| File | Node(s) | Covers |
|---|---|---|
| `algo_gaze/algo_gaze/config/gaze_params.yaml` | `algo_gaze_node` | Detection thresholds, `audio_match_tolerance_deg`, `camera_hfov_deg`, `lost_target_recenter_delay_sec` |
| `audio_gaze/config/sim_audio_params.yaml` | `sim_mic_array_node`, `gcc_phat_node` | Everything for the hardware-free simulated audio pipeline |
| `audio_gaze/config/real_audio_params.yaml` | `real_mic_array_node`, `gcc_phat_node` | Everything for the real mic array — including the placeholder geometry from 4f that you MUST measure and override |

Verified live: edited `mic_array_radius_m` in `sim_audio_params.yaml`, relaunched,
`ros2 param get /sim_mic_array_node mic_array_radius_m` reflected the new
value immediately — no code touched.

`mode:=sim` in `algo_gaze_launch.py` still layers its two CGI-domain-gap
threshold overrides (`yolo_conf_threshold`/`mediapipe_min_detection_confidence`
→ 0.3) on top of `gaze_params.yaml` in code, since those are tied to which
mode you picked, not something to hand-tune per experiment — edit that
override in the launch file itself if you want to change the *simulation*
values specifically; edit the YAML for everything else, and for `real`/`hybrid`.

---

## 4i. Conventions: angles, coordinates, and units

Three different angle conventions and two different coordinate frames are in
play across this project. Mixing them up is the single easiest way to get a
confidently-wrong direction number, so here's the complete map.

### `direction_deg` — the one convention almost everything uses

Every `AudioCue`, every `/audio/mock_trigger` value, `camera_hfov_deg`,
`audio_match_tolerance_deg`, and every bearing quoted anywhere in this README
(the pedestrian angles in 4d, the trigger commands in section 6) uses this
one convention:

```
        0°  (straight ahead / camera center)
         |
-90° ----+---- +90°     -180..180, wraps at ±180
 left    |    right
       -180°/180° (directly behind)
```

**Negative = left, positive = right, 0 = forward.** This matches how a
person would describe it ("she's about 20 degrees to my left") — deliberately
*not* the standard math convention (see below), because every place this
number is compared against something (a person's on-screen position, a
trigger command you type by hand) is easier to reason about this way.

### Math convention — internal to the DSP module only

`audio_gaze/dsp/` (the actual GCC-PHAT/SRP-PHAT geometry math) works in the
**opposite**, standard trigonometric convention: angles measured
**counter-clockwise from the array's local +x axis**, same as `atan2(y, x)`.
This is `theta_deg` in the code — never exposed outside `dsp/`, always
converted at the boundary:

```python
# audio_gaze/dsp/array_geometry.py
def math_angle_to_direction_deg(theta_deg, yaw_offset_deg=0.0):
    return wrap180(yaw_offset_deg - theta_deg)

def direction_deg_to_math_angle(direction_deg, yaw_offset_deg=0.0):
    return wrap180(yaw_offset_deg - direction_deg)
```

`yaw_offset_deg` (the `mic_array_yaw_offset_deg` parameter from 4f/4h) is
where you calibrate for how the array is physically mounted — if channel 0
doesn't point at the robot's forward axis, this is the correction, applied
uniformly by these two functions everywhere direction is computed.

### World/robot frame — meters, used for Webots positions

Robot and world positions (Webots translations, the pedestrian coordinates in
4d) use a standard ROS-style **X = forward, Y = left, Z = up** frame, in
meters, with the robot at the origin facing +X. To go from a world position
to a `direction_deg` bearing:

```python
import math
def direction_deg(x, y):          # (x, y) relative to the robot, meters
    return -math.degrees(math.atan2(y, x))
```
This is exactly the formula used to compute the 4d pedestrian bearings
(e.g. `pedestrian(1)` at `(2.5, 0.9)` → `-19.8°`) — reuse it directly for any
new object placed in the Webots world.

### Camera pixels → `direction_deg`

`pixel_x_to_azimuth_deg()` in `main.py` converts a detected person's on-screen
horizontal position to the same `direction_deg` convention, using a **simple
linear field-of-view approximation** (not a lens-calibrated projection —
see section 5):
```python
norm_offset = (person_center_x - frame_width/2) / (frame_width/2)  # -1..1
azimuth_deg = norm_offset * (camera_hfov_deg / 2.0)
```

### Units at a glance

| Quantity | Unit | Where |
|---|---|---|
| `direction_deg`, all bearings/tolerances (`audio_match_tolerance_deg`, `camera_hfov_deg`, `mic_start_angle_deg`, `mic_array_yaw_offset_deg`, `doa_grid_step_deg`) | **degrees** | messages, YAML params, this README |
| Head pan/tilt internally (`self.current_pan`/`self.current_tilt`) | **radians**, clamped to `[-1.4, 1.4]` (pan) / `[-1.0, 1.0]` (tilt) | `main.py` servo state + published `JointState` |
| Head pan/tilt on the dashboard | **degrees** | converted once, at the telemetry boundary: `math.degrees(self.current_pan)` |
| Mic array geometry (`mic_array_radius_m`), Webots positions, `speed_of_sound_mps` | **meters** / **m/s** | YAML params, `.wbt` world file |
| Time-based params (`*_sec`, `*_delay`) | **seconds** | YAML params |
| `confidence` (`AudioCue.confidence`) | **0.0–1.0**, but *not one consistent measurement* — see section on this in-conversation: VAD-strength for `real_mono_mic_node`, SRP-PHAT peak sharpness (a z-score, not a probability) for `gcc_phat_node`/simulation, always `1.0`/`0.0` for `mock_audio_node` | `/audio/cue` |

### Worked example, start to finish

Pedestrian at `(2.5, 0.9)` in the world → `direction_deg = -19.8°` (4d) →
`gcc_phat_node` internally converts that to `theta = direction_deg_to_math_angle(-19.8, yaw_offset_deg=0) = 19.8°`
→ SRP-PHAT scans candidate `theta` values, finds the best match, converts the
winner back with `math_angle_to_direction_deg()` → publishes `direction_deg: -19.8`
on `/audio/cue` → `algo_gaze` compares that to the pedestrian's on-screen
`pixel_x_to_azimuth_deg()` result → within `audio_match_tolerance_deg` (15°) →
matched.

---

## 4j. Audio search: turning toward off-camera speech

Until now, unmatched audio (someone talking outside the camera's ~78° FOV,
or just not visually confirmed) was logged as `[FUSION] Off-camera speech
detected` and nothing else — the robot kept doing whatever it was already
doing. **This was never a detection limitation** — GCC-PHAT/SRP-PHAT direction
estimates cover the full 360° regardless of `camera_hfov_deg`, proven by the
log line itself correctly reporting angles like -90° that the camera can't
see at all. The gap was purely reactive: nothing ever *acted* on an unmatched
direction.

New behavior in `algo_gaze/main.py`: whenever `/audio/cue` reports speech
that doesn't match anyone currently visible, the head pans toward that
`direction_deg` instead of just logging it — **regardless of whether other
people happen to be visible elsewhere in frame**; unmatched speech preempts
passive tracking/idling. Tilt is left untouched (a horizontal mic array
carries no elevation information to search with), and beyond the pan joint's
physical limit (±1.4 rad, ~±80°) the head simply pans as far as it
mechanically can — it can't reach directly behind the robot, same physical
constraint any real neck has.

```python
target_pan = -radians(direction_deg)   # same sign convention as track_face_smooth's pan_dir=-1
# smoothly approaches target_pan each frame, same style as recenter_head_smooth
```

Two new parameters in `gaze_params.yaml` (4h): `enable_audio_search` (default
`true`) and `audio_search_gain` (default `0.08` — higher turns faster/snappier,
lower is smoother/slower).

**Verified live**: fed `algo_gaze_node` a fake unmatched `AudioCue` at -90°
directly (bypassing Webots, which isn't reliably launchable in this sandbox
environment) — commanded pan climbed steadily and monotonically from 0 toward
the +1.4 rad clamp, exactly matching the derived sign/formula, with a single
`[AUDIO SEARCH] Turning toward...` log line and no oscillation once
CPU-starving stray background processes (an unrelated sandbox artifact from
earlier testing) were cleared.

---

## 4k. World ground truth for the dashboard's top-down map

`gaze_dashboard`'s map panel needed real (x, y) positions from inside Webots
itself — nothing published those. Rather than write a whole separate Webots
controller, extended the **existing** one: `op3_extern_controller` already
runs as a `webots::Supervisor` (it was already using that access for the
robot's own center-of-mass) — it just never looked at anything else in the
scene.

- **`robotis_op3_extern.wbt`** — the three pedestrians (4d) now have `DEF`
  labels (`PEDESTRIAN_0/1/2`) so the Supervisor API can find them
  (`Supervisor::getFromDef`) — separate from their `name` field, which is
  what the dashboard actually displays.
- **`op3_extern_controller.cpp`** — new `publishWorldGroundTruth()`, called
  at ~10Hz (throttled from the main ~125Hz loop — a map doesn't need camera
  framerate). Publishes `/webots/world_ground_truth`, JSON on a plain
  `std_msgs/String` (same lightweight pattern as algo_gaze's own
  `/gaze_model/telemetry` — no new `.msg` package needed): each pedestrian's
  real `(x, y)` and `name`, plus the robot's own `(x, y, yaw_deg)` from
  `getSelf()->getPosition()`/`getOrientation()`. A missing `DEF` warns and
  skips that pedestrian rather than crashing — the world file can gain or
  lose pedestrians without touching this code.
- **Deliberately carries no audio information at all.** `dashboard_node.py`
  combines these raw positions with the `/audio/cue` it already receives to
  compute, per person, a real bearing-from-robot (the exact formula from 4i)
  and whether *that* person is within `audio_match_tolerance_deg` of the
  current audio direction (`speaking: true` on the closest match, if any) —
  and combines the robot's yaw with the pan angle it already tracks to get
  `camera_heading_deg` for the FOV cone. Keeping the C++ controller "dumb"
  (positions only) means it needs no coupling to the audio stack at all.

**Verified**: the C++ builds clean (zero warnings). The Supervisor connection
itself was confirmed live — a real, connected `op3_extern_controller`
gracefully logged and skipped all three pedestrians when the world it had
loaded predated the `DEF` labels (an old cached Webots instance from earlier
testing), proving the fallback path works without crashing. The full
happy path (`DEF` found → real `getPosition()` → dashboard map render) is
implemented and the Python-side derivation is separately verified against
realistic fake data (bearings landed exactly on the expected -19.8°/0°/19.8°
from 4d, and `speaking` correctly picked only the matching pedestrian) — but
this sandbox's Webots instances only ever accept one `<extern>` connection
per launch (reconnecting after a controller dies hits "not in the list of
robots with extern controllers", not a retry-related timeout like elsewhere
in this project), so the live end-to-end render couldn't be confirmed here
despite several attempts. **Should just work on a normal launch** — flag it
if the map stays empty with pedestrians actually in the loaded world.

---

## 5. Known limitations / open items

**Not yet validated on real hardware at all:**
- **Mic array geometry is still simulation-placeholder** (`mic_array_radius_m`,
  `mic_start_angle_deg`, `mic_array_yaw_offset_deg`) — measure the real
  device's spacing/mounting and override before trusting any direction output.
- **Self-occlusion/diffraction risk is quantified but unconfirmed** (section
  4g): up to 168° worst-case error predicted from the robot's own body
  shadowing the array, and the confidence gate does not reliably catch it.
  Needs an actual calibration pass once mounted.
- **`real_mic_array_node` has only been validated against a 2-channel laptop
  mic**, not a purpose-built array (e.g. ReSpeaker) — the driver code is
  generic (any `sounddevice`-visible multi-channel device), but real-array
  behavior (onboard DSP, different sample formats) is untested.
- **Vision thresholds are tuned the wrong way for real deployment.**
  `yolo_conf_threshold`/`mediapipe_min_detection_confidence` were lowered to
  0.3 specifically to compensate for Webots' unrealistic CGI pedestrians —
  this needs to go back up (`mode:=real`/`hybrid` already default to 0.5) or
  a real camera will likely produce more false-positive detections.
- **Acoustic parameters (VAD threshold, noise-floor adaptation rate,
  confidence scaling) are tuned against synthetic white noise**, not a real
  room's reverberation/colored noise — expect to re-tune once on real
  hardware.

**Structural gaps, not yet built:**
- **No active search/scan when idle** (nobody detected AND no active audio
  cue) — the head still just passively recenters, it doesn't sweep looking
  for someone. (Reacting to *audio* when idle now works — see 4j; this
  remaining gap is specifically the no-audio-at-all idle case.)
- **`AudioCue` represents one speaker/direction at a time.** If multi-speaker
  SSL (e.g. MUSIC-based) is wanted later, this should become an `AudioCueArray`
  — not yet built, flagged as a future decision point.
- **No occlusion/re-identification stress-testing** — the 3-pedestrian test
  world (4d) places them non-overlapping by design; real people crossing
  paths will exercise BoTSORT's tracking IDs for the first time on real data.
- **No timestamp synchronization** between camera and mic array beyond a
  coarse staleness check (`audio_cue_timeout_sec`) — fine when both share one
  ROS clock in sim, worth watching once both are independent real USB devices.
- **`gaze_dashboard`'s top-down map (4k) is built but its live end-to-end
  render is unconfirmed in this sandbox** — the Supervisor connection, the
  graceful-fallback path, the C++ build, and the Python-side bearing/speaking
  math are all separately verified; only the specific "real pedestrian found
  via DEF → rendered on the map" path couldn't be exercised live here (this
  sandbox's Webots instances refuse a second `<extern>` reconnect after the
  first one dies, and every launch attempt died before fully connecting).
  Should just work on a normal launch.
- **Azimuth mapping is a simple linear FOV approximation**, not a
  lens-calibrated projection — fine for tolerance-based matching, not
  precise angle measurement.
- **Latency is untested against real-time pressure.** Observed ~4–15 FPS,
  70–250ms latency on CPU-only YOLO11s+MediaPipe in this dev environment —
  worth profiling on the actual OP3's onboard compute; consider a smaller
  model (e.g. `yolo11n`) if it's not fast enough there.

---

## 6. Quick reference: running everything

```bash
# Every new terminal:
source /opt/ros/humble/setup.bash
source ~/Documents/BRONE_audio_gaze_project/install/setup.bash
cd ~/Documents/BRONE_audio_gaze_project

# Rebuild after any source change (safe to always list all four):
colcon build --symlink-install --packages-select audio_gaze_msgs audio_gaze algo_gaze gaze_dashboard
source install/setup.bash
```

### Tune a value (section 4h) -- edit, then just relaunch, no rebuild

```bash
$EDITOR algo_gaze/algo_gaze/config/gaze_params.yaml       # detection thresholds, timing, tolerances
$EDITOR audio_gaze/config/sim_audio_params.yaml           # simulated audio pipeline
$EDITOR audio_gaze/config/real_audio_params.yaml          # real mic array -- geometry lives here
```

### Vision (`mode`) x Audio (`audio_mode`) combinations

```bash
# mode: real | sim | hybrid        audio_mode: mock | gcc_phat_sim | real_mono_mic | real_mic_array
ros2 launch algo_gaze algo_gaze_launch.py mode:=sim audio_mode:=gcc_phat_sim      # fully hardware-free, recommended default
ros2 launch algo_gaze algo_gaze_launch.py mode:=hybrid audio_mode:=real_mono_mic  # real camera + real mic (no DOA, see 4f)
ros2 launch algo_gaze algo_gaze_launch.py mode:=hybrid audio_mode:=real_mic_array # real camera + real N-ch mic array DOA (see 4f)
ros2 launch algo_gaze algo_gaze_launch.py mode:=real video_device:=/dev/video1    # physical OP3 hardware
```

### Trigger a simulated speaker (mock or gcc_phat_sim -- same command either way)

```bash
ros2 topic pub --once /audio/mock_trigger std_msgs/msg/Float32 "{data: 0.0}"    # pedestrian(0), center
ros2 topic pub --once /audio/mock_trigger std_msgs/msg/Float32 "{data: -19.8}"  # pedestrian(1), left
ros2 topic pub --once /audio/mock_trigger std_msgs/msg/Float32 "{data: 19.8}"   # pedestrian(2), right
```

### Real multi-channel mic array (section 4f) -- standalone or plugged into the audio_mode above

```bash
python3 -c "import sounddevice as sd; print(sd.query_devices())"   # find your device index/channel count

# Standalone (device/geometry passed as launch args):
ros2 launch audio_gaze audio_gcc_phat_real.launch.py input_device:=<idx> num_channels:=<n>

# Or as part of the unified launch above (device/geometry come from
# audio_gaze/config/real_audio_params.yaml instead -- edit that file first):
ros2 launch algo_gaze algo_gaze_launch.py mode:=hybrid audio_mode:=real_mic_array
```

### Watch it

```bash
# Live web dashboard (recommended) -- match mode/audio_backend to what you launched above:
ros2 launch gaze_dashboard dashboard.launch.py mode:=sim audio_backend:=gcc_phat_sim
# then open http://localhost:8080

# ...or the individual rqt tools:
ros2 topic echo /audio/cue
ros2 run rqt_image_view rqt_image_view      # select /gaze_model/annotated_image
rqt_graph
ros2 run rqt_plot rqt_plot /audio/cue/direction_deg /audio/cue/confidence
```

### Diagnostics

```bash
# Raw per-candidate YOLO confidence on the current live frame, bypassing algo_gaze's
# threshold entirely -- settles "is this a threshold problem or a real detection failure":
python3 diagnose_yolo.py

# Offline DSP correctness (GCC-PHAT/SRP-PHAT/beamforming), no ROS/hardware needed:
cd audio_gaze && PYTEST_DISABLE_PLUGIN_AUTOLOAD=1 python3 -m pytest test/test_gcc_phat_dsp.py -v
```

**⚠️ Workspace path is fixed:** if the project folder is ever renamed or moved
again, `build/`, `install/`, and `log/` must be deleted and fully rebuilt —
colcon bakes absolute paths in, it doesn't use relative ones.