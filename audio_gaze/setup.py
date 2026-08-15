from setuptools import find_packages, setup

package_name = 'audio_gaze'

setup(
    name=package_name,
    version='0.0.1',
    packages=find_packages(exclude=['test']),
    data_files=[
        ('share/ament_index/resource_index/packages',
            ['resource/' + package_name]),
        ('share/' + package_name, ['package.xml']),
        ('share/' + package_name + '/launch', ['launch/audio_mock.launch.py']),
    ],
    install_requires=['setuptools'],
    zip_safe=True,
    maintainer='evan',
    maintainer_email='evan@todo.todo',
    description='Audio perception nodes (mock + future real mic-array driver) for vision-audio fusion attention.',
    license='TODO: License declaration',
    extras_require={
        'test': [
            'pytest',
        ],
    },
    entry_points={
        'console_scripts': [
            'mock_audio_node = audio_gaze.mock_audio_node:main',
        ],
    },
)
