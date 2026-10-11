use super::{Ports, Tandy};

impl<P: Ports> Tandy<P> {
    /// `0x1CD6` — the two-voice sfx engine, entered on the first
    /// `≥ 0xD0`-latched channel (ch2 — it renders once a tick; its
    /// `0x1D4B` `ret` pops the render call, so the channels after it
    /// are skipped): `end` countdown, `cur` sweep with the
    /// `sel`-picked phase delta and `bcnt`-paced `reset`, divisor to
    /// ch3's latch + `0xE7` noise, ch2's `evol` as volume onto the
    /// `vsel`-picked channel — its partner muted. Returns `false`
    /// when the `end`-hit path fell through to `music` — the render
    /// continues to the next channel like the asm's `jmp 0x1DBE`.
    pub(super) fn sfx(&mut self, c: usize) -> bool {
        if self.end != 0 {
            self.end -= 1;
            if self.end == 0 {
                self.sfx_off();
                self.music(c);
                return false;
            }
        }
        self.env(c);
        self.sweep();
        let (w, m) = self.vsel_pick();
        self.vol(w, self.chans[2].evol);
        self.vol(m, 0);
        true
    }

    /// `0x1CE7`–`0x1D26` — the sweep divisor: `cur + add`, the
    /// `pcnt`-paced `sel` phase delta (`+6`/`+8`), the `bcnt`-paced
    /// `+2` `reset`, `| +4` or-mask, then ch3 tone pair + `0xE7` noise.
    fn sweep(&mut self) {
        let di = self.srec;
        let mut dx = self.cur.wrapping_add(self.d16(di));
        self.pcnt = self.pcnt.wrapping_sub(1);
        if self.pcnt == 0 {
            self.pcnt = self.d8(di + 0x0B);
            let da = if self.sel & 1 == 0 { 6 } else { 8 };
            self.sel = self.sel.rotate_right(1);
            dx = dx.wrapping_add(self.d16(di + da));
        }
        self.bcnt = self.bcnt.wrapping_sub(1);
        if self.bcnt == 0 {
            self.bcnt = self.d8(di + 0x0A);
            dx = self.d16(di + 2);
        }
        self.cur = dx;
        self.twrite(0xC0, dx | self.d16(di + 4));
        self.ports.outb(0xC0, 0xE7);
    }

    /// `0x1D2B` — the `vsel` rotate picks which of ch2 (`0xD0`) /
    /// ch3 (`0xF0`) gets the sfx volume; carry → ch3.
    fn vsel_pick(&mut self) -> (u8, u8) {
        let pick = self.vsel & 1 != 0;
        self.vsel = self.vsel.rotate_right(1);
        if pick {
            (0xF0, 0xD0)
        } else {
            (0xD0, 0xF0)
        }
    }

    /// `0x1CC5` — `end` hit: disarm and zero both sweep channels'
    /// env counters (the `bx=0` word writes at `[0x134]`/`[0x152]`),
    /// then render as music.
    fn sfx_off(&mut self) {
        self.sfx_on = 0;
        for i in 2..4 {
            self.chans[i].ewait = 0;
            self.chans[i].evol = 0;
        }
    }
}
