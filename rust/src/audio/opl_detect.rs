use super::{Opl, Ports};

impl<P: Ports> Opl<P> {
    /// `AdLib` presence detect — file `0x3EA4`. The canonical OPL2 timer
    /// test: mask both timers (`reg 4 = 0x60`), clear flags
    /// (`reg 4 = 0x80`), snapshot status `st0`; load timer 1 (`reg 2 =
    /// 0xFF`), start it (`reg 4 = 0x21`), wait ~4×38 status reads, take
    /// `st1`; restore both reg-4 writes. Detected when `st0 & 0xE0 == 0`
    /// (quiet before) and `st1 & 0xE0 == 0xC0` (both timers expired).
    /// Returns the `cl` result the asm builds (1 = present).
    pub fn detect(&mut self) -> bool {
        self.write(0x04, 0x60);
        self.write(0x04, 0x80);
        let st0 = self.status();
        self.write(0x02, 0xFF);
        self.write(0x04, 0x21);
        for _ in 0..4 {
            self.delay();
        }
        let _ = self.status();
        let st1 = self.status();
        self.write(0x04, 0x60);
        self.write(0x04, 0x80);
        st0 & 0xE0 == 0 && st1 & 0xE0 == 0xC0
    }
}
