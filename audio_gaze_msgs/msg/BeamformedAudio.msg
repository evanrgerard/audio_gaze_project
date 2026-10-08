# Mono audio produced by steering a delay-and-sum beam at the currently
# estimated direction of arrival (see gcc_phat_node / audio_gaze.dsp.beamformer).
# Not needed for the vision-audio fusion path (that only needs AudioCue), but
# published for future use -- e.g. feeding a directionally-enhanced signal to
# a real VAD/ASR model instead of the raw single-channel energy heuristic.

std_msgs/Header header

uint32 sample_rate
float32 direction_deg   # steering direction used, AudioCue convention
float32[] samples       # mono, in [-1.0, 1.0]
