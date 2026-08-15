from setuptools import find_packages
from setuptools import setup

setup(
    name='audio_gaze_msgs',
    version='0.0.1',
    packages=find_packages(
        include=('audio_gaze_msgs', 'audio_gaze_msgs.*')),
)
