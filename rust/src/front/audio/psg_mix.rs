use super::psg::{Psg, NF_DIV, PSG_HZ};

impl Psg {
    /// One output sample — the three tone squares plus noise, while
    /// `on` (the `0x61 | 0x60` enable the driver's tick tail writes)
    /// holds; silent otherwise.
    pub fn mix(&mut self, on: bool) -> f32 {
        if !on {
            return 0.0;
        }
        let mut s = 0.0;
        for c in 0..3 {
            s += self.square(c);
        }
        s + self.noise_out()
    }

    /// Channel `c`'s square wave — toggles at `PSG_HZ / (32·n)`, scaled
    /// by its attenuator; a `0` divisor runs at `n = 1` like the chip.
    #[allow(clippy::cast_precision_loss)]
    fn square(&mut self, c: usize) -> f32 {
        let n = self.tone[c].max(1);
        let step = PSG_HZ as f64 / (32.0 * f64::from(n) * f64::from(self.rate));
        self.phase[c] = (self.phase[c] + step) % 1.0;
        let sign = if self.phase[c] < 0.5 { 1.0 } else { -1.0 };
        sign * self.gain[usize::from(self.att[c])]
    }

    /// The noise channel — the LFSR shifts at `PSG_HZ / (16·n)` where
    /// `n` is the `NF` divisor or channel 2's tone (`NF == 3`); bit 0
    /// drives the output, white mode (`FB`) XORs bit 3 into feedback.
    #[allow(clippy::cast_precision_loss)]
    fn noise_out(&mut self) -> f32 {
        let n = match self.noise & 3 {
            3 => self.tone[2].max(1),
            nf => NF_DIV[usize::from(nf)],
        };
        self.nclk += PSG_HZ as f64 / (16.0 * f64::from(n) * f64::from(self.rate));
        while self.nclk >= 1.0 {
            self.nclk -= 1.0;
            self.shift();
        }
        let out = if self.lfsr & 1 == 0 { -1.0 } else { 1.0 };
        out * self.gain[usize::from(self.att[3])]
    }

    /// One LFSR step — feedback is bit 0 (periodic) or bit 0 ⊕ bit 3
    /// (white noise, `FB` set).
    fn shift(&mut self) {
        let mut fb = self.lfsr & 1;
        if self.noise & 4 != 0 {
            fb ^= self.lfsr >> 3 & 1;
        }
        self.lfsr = self.lfsr >> 1 | fb << 15;
    }
}
