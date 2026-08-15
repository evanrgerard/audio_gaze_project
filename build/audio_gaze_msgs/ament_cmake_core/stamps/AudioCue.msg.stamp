# A single audio perception event: is someone speaking right now, and from
# what direction relative to the robot's forward/camera-center axis?

std_msgs/Header header

bool is_speech          # Voice Activity Detection result: true if speech energy detected
float32 direction_deg   # Direction of Arrival, degrees. 0 = straight ahead (camera center).
                         # Negative = to the robot's left, positive = to the robot's right.
                         # Range: -180 to 180 (mic arrays like ReSpeaker report 0-360;
                         # convert to this convention in the driver/adapter node).
float32 confidence      # 0.0-1.0, VAD/DOA confidence. Mock node can just send 1.0.
