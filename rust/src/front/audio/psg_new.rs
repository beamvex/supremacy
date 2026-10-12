use super::psg::{Psg, LFSR_SEED, PSG_AMP};

/// One attenuator step ≈ −2 dB (`10^(−2/20)`).
const DB_STEP: f32 = 0.794_328_2;

impl Psg {
    /// A power-on PSG — every attenuator at `0xF` (silent), tones and
    /// noise zeroed, `lfsr` seeded, sampling at `rate` Hz.
    #[must_use]
    pub fn new(rate: u32) -> Self {
        Self {
            tone: [0; 3],
            att: [0xF; 4],
            noise: 0,
            latched: 0,
            lfsr: LFSR_SEED,
            phase: [0.0; 3],
            nclk: 0.0,
            gain: gains(),
            rate,
        }
    }

    /// Retarget the render rate — register state survives like the
    /// real chip (only the phase clocks retune).
    pub fn set_rate(&mut self, rate: u32) {
        self.rate = rate;
    }
}

/// The attenuation table — `0xF` is fully silent, each step below it
/// doubles every three steps (2 dB).
fn gains() -> [f32; 16] {
    let mut g = [0.0; 16];
    let mut v = PSG_AMP;
    for slot in g.iter_mut().take(15) {
        *slot = v;
        v *= DB_STEP;
    }
    g
}
