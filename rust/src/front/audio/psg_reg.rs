use super::psg::{Psg, LFSR_SEED};

impl Psg {
    /// `out 0xC0` — the SN76489 single-port protocol: a `1rrtdddd`
    /// latch byte selects register `rr·2 + t` and takes the low
    /// nibble (tone low bits / noise control / attenuation); a `0`-bit
    /// data byte delivers the top six bits to the last-latched *tone*
    /// register (data bytes on an attenuator/noise latch are dropped —
    /// the driver never emits them).
    pub fn write(&mut self, val: u8) {
        if val & 0x80 == 0 {
            self.data(val);
            return;
        }
        let reg = (val >> 4) & 7;
        self.latched = reg;
        let d = val & 0xF;
        if reg & 1 == 0 {
            self.latch(reg, d);
        } else {
            self.att[usize::from(reg >> 1)] = d;
        }
    }

    /// The latch half — even register indices: 0/2/4 = tone low
    /// nibble, 6 = the 3-bit noise control (`NF` + white) with the
    /// LFSR reset the chip performs on a noise write.
    fn latch(&mut self, reg: u8, d: u8) {
        if reg == 6 {
            self.noise = d & 7;
            self.lfsr = LFSR_SEED;
            return;
        }
        let c = usize::from(reg >> 1);
        self.tone[c] = self.tone[c] & 0x3F0 | u16::from(d);
    }

    /// The data half — only tone registers have a second byte:
    /// `(top6 << 4) | low4` reassembles the driver's `d & 0xF` /
    /// `(d >> 4) & 0x3F` pair back into `d & 0x3FF`.
    fn data(&mut self, val: u8) {
        if self.latched > 4 || self.latched & 1 != 0 {
            return;
        }
        let c = usize::from(self.latched >> 1);
        self.tone[c] = self.tone[c] & 0xF | u16::from(val & 0x3F) << 4;
    }
}
