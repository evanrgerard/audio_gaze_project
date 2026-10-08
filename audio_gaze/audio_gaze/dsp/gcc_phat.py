"""GCC-PHAT (Generalized Cross-Correlation with Phase Transform).

Standard TDOA estimator: whitens the cross-power spectrum (dividing by its
magnitude) before inverse-transforming, which sharpens the correlation peak
and makes the estimate far more robust to reverberation/spectral coloration
than plain cross-correlation -- the classic choice for mic-array DOA (e.g.
ReSpeaker's own onboard SSL) and the basis of SRP-PHAT beamforming
(see `beamformer.py`).
"""
import numpy as np


def gcc_phat(sig, refsig, fs, max_tau=None, interp=16):
    """Return `(tau_seconds, cross_correlation)` for `sig` relative to `refsig`.

    `tau > 0` means `sig` arrives `tau` seconds after `refsig`, i.e.
    `sig(t) ~= refsig(t - tau)`.

    `cross_correlation` is the (optionally time-limited) upsampled PHAT
    cross-correlation, centered so index `len(cross_correlation) // 2`
    corresponds to zero lag -- callers that need to sample it at an arbitrary
    predicted lag (e.g. `beamformer.srp_phat_doa`) can do so directly instead
    of re-running the FFT.
    """
    n = sig.shape[0] + refsig.shape[0]
    sig_f = np.fft.rfft(sig, n=n)
    ref_f = np.fft.rfft(refsig, n=n)
    cross = sig_f * np.conj(ref_f)
    denom = np.abs(cross)
    denom[denom < 1e-15] = 1e-15
    cc = np.fft.irfft(cross / denom, n=interp * n)

    max_shift = interp * n // 2
    if max_tau is not None:
        max_shift = min(int(interp * fs * max_tau), max_shift)

    cc = np.concatenate((cc[-max_shift:], cc[:max_shift + 1]))
    shift = int(np.argmax(np.abs(cc))) - max_shift
    tau = shift / float(interp * fs)
    return tau, cc


def fractional_delay(sig, delay_samples):
    """Delay a real 1-D signal by a (possibly fractional) number of samples
    via an FFT phase shift: `output(n) = sig(n - delay_samples)`.

    Used both by the mic-array simulator (to place a synthetic source at an
    exact geometric delay) and by the delay-and-sum beamformer (to undo each
    channel's relative delay). Treats `sig` as one periodic block -- fine for
    short simulated/captured frames, not a streaming filter.
    """
    n = sig.shape[0]
    sig_f = np.fft.rfft(sig)
    freqs = np.fft.rfftfreq(n)
    phase = np.exp(-2j * np.pi * freqs * delay_samples)
    return np.fft.irfft(sig_f * phase, n=n)
