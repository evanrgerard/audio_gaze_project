from setuptools import find_packages, setup

package_name = 'gaze_dashboard'

setup(
    name=package_name,
    version='0.0.1',
    packages=find_packages(exclude=['test']),
    data_files=[
        ('share/ament_index/resource_index/packages',
            ['resource/' + package_name]),
        ('share/' + package_name, ['package.xml']),
        ('share/' + package_name + '/launch', ['launch/dashboard.launch.py']),
    ],
    install_requires=['setuptools', 'flask', 'opencv-python'],
    zip_safe=True,
    maintainer='evan',
    maintainer_email='evan@todo.todo',
    description='Local web dashboard for observing the algo_gaze / audio_gaze system live.',
    license='TODO: License declaration',
    extras_require={
        'test': [
            'pytest',
        ],
    },
    entry_points={
        'console_scripts': [
            'dashboard_node = gaze_dashboard.dashboard_node:main',
        ],
    },
)
