"""One-shot diagnostic: grab a live camera frame and run YOLO on it directly
with a near-zero confidence threshold, bypassing algo_gaze entirely. Tells you
definitively whether YOLO can see these pedestrians at all, and at what
confidence, independent of whatever threshold algo_gaze's node currently has.

Run (with your sim already launched in another terminal):
    source /opt/ros/humble/setup.bash
    source ~/Documents/BRONE_audio_gaze_project/install/setup.bash
    python3 diagnose_yolo.py

Writes two files next to this script:
    frame_raw.png       -- the captured camera frame, unmodified
    frame_yolo_all.png  -- same frame with EVERY YOLO person candidate drawn,
                           however low-confidence, labeled with its score
"""
import os
import rclpy
from rclpy.node import Node
from sensor_msgs.msg import Image
from cv_bridge import CvBridge
import cv2
from ultralytics import YOLO

HERE = os.path.dirname(os.path.abspath(__file__))


class Grabber(Node):
    def __init__(self):
        super().__init__('yolo_diagnose')
        self.bridge = CvBridge()
        self.frame = None
        self.sub = self.create_subscription(
            Image, '/robotis_op3/camera/image_raw', self.cb, 10)

    def cb(self, msg):
        self.frame = self.bridge.imgmsg_to_cv2(msg, 'bgr8')


def main():
    rclpy.init()
    node = Grabber()
    print('Waiting for a frame on /robotis_op3/camera/image_raw ...')
    start = node.get_clock().now()
    while rclpy.ok() and node.frame is None:
        rclpy.spin_once(node, timeout_sec=0.5)
        if (node.get_clock().now() - start).nanoseconds / 1e9 > 20.0:
            print('TIMEOUT: no frame received in 20s. Is the sim actually running '
                  'and is /robotis_op3/camera/image_raw publishing? '
                  '(ros2 topic hz /robotis_op3/camera/image_raw)')
            return
    node.destroy_node()
    rclpy.try_shutdown()

    frame = node.frame
    cv2.imwrite(os.path.join(HERE, 'frame_raw.png'), frame)
    print(f'Saved raw frame: {frame.shape}')

    model = YOLO('yolo11s.pt')  # matches algo_gaze's default model
    results = model(frame, classes=0, conf=0.01, verbose=False)  # near-zero threshold

    boxes = results[0].boxes
    vis = frame.copy()
    if boxes is None or len(boxes) == 0:
        print('\n*** ZERO person candidates found, even at conf=0.01. ***')
        print('YOLO is not recognizing this pedestrian model as a person at ANY')
        print('confidence -- this is a genuine detection failure (CGI domain gap),')
        print('not just a threshold that is set too high.')
    else:
        print(f'\n*** {len(boxes)} person candidate(s) found: ***')
        for i, box in enumerate(boxes):
            conf = float(box.conf[0])
            x1, y1, x2, y2 = map(int, box.xyxy[0])
            print(f'  #{i}: confidence={conf:.3f}  bbox=({x1},{y1})-({x2},{y2})')
            color = (0, 255, 0) if conf >= 0.3 else (0, 165, 255)
            cv2.rectangle(vis, (x1, y1), (x2, y2), color, 2)
            cv2.putText(vis, f'{conf:.2f}', (x1, y1 - 6),
                        cv2.FONT_HERSHEY_SIMPLEX, 0.6, color, 2)
        best = max(float(b.conf[0]) for b in boxes)
        print(f'\nBest score: {best:.3f}')
        if best < 0.3:
            print('This is BELOW the 0.3 sim-mode threshold -- that is why nothing')
            print('draws in algo_gaze right now. Options: lower yolo_conf_threshold')
            print('further (e.g. 0.15-0.2), or the model just cannot reliably see')
            print('this pedestrian style (consider mode:=hybrid for real detection).')
        else:
            print('This IS above the 0.3 sim-mode threshold. If algo_gaze still shows')
            print('nothing, the running node likely does not have your latest code/')
            print('param -- rebuild + re-source + relaunch:')
            print('  colcon build --symlink-install --packages-select algo_gaze')
            print('  source install/setup.bash')

    cv2.imwrite(os.path.join(HERE, 'frame_yolo_all.png'), vis)
    print(f'\nSaved annotated frame (all candidates): {os.path.join(HERE, "frame_yolo_all.png")}')


if __name__ == '__main__':
    main()
