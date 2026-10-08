use super::{Opl, Ports};

impl<P: Ports> Opl<P> {
    /// Key-off a melodic channel — file `0x3E90`: regs `0xA0+ch` and
    /// `0xB0+ch` cleared, which drops the key-on bit.
    pub fn key_off(&mut self, ch: u8) {
        self.write(0xA0 + ch, 0);
        self.write(0xB0 + ch, 0);
    }

    /// F-number/block/key-on write — the register half of the note-on
    /// routine at file `0x3E5D`. `bx` there packs f-num low in `bl`, and
    /// `bh` carries the 2 f-num-high bits plus block in bits 2–4, so the
    /// `0xB0+ch` byte is `(bh & 3) | ((bh >> 1) & 0x1C) | keybits`.
    pub fn freq(&mut self, ch: u8, fnum: u16, block: u8, keybits: u8) {
        let [lo, hi] = fnum.to_le_bytes();
        self.write(0xA0 + ch, lo);
        self.write(0xB0 + ch, hi & 3 | (block & 7) << 2 | keybits);
    }
}
