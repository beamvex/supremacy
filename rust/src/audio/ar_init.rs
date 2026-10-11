use super::{Ar, Mpu, Ports};

impl<P: Ports> Ar<P> {
    /// `0x27CF` — Roland `init`: MPU `detect` + setup, and on success
    /// `[0x19AC] = 1`, `dvol = 0x4B`. Returns `al` = `detect` result.
    pub(super) fn roland_init(&mut self) -> u8 {
        if !self.roland_detect() {
            return 0;
        }
        self.mode = 1;
        self.dvol = 0x4B;
        1
    }

    /// `0x280D` — `0x2873` MPU `reset`/`detect`, then `0x281D` channel
    /// `init` + the `[0x1C07]` setup `SysEx` (with settle delay).
    fn roland_detect(&mut self) -> bool {
        let mut m = Mpu {
            ports: &mut self.ports,
            ack: self.ack,
        };
        if !m.reset() {
            self.ack = m.ack;
            return false;
        }
        self.ack = m.ack;
        self.roland_setup();
        true
    }

    /// `0x281D` — `0x28F5(0x3F)` UART-mode enter (skipped when already
    /// Roland), `B0/B0/E0` `init` triplets on channels 1–9, then the
    /// `cs:0x1C07` counted `SysEx` block (`0x28B9` send + delay).
    fn roland_setup(&mut self) {
        if self.mode != 1 {
            self.mputil(0x3F);
        }
        for ch in 1..10u8 {
            self.midi(0xB0 + ch);
            self.midi(0x7B);
            self.midi(0);
            self.midi(0xB0 + ch);
            self.midi(0x40);
            self.midi(0);
            self.midi(0xE0 + ch);
            self.midi(0);
            self.midi(0x40);
        }
        let b = self.img.get(0x1C07..0x1C07 + 0x10).unwrap_or(&[]).to_vec();
        self.syx(&b);
    }

    /// `0x2D3B` — `AdLib` `init`: `0x32A4` OPL `detect`, `0x2D55` register
    /// `init`; `mode = 0`, `dvol = 0x7F`. Returns `al` = `detect` result.
    pub(super) fn adlib_init(&mut self) -> u8 {
        if !self.opl_detect() {
            return 0;
        }
        self.opl_init();
        self.mode = 0;
        self.dvol = 0x7F;
        1
    }

    /// `0x2952` — `install` the int-8 `service`: `[0x1C85] = 0xFF`, PIT
    /// programmed at the module's `[0x1C00]` divisor (`0x5D37`).
    pub(super) fn install(&mut self) {
        self.timer = true;
        self.pit(self.d16(0x1C00));
    }

    /// `0x298C` — when the timer flag is set: `0x293D(0)` restores
    /// the BIOS rate and int 8 is chained back (`[0x1C85]` is left
    /// set — the asm doesn't clear it).
    pub(super) fn uninstall(&mut self) {
        if self.timer {
            self.pit(0);
        }
    }

    /// `0x293D` — `out 0x43,0x36` then the `ax` divisor lsb/msb to
    /// `0x40` (`ax = 0` restores the 18.2 Hz BIOS tick).
    fn pit(&mut self, ax: u16) {
        let [lo, hi] = ax.to_le_bytes();
        self.ports.outb(0x43, 0x36);
        self.ports.outb(0x40, lo);
        self.ports.outb(0x40, hi);
    }
}
