use super::{Ar, Ports};

impl<P: Ports> Ar<P> {
    /// `0x24B6` — the Roland continuous-event pass (non-`fetch` ticks):
    /// kind `1`/`2` ramp `bend` by `param·32`, kind `3` steps the
    /// slide, kind `0xA` ramps `cvol` (then the `0x2506` vol send).
    pub(super) fn service_r(&mut self, c: usize) {
        if self.chans[c].kind | self.chans[c].param == 0 {
            return;
        }
        let (p, bp) = (self.chans[c].param, u8::try_from(c + 1).unwrap_or(u8::MAX));
        match self.chans[c].kind {
            1 => {
                self.chans[c].bend += u16::from(p) * 32;
                self.bend_send(c);
            }
            2 => {
                self.chans[c].bend -= u16::from(p) * 32;
                self.bend_send(c);
            }
            3 => self.slide(c),
            0x0A => {
                let d = p >> 4;
                if d != 0 {
                    self.chans[c].cvol = self.chans[c].cvol.saturating_add(d).min(0x40);
                } else {
                    self.chans[c].cvol = self.chans[c].cvol.saturating_sub(p & 0xF);
                }
                self.cvol_send(c, bp);
            }
            _ => {}
        }
    }

    /// `0x2506` — `cvol` changed? Update `volc[bp]` and send
    /// `B0+ch, 7, cvol·2` (clamped `0x7F`).
    pub(super) fn cvol_send(&mut self, c: usize, bp: u8) {
        let v = self.chans[c].cvol;
        if self.volc[usize::from(bp)] == v {
            return;
        }
        self.volc[usize::from(bp)] = v;
        self.midi(0xB0 + bp);
        self.midi(7);
        let s = v.wrapping_shl(1);
        self.midi(if s >= 0x80 { 0x7F } else { s });
    }

    /// The `0x242E` apply-side volume send — `volc[bp]` dedup then
    /// `B0+ch, 7, v` (`v` is already `0x7F`-domain).
    pub(super) fn rvol(&mut self, bp: u8, v: u8) {
        if self.volc[usize::from(bp)] == v {
            return;
        }
        self.volc[usize::from(bp)] = v;
        self.midi(0xB0 + bp);
        self.midi(7);
        self.midi(v);
    }

    /// `0x255C` — the pitch-bend send: `E0+ch`, lsb, msb.
    fn bend_send(&mut self, c: usize) {
        let (b, bp) = (self.chans[c].bend, u8::try_from(c + 1).unwrap_or(u8::MAX));
        self.midi(0xE0 + bp);
        self.midi(b.to_le_bytes()[0] & 0x7F);
        self.midi((b >> 7).to_le_bytes()[0] & 0x7F);
    }

    /// `0x2689` — primary note-on `al`: `reset` `bend` to `0x2000` when
    /// it drifted (sending the new value), `B0+ch,0x7B,0` all-off,
    /// then `90+ch, note, 0x7F`; the note latches `+0x0A`.
    pub(super) fn note_on(&mut self, c: usize, n: u8) {
        if n == 0 {
            return;
        }
        let bp = u8::try_from(c + 1).unwrap_or(u8::MAX);
        if self.chans[c].bend != 0x2000 {
            self.chans[c].bend = 0x2000;
            self.bend_send(c);
        }
        self.noff(bp);
        self.chans[c].last_note = n;
        self.note_aux(bp, n);
    }

    /// `0x26CA` — the chord note-on (no bend/all-off `reset`):
    /// `90+ch, n, 0x7F`. Zero notes skip (`or al,al` guard).
    pub(super) fn note_aux(&mut self, bp: u8, n: u8) {
        if n == 0 {
            return;
        }
        self.midi(0x90 + bp);
        self.midi(n);
        self.midi(0x7F);
    }

    /// `B0+ch, 0x7B, 0` — the all-notes-off controller triplet.
    pub(super) fn noff(&mut self, bp: u8) {
        self.midi(0xB0 + bp);
        self.midi(0x7B);
        self.midi(0);
    }

    /// `0x26E3` — arm a slide: `+0x12` = target note, `+0x0C` =
    /// `rtab[target]`, `+0x10` = `rtab[last_note]`, `+0x0E` = dir
    /// (`1` when the target is below the current value).
    pub(super) fn slide_set(&mut self, c: usize) {
        let (t, l) = (self.chans[c].notes[0], self.chans[c].last_note);
        self.chans[c].slide_note = t;
        self.chans[c].slide_tgt = self.rtab(t);
        self.chans[c].pitch = self.rtab(l);
        self.chans[c].slide_dir = 0;
        if self.chans[c].slide_tgt == self.chans[c].pitch {
            self.chans[c].slide_tgt = 0;
        } else if self.chans[c].slide_tgt < self.chans[c].pitch {
            self.chans[c].slide_dir = 1;
        }
    }

    /// `0x2721` — one slide step (Roland): `param` latches the rate
    /// once; `pitch` walks `rtab` values toward `slide_tgt` while
    /// `bend` compensates `rate·32` the other way; on arrival the
    /// slide clears and `+0x12` keys on.
    fn slide(&mut self, c: usize) {
        let p = self.chans[c].param;
        if p != 0 {
            self.chans[c].slide_rate = p;
            self.chans[c].param = 0;
        }
        if self.chans[c].slide_tgt == 0 {
            return;
        }
        let r = u16::from(self.chans[c].slide_rate);
        let done = if self.chans[c].slide_dir == 0 {
            self.chans[c].pitch += r;
            self.chans[c].bend -= r * 32;
            if self.chans[c].pitch < self.chans[c].slide_tgt {
                self.bend_send(c);
                false
            } else {
                true
            }
        } else if self.chans[c].pitch < r {
            true
        } else {
            self.chans[c].pitch -= r;
            self.chans[c].bend += r * 32;
            if self.chans[c].pitch > self.chans[c].slide_tgt {
                self.bend_send(c);
                false
            } else {
                true
            }
        };
        if done {
            self.chans[c].slide_tgt = 0;
            let n = self.chans[c].slide_note;
            self.note_on(c, n);
        }
    }

    /// `rtab[n]` — the `cs:0x1B00` pitch table (word per note).
    fn rtab(&self, n: u8) -> u16 {
        self.d16(0x1B00 + u16::from(n) * 2)
    }
}
