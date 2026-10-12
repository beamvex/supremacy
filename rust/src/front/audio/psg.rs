/// Tandy PSG clock — the 3.579545 MHz SN76489/SN76496.
pub(super) const PSG_HZ: u64 = 3_579_545;
/// Per-voice mix amplitude — four channels of ±0.12 stay inside the
/// `sample` clamp next to the speaker and OPL.
pub(super) const PSG_AMP: f32 = 0.12;
/// LFSR reset state — the all-but-top-bit-clear pattern SN76489
/// emulators seed (`1 << 15` of a 16-bit register).
pub(super) const LFSR_SEED: u16 = 0x8000;
/// Noise shift divisors for `NF` 0–2 (`NF` 3 taps tone channel 2).
pub(super) const NF_DIV: [u16; 3] = [0x10, 0x20, 0x40];

/// The SN76489 behind port `0xC0` — the [`Bus`][super::Bus] side of the
/// file-`0x1950` driver's latch/data writes (GAM-7): three tone
/// channels plus an LFSR noise channel, each with a 4-bit attenuator
/// (2 dB steps, `0xF` silent). Register indices follow the chip's
/// `1 rr t dddd` latch byte — even = the data register of channel
/// `rr>>1` (tone divisor on 0/2/4, noise control on 6), odd = its
/// attenuator.
pub struct Psg {
    /// 10-bit tone divisors (`freq = PSG_HZ / (32·n)`), one per channel.
    pub(super) tone: [u16; 3],
    /// 4-bit attenuators — channels 0–2 plus noise at index 3.
    pub(super) att: [u8; 4],
    /// Noise control byte — bits 0–1 = `NF`, bit 2 = white (`FB`).
    pub(super) noise: u8,
    /// Register index the last latch byte selected (data-byte target).
    pub(super) latched: u8,
    /// 16-bit noise LFSR — bit 0 is the output tap; white noise adds
    /// bit 3 to the feedback.
    pub(super) lfsr: u16,
    /// Square-wave cycle phases for the tone channels.
    pub(super) phase: [f64; 3],
    /// Noise shift-clock accumulator — fires at `PSG_HZ / (16·n)`.
    pub(super) nclk: f64,
    /// Attenuator → gain table (2 dB steps, `0xF` = 0).
    pub(super) gain: [f32; 16],
    /// Output sample rate the phases accumulate against.
    pub(super) rate: u32,
}
