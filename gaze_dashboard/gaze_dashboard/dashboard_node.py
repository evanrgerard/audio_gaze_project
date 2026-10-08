#!/usr/bin/env python3
"""
Local web dashboard for observing the whole algo_gaze / audio_gaze system live:
world/launch configuration, audio cue history, per-person fuzzy scores, head
pan/tilt, and recent ROS log lines from the relevant nodes -- all in one page,
no juggling multiple rqt windows.

Run (after launching your normal algo_gaze_launch.py in another terminal):
    ros2 run gaze_dashboard dashboard_node

Then open in a browser:
    http://localhost:8080

Parameters:
    http_port (int, default 8080)
    mode (string, default '') -- purely informational, shown in the Config
        panel. Pass the same value you used for algo_gaze_launch.py's `mode`
        so the dashboard can display it (ROS has no built-in way to ask
        another node "what launch args were you started with").
    audio_backend (string, default '')
"""
import json
import math
import os
import signal
import subprocess
import threading
import time
from collections import deque
from datetime import datetime

import cv2
import rclpy
from cv_bridge import CvBridge
from flask import Flask, Response, jsonify, render_template_string
from rcl_interfaces.msg import Log
from rclpy.node import Node
from sensor_msgs.msg import Image, JointState
from std_msgs.msg import Bool, String

from audio_gaze_msgs.msg import AudioCue

from gaze_dashboard.bag_metrics import summarize_bag
from gaze_dashboard.dashboard_page import DASHBOARD_HTML

# Topics recorded for every experiment bag -- matches what the project's
# analyze_bag.py/analyze_exp*.py scripts already read, so bags made via the
# dashboard's Record button are a drop-in replacement for the ones those
# scripts were originally pointed at by hand.
RECORD_TOPICS = ['/experiment/gaze_metrics', '/robotis/present_joint_states', '/audio/cue']

# Only these nodes' /rosout lines show up in the Live Log panel -- otherwise
# it's drowned out by every other node in the workspace (rviz, webots
# internals, etc.) that has nothing to do with the gaze/audio pipeline.
RELEVANT_NODE_NAMES = {
    'algo_gaze_node', 'mock_audio_node', 'sim_mic_array_node', 'gcc_phat_node',
    'real_mono_mic_node', 'op3_extern_controller', 'gaze_dashboard_node',
}

LOG_LEVEL_NAMES = {
    Log.DEBUG: 'DEBUG', Log.INFO: 'INFO', Log.WARN: 'WARN',
    Log.ERROR: 'ERROR', Log.FATAL: 'FATAL',
}

MAX_LOG_LINES = 200
MAX_AUDIO_HISTORY = 50

# Same convention/value as algo_gaze's audio_match_tolerance_deg default
# (README.md 4i) -- used here to mark which pedestrian (if any) the current
# /audio/cue direction actually points at, for the map's "speaking" dot.
AUDIO_MATCH_TOLERANCE_DEG = 15.0


class DashboardNode(Node):

    def __init__(self):
        super().__init__('dashboard_node')

        self.declare_parameter('http_port', 8080)
        self.declare_parameter('mode', '')
        self.declare_parameter('audio_backend', '')
        # Where `ros2 bag record` (triggered from the dashboard's Record
        # button) writes experiment bags, and where the Experiments panel
        # looks for past ones. One flat directory of bag subdirectories --
        # matches how analyze_bag.py's hardcoded 'eksperimen_1' etc. names
        # were already being used, just no longer hand-typed per run.
        self.declare_parameter('bags_dir', os.path.expanduser('~/brone_experiments'))

        self.http_port = int(self.get_parameter('http_port').value)
        self.mode = self.get_parameter('mode').value
        self.audio_backend = self.get_parameter('audio_backend').value
        self.bags_dir = self.get_parameter('bags_dir').value
        os.makedirs(self.bags_dir, exist_ok=True)

        self.bridge = CvBridge()
        self._lock = threading.Lock()
        self._latest_jpeg = None
        self._gaze_state = {}
        self._pan_tilt_actual = {}
        self._audio_current = None
        self._audio_history = deque(maxlen=MAX_AUDIO_HISTORY)
        self._last_audio_is_speech = False
        self._logs = deque(maxlen=MAX_LOG_LINES)
        self._world_raw = None  # raw {robot:{x,y,yaw_deg}, people:[{x,y,name}]} from Webots

        # --- Recording (rosbag2) state -- guarded by self._lock like everything else ---
        self._recording_proc = None       # subprocess.Popen while a `ros2 bag record` is running
        self._recording_bag_path = None
        self._recording_start_time = None

        self.create_subscription(Image, '/gaze_model/annotated_image', self._on_image, 10)
        self.create_subscription(String, '/gaze_model/telemetry', self._on_telemetry, 10)
        # Real-robot equivalent of /gaze_model/telemetry: brone_gaze_node.py
        # (the NUC-side gaze node, a separately-evolved fork of algo_gaze's
        # main.py) publishes a differently-shaped JSON here instead, only
        # while an experiment trigger is active. Normalized to the same
        # internal `_gaze_state` shape so the UI doesn't need to know which
        # pipeline it's looking at.
        self.create_subscription(
            String, '/experiment/gaze_metrics', self._on_real_gaze_metrics, 10)
        self.create_subscription(
            JointState, '/robotis/present_joint_states', self._on_joint_states, 10)
        self.create_subscription(AudioCue, '/audio/cue', self._on_audio_cue, 10)
        self.create_subscription(Log, '/rosout', self._on_log, 10)
        self.create_subscription(
            String, '/webots/world_ground_truth', self._on_world_ground_truth, 10)

        # Drives brone_gaze_node.py's/algo_gaze's own is_recording flag (the
        # same /experiment/trigger both pipelines already listen on) so the
        # Record button starts/stops BOTH the gaze node's own metrics
        # publishing and the rosbag2 capture together, in sync.
        self.trigger_pub = self.create_publisher(Bool, '/experiment/trigger', 10)

        self._start_flask()
        self.get_logger().info(
            f'Dashboard running at http://localhost:{self.http_port} '
            f'(mode={self.mode or "?"}, audio_backend={self.audio_backend or "?"})'
        )

    # --- ROS callbacks -----------------------------------------------------

    def _on_image(self, msg: Image):
        frame = self.bridge.imgmsg_to_cv2(msg, 'bgr8')
        ok, buf = cv2.imencode('.jpg', frame, [cv2.IMWRITE_JPEG_QUALITY, 80])
        if ok:
            with self._lock:
                self._latest_jpeg = buf.tobytes()

    def _on_telemetry(self, msg: String):
        try:
            state = json.loads(msg.data)
        except json.JSONDecodeError:
            return
        with self._lock:
            self._gaze_state = state

    def _on_real_gaze_metrics(self, msg: String):
        """brone_gaze_node.py's /experiment/gaze_metrics, only published
        while its own is_recording is on. Different shape than algo_gaze's
        /gaze_model/telemetry (radians not degrees, `participants` not
        `people`/`cues`, no single `fps`-every-frame stream since it's
        recording-gated) -- normalized here into the same `_gaze_state`
        shape _on_telemetry produces, so dashboard_page.py's JS needs no
        branching between the two pipelines."""
        try:
            d = json.loads(msg.data)
        except json.JSONDecodeError:
            return
        people = [{
            'id': p.get('id'),
            'score': p.get('score'),
            'cues': {'speech': p.get('is_speaking')},
        } for p in d.get('participants', [])]
        cmd_pan, cmd_tilt = d.get('command_pan'), d.get('command_tilt')
        state = {
            'fps': d.get('fps'),
            'latency_ms': d.get('ai_latency_ms'),
            'is_recording': True,  # this topic only ever arrives while recording
            'pan_deg': round(math.degrees(cmd_pan), 1) if cmd_pan is not None else None,
            'tilt_deg': round(math.degrees(cmd_tilt), 1) if cmd_tilt is not None else None,
            'target_id': None,  # not included in this schema
            'people': people,
            'audio': {'matched_person': bool(d.get('audio_matched_any'))},
        }
        with self._lock:
            self._gaze_state = state

    def _on_joint_states(self, msg: JointState):
        if 'head_pan' not in msg.name or 'head_tilt' not in msg.name:
            return
        pan = math.degrees(msg.position[msg.name.index('head_pan')])
        tilt = math.degrees(msg.position[msg.name.index('head_tilt')])
        with self._lock:
            self._pan_tilt_actual = {'pan_deg': round(pan, 1), 'tilt_deg': round(tilt, 1)}

    def _on_audio_cue(self, msg: AudioCue):
        cue = {
            'is_speech': msg.is_speech,
            'direction_deg': round(msg.direction_deg, 1),
            'confidence': round(msg.confidence, 3),
        }
        with self._lock:
            self._audio_current = cue
            # Log a history entry on speech start/end transitions, matching
            # mock_audio_node's "burst" framing -- not every single message.
            if msg.is_speech != self._last_audio_is_speech:
                self._audio_history.append({
                    'stamp': self.get_clock().now().nanoseconds / 1e9,
                    'is_speech': msg.is_speech,
                    'direction_deg': cue['direction_deg'],
                    'confidence': cue['confidence'],
                })
            self._last_audio_is_speech = msg.is_speech

    def _on_world_ground_truth(self, msg: String):
        # Raw positions only (op3_extern_controller has no idea about audio --
        # see README.md 4j); "speaking" and camera_heading_deg are derived
        # dashboard-side in _build_world_positions, combining this with
        # /audio/cue and the pan angle we already track separately.
        try:
            raw = json.loads(msg.data)
        except json.JSONDecodeError:
            return
        with self._lock:
            self._world_raw = raw

    def _on_log(self, msg: Log):
        if msg.name not in RELEVANT_NODE_NAMES:
            return
        with self._lock:
            self._logs.append({
                'stamp': msg.stamp.sec + msg.stamp.nanosec / 1e9,
                'node': msg.name,
                'level': LOG_LEVEL_NAMES.get(msg.level, str(msg.level)),
                'msg': msg.msg,
            })

    def _build_world_positions(self):
        """Turn the raw {robot, people} ground truth into what the map
        actually draws: a robot-relative bearing (direction_deg convention,
        see README.md 4i) for every person, camera_heading_deg for the FOV
        cone, and which person (if any) the current /audio/cue direction
        actually points at -- computed here, not by op3_extern_controller,
        which knows nothing about audio at all."""
        raw = self._world_raw
        if raw is None or 'robot' not in raw:
            return None

        robot = raw['robot']
        pan_deg = self._pan_tilt_actual.get('pan_deg', 0.0)
        # world-frame math-convention heading the camera currently points --
        # see README.md 4i/4j for the -pan_deg direction_deg<->pan relation
        # this is built from.
        camera_heading_deg = robot['yaw_deg'] + pan_deg

        audio = self._audio_current
        audio_active = bool(audio and audio.get('is_speech'))
        audio_dir = audio['direction_deg'] if audio_active else None

        people = []
        best_idx, best_diff = None, None
        for i, p in enumerate(raw.get('people', [])):
            dx, dy = p['x'] - robot['x'], p['y'] - robot['y']
            world_bearing_deg = math.degrees(math.atan2(dy, dx))
            direction_deg = ((robot['yaw_deg'] - world_bearing_deg) + 180) % 360 - 180
            people.append({
                'x': p['x'], 'y': p['y'], 'name': p['name'],
                'bearing_deg': round(direction_deg, 1), 'speaking': False,
            })
            if audio_active:
                diff = abs(((audio_dir - direction_deg) + 180) % 360 - 180)
                if diff <= AUDIO_MATCH_TOLERANCE_DEG and (best_diff is None or diff < best_diff):
                    best_idx, best_diff = i, diff

        if best_idx is not None:
            people[best_idx]['speaking'] = True

        return {
            'robot': {'x': robot['x'], 'y': robot['y'], 'yaw_deg': robot['yaw_deg'],
                      'camera_heading_deg': round(camera_heading_deg, 1)},
            'people': people,
        }

    # --- Recording (rosbag2) -------------------------------------------------
    # Called from Flask's request thread, not the rclpy spin thread -- guard
    # every shared read/write with self._lock, same as the ROS callbacks do.

    def start_recording(self):
        with self._lock:
            if self._recording_proc is not None:
                return {'ok': False, 'error': 'already recording'}, 409
            bag_name = datetime.now().strftime('%Y%m%d_%H%M%S')
            bag_path = os.path.join(self.bags_dir, bag_name)
            log_path = bag_path + '.log'
            try:
                log_file = open(log_path, 'wb')
                proc = subprocess.Popen(
                    ['ros2', 'bag', 'record', '-s', 'mcap', '-o', bag_path] + RECORD_TOPICS,
                    stdout=log_file, stderr=subprocess.STDOUT)
            except FileNotFoundError as e:
                return {'ok': False, 'error': f'ros2 not found: {e}'}, 500
            # `ros2 bag record` with a bad/missing storage plugin (e.g. the
            # mcap plugin not installed on this machine) exits almost
            # immediately with a usage error rather than hanging -- catch
            # that here instead of silently reporting "started" for a
            # process that's already dead and wrote nothing.
            time.sleep(0.4)
            if proc.poll() is not None:
                log_file.close()
                with open(log_path) as f:
                    err = f.read().strip()[-500:]
                return {'ok': False, 'error': f'ros2 bag record exited immediately: {err}'}, 500
            self._recording_proc = proc
            self._recording_bag_path = bag_path
            self._recording_start_time = time.time()
        self.trigger_pub.publish(Bool(data=True))
        self.get_logger().warn(f'>>> EXPERIMENT RECORDING STARTED: {bag_path} <<<')
        return {'ok': True, 'bag_name': bag_name}, 200

    def stop_recording(self):
        with self._lock:
            proc = self._recording_proc
            bag_path = self._recording_bag_path
            self._recording_proc = None
            self._recording_bag_path = None
            self._recording_start_time = None
        if proc is None:
            return {'ok': False, 'error': 'not recording'}, 409
        self.trigger_pub.publish(Bool(data=False))
        # SIGINT, not kill/terminate(SIGTERM) -- `ros2 bag record` needs a
        # graceful shutdown to flush the mcap writer; a hard kill here can
        # leave the bag unreadable by summarize_bag/analyze_bag.py.
        proc.send_signal(signal.SIGINT)
        try:
            proc.wait(timeout=10.0)
        except subprocess.TimeoutExpired:
            self.get_logger().error(
                f'ros2 bag record did not exit within 10s of SIGINT for {bag_path} -- '
                'it may still be flushing, or the bag may be incomplete.')
        self.get_logger().warn(f'>>> EXPERIMENT RECORDING STOPPED: {bag_path} <<<')
        return {'ok': True, 'bag_path': bag_path}, 200

    def list_experiments(self):
        """Newest-first list of recorded bags under bags_dir."""
        entries = []
        if os.path.isdir(self.bags_dir):
            for name in os.listdir(self.bags_dir):
                full = os.path.join(self.bags_dir, name)
                if os.path.isdir(full):
                    entries.append({'name': name, 'mtime': os.path.getmtime(full)})
        entries.sort(key=lambda e: e['mtime'], reverse=True)
        return entries

    # --- Flask ---------------------------------------------------------------

    def _snapshot_state(self):
        with self._lock:
            bag_path = self._recording_bag_path
            recording = {
                'active': self._recording_proc is not None,
                'bag_name': os.path.basename(bag_path) if bag_path else None,
                'elapsed_sec': (time.time() - self._recording_start_time)
                if self._recording_start_time else None,
            }
            return {
                'config': {'mode': self.mode, 'audio_backend': self.audio_backend},
                'gaze': self._gaze_state,
                'pan_tilt_actual': self._pan_tilt_actual,
                'audio_current': self._audio_current,
                'audio_history': list(self._audio_history),
                'world_positions': self._build_world_positions(),
                'logs': list(self._logs),
                'recording': recording,
            }

    def _mjpeg_generator(self):
        # Runs on Flask's own request-handling thread, not the rclpy spin
        # thread -- just polls the shared frame buffer at a display-friendly
        # rate rather than trying to touch the node/executor from here.
        while True:
            with self._lock:
                frame = self._latest_jpeg
            if frame is not None:
                yield (b'--frame\r\nContent-Type: image/jpeg\r\n\r\n' + frame + b'\r\n')
            time.sleep(0.05)

    def _start_flask(self):
        app = Flask(__name__)

        @app.route('/')
        def index():
            return render_template_string(DASHBOARD_HTML)

        @app.route('/video_feed')
        def video_feed():
            return Response(
                self._mjpeg_generator(),
                mimetype='multipart/x-mixed-replace; boundary=frame')

        @app.route('/api/state')
        def api_state():
            return jsonify(self._snapshot_state())

        @app.route('/api/record/start', methods=['POST'])
        def api_record_start():
            body, status = self.start_recording()
            return jsonify(body), status

        @app.route('/api/record/stop', methods=['POST'])
        def api_record_stop():
            body, status = self.stop_recording()
            return jsonify(body), status

        @app.route('/api/experiments')
        def api_experiments():
            return jsonify(self.list_experiments())

        @app.route('/api/experiments/<name>')
        def api_experiment_detail(name):
            # name comes straight from the URL -- restrict to exactly the
            # bag-directory names list_experiments() itself produced
            # (timestamp strings), not an arbitrary filesystem path.
            if name not in {e['name'] for e in self.list_experiments()}:
                return jsonify({'error': 'not found'}), 404
            try:
                return jsonify(summarize_bag(os.path.join(self.bags_dir, name)))
            except Exception as e:
                return jsonify({'error': str(e)}), 500

        thread = threading.Thread(
            target=app.run,
            kwargs={'host': '0.0.0.0', 'port': self.http_port, 'threaded': True,
                    'use_reloader': False, 'debug': False},
            daemon=True)
        thread.start()


def main(args=None):
    rclpy.init(args=args)
    node = DashboardNode()
    try:
        rclpy.spin(node)
    except KeyboardInterrupt:
        pass
    finally:
        node.destroy_node()
        rclpy.try_shutdown()


if __name__ == '__main__':
    main()
