use super::{Ar, Ports};

impl<P: Ports> Ar<P> {
    /// `0x2A04` — the `AdLib` apply for channel `c` (`bp = c` is the
    /// OPL channel, `[0x1FD0]` the record). A new program first keys
    /// the old note off (kind != 3), then `vol_apply` at
    /// `progmap.hi·2` and `0x2DE6` when the program differs.
    pub(super) fn apply_a(&mut self, c: usize) {
        self.chan = c;
        let prog = self.chans[c].prog;
        if prog != 0 {
            if self.chans[c].kind != 3 && self.chans[c].notes[0] != 0 {
                self.note_off_a(c);
            }
            let v = self.progmap(prog - 1).1.wrapping_shl(1);
            let v = if v >= 0x80 { 0x7F } else { v };
            let i = c & 0xF;
            if self.volc[i] != v {
                self.volc[i] = v;
                self.vol_apply(c, v);
            }
            let p = prog - 1;
            if self.chans[c].last_prog != p {
                self.chans[c].last_prog = p;
                self.patch_select(c, p);
            }
        }
        if self.chans[c].notes.iter().all(|&n| n == 0) {
            self.ext_a(c);
            return;
        }
        if self.chans[c].kind == 3 {
            self.slide_set_a(c);
            self.ext_a(c);
            return;
        }
        self.ext_a(c);
        let n = self.chans[c].notes[0];
        self.note_on_a(c, n);
    }

    /// `0x2ABE` — the `AdLib` continuous pass: kind `1`/`2` ramp the
    /// `fnum` by `param·2` (octave wrap), `3` steps the slide, `0xA`
    /// ramps `cvol` into `vol_apply`.
    pub(super) fn service_a(&mut self, c: usize) {
        self.chan = c;
        if self.chans[c].kind | self.chans[c].param == 0 {
            return;
        }
        let p = self.chans[c].param;
        match self.chans[c].kind {
            1 => self.ramp(c, i16::from(p) * 2),
            2 => self.ramp(c, -i16::from(p) * 2),
            3 => self.slide_a(c),
            0x0A => {
                let d = p >> 4;
                if d != 0 {
                    self.chans[c].cvol = self.chans[c].cvol.saturating_add(d).min(0x40);
                } else {
                    self.chans[c].cvol = self.chans[c].cvol.saturating_sub(p & 0xF);
                }
                self.avol(c);
            }
            _ => {}
        }
    }

    /// `0x2B19` — the `cvol` → `vol_apply` bridge (`volc` dedup,
    /// `cvol·2` clamped `0x7F`).
    fn avol(&mut self, c: usize) {
        let v = self.chans[c].cvol;
        let i = c & 0xF;
        if self.volc[i] == v {
            return;
        }
        self.volc[i] = v;
        let s = v.wrapping_shl(1);
        self.vol_apply(c, if s >= 0x80 { 0x7F } else { s });
    }

    /// `0x2BB5` — the `AdLib` ext dispatch: `0xD`/`0xB` `rst`/`idx`,
    /// `0xC` absolute `cvol`, `0xF` tempo, `4`/`other` ignored,
    /// `6` kind-clear + `0x2F64` note-off.
    fn ext_a(&mut self, c: usize) {
        let p = self.chans[c].param;
        match self.chans[c].kind {
            0x0D => self.rst = !self.rst,
            0x0B => {
                self.idx = p.wrapping_sub(1);
                self.rst = !self.rst;
            }
            0x0C => {
                self.chans[c].cvol = p.min(0x40);
                self.avol(c);
            }
            0x0F => {
                let v = p & 0x1F;
                if v != 0 {
                    self.cnt = 0;
                    self.div = v;
                }
            }
            6 => {
                self.chans[c].kind = 0;
                self.note_off_a(c);
            }
            _ => {}
        }
    }

    /// `0x2B59`/`0x2B86` — the `fnum` ramp: `fnum += d` wrapping at the
    /// `0x157..0x2AE` octave bounds (`block` tracking), then
    /// `0x2ECF` re-writes `A0`/`B0`.
    fn ramp(&mut self, c: usize, d: i16) {
        let mut f = i32::from(self.chans[c].fnum) + i32::from(d);
        let mut b = self.chans[c].block;
        while f >= 0x2AE {
            f += 0x157 - 0x2AE;
            b = b.wrapping_add(1);
        }
        while f <= 0x142 {
            f = 0x28A + (0x142 - f);
            b = b.wrapping_sub(1);
        }
        self.chans[c].fnum = u16::try_from(f & 0xFFFF).unwrap_or_default();
        self.chans[c].block = b;
        self.freq_apply(c);
    }

    /// `0x2C2C` — arm an `AdLib` slide: `+0x12`/`+0x0C` = target
    /// note/`ftab` value, `+0x0E` = `1` when sliding up (the asm's
    /// `jc` leaves `0` for target-below-current).
    fn slide_set_a(&mut self, c: usize) {
        let n = self.chans[c].notes[0];
        self.chans[c].slide_note = n;
        let t = self.ftab(n);
        self.chans[c].slide_tgt = t;
        self.chans[c].slide_dir = 0;
        if t == self.chans[c].pitch {
            self.chans[c].slide_tgt = 0;
        } else if t > self.chans[c].pitch {
            self.chans[c].slide_dir = 1;
        }
    }

    /// `0x2C5D` — one `AdLib` slide step: `param` latches `+0x0F`;
    /// `fnum` walks `rate·2` toward `slide_tgt` (full `{block,fnum}`
    /// compare); on arrival `+0x12` keys on, else `0x2ECF`.
    fn slide_a(&mut self, c: usize) {
        let p = self.chans[c].param;
        if p != 0 {
            self.chans[c].slide_rate = p;
            self.chans[c].param = 0;
        }
        if self.chans[c].slide_tgt == 0 {
            return;
        }
        self.ramp(c, self.rstep(c));
        let full = self.chans[c].fnum | u16::from(self.chans[c].block) << 12;
        let done = if self.chans[c].slide_dir == 0 {
            full <= self.chans[c].slide_tgt
        } else {
            full >= self.chans[c].slide_tgt
        };
        if done {
            self.chans[c].slide_tgt = 0;
            let n = self.chans[c].slide_note;
            self.note_on_a(c, n);
        }
    }

    /// The slide's signed step for this channel (`± rate·2`).
    fn rstep(&self, c: usize) -> i16 {
        let r = i16::from(self.chans[c].slide_rate) * 2;
        if self.chans[c].slide_dir == 0 {
            -r
        } else {
            r
        }
    }

    /// `ftab[n]` — the `cs:0x1D60` `fnum` table (`n - 0xC`, floored at
    /// the table base like `note_on` does).
    fn ftab(&self, n: u8) -> u16 {
        self.d16(0x1D60 + u16::from(n.saturating_sub(0xC)) * 2)
    }
}
