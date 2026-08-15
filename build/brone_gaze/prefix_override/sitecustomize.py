import sys
if sys.prefix == '/usr':
    sys.real_prefix = sys.prefix
    sys.prefix = sys.exec_prefix = '/home/evan/Documents/BRONE_audio_gaze_project/install/brone_gaze'
