# One block of synchronized multi-channel microphone-array audio, as consumed
# by gcc_phat_node for GCC-PHAT/SRP-PHAT direction-of-arrival estimation and
# delay-and-sum beamforming. Published either by a real mic-array capture
# driver or, for hardware-free testing, by sim_mic_array_node.

std_msgs/Header header

uint32 sample_rate      # samples per second, per channel
uint32 num_channels      # number of microphones
uint32 frame_size        # samples per channel in this message

# Interleaved, sample-major, in [-1.0, 1.0]:
#   data[s * num_channels + c] = sample s of channel c
# length == frame_size * num_channels
float32[] data
