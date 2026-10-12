use super::{Ar, Ports};

impl<P: Ports> Ar<P> {
    /// `0x23EF`/`0x23F9` — the post-`fetch` apply for channel `c`
    /// (0-based; `bp = c + 1` is the MIDI channel). Program changes
    /// send `B0+ch,7,v` (`progmap.hi·2` clamped `0x7F`, dedup'd by
    /// `volc`) and `C0+ch` (dedup'd by `+0xB`); kind-3 notes arm the
    /// slide instead of keying on.
    pub(super) fn apply_r(&mut self, c: usize) {
        self.chan = c;
        let bp = u8::try_from(c + 1).unwrap_or(u8::MAX);
        let prog = self.chans[c].prog;
        if prog != 0 {
            if self.chans[c].notes.iter().any(|&n| n != 0) {
                let v = self.progmap(prog - 1).1.wrapping_shl(1);
                self.rvol(bp, if v >= 0x80 { 0x7F } else { v });
            }
            let p = prog - 1;
            if self.chans[c].last_prog != p {
                self.chans[c].last_prog = p;
                self.midi(0xC0 + bp);
                self.midi(p);
            }
        }
        if self.chans[c].notes.iter().all(|&n| n == 0) {
            self.ext_r(c, bp);
            return;
        }
        if self.chans[c].kind == 3 {
            self.slide_set(c);
            self.ext_r(c, bp);
            return;
        }
        self.ext_r(c, bp);
        let n = self.chans[c].notes;
        self.note_on(c, n[0]);
        for &x in &n[1..] {
            self.note_aux(bp, x);
        }
    }

    /// `0x258A` — the Roland pending-event dispatch (ext ops).
    /// `0xD`/`0xB` toggle `rst` (+ `idx`), `0xC` absolute volume,
    /// `0xF` tempo, `4` modulation, `5` pan, `6` all-notes-off,
    /// `7`/`8` `sfxbuf` `SysEx` triggers.
    fn ext_r(&mut self, c: usize, bp: u8) {
        let p = self.chans[c].param;
        match self.chans[c].kind {
            0x0D => self.rst = !self.rst,
            0x0B => {
                self.idx = p.wrapping_sub(1);
                self.rst = !self.rst;
            }
            0x0C => {
                let v = p.min(0x40);
                self.chans[c].cvol = v;
                self.cvol_send(c, bp);
            }
            0x0F => {
                let v = p & 0x1F;
                if v != 0 {
                    self.cnt = 0;
                    self.div = v;
                }
            }
            4 => {
                self.midi(0xB0 + bp);
                self.midi(1);
                self.midi((self.chans[c].notes[3] & 0xF).wrapping_shl(3));
            }
            5 => self.pan(bp, p),
            6 => {
                self.chans[c].kind = 0;
                self.noff(bp);
            }
            7 => {
                self.chans[c].kind = 0;
                self.sfxbuf[0x0A] = p;
                let b = self.sfxbuf;
                self.syx(&b);
            }
            8 => {
                self.chans[c].kind = 0;
                self.sfxbuf[0x0B] = p & 0xF;
                self.sfxbuf[0x0C] = p >> 4;
                let b = self.sfxbuf;
                self.syx(&b);
            }
            _ => {}
        }
    }

    /// `0x260D` — kind-5 pan: `param & 0xF0` pans right (`0x40 +
    /// nibble·4`), `param & 0xF` pans left (`0x40 - nibble·4`), zero
    /// centres — and zero also clears `kind` (the asm's `[si+5] = 0`
    /// on the centre path), so a `0` param doesn't re-pan each pass.
    /// `B0+ch, 0x0A, v`.
    fn pan(&mut self, bp: u8, p: u8) {
        if p == 0 {
            self.chans[usize::from(bp) - 1].kind = 0;
        }
        let v = if p & 0xF0 != 0 {
            0x40 + ((p & 0xF0) >> 2)
        } else if p & 0xF != 0 {
            0x40u8.wrapping_sub((p & 0xF).wrapping_shl(2))
        } else {
            0x40
        };
        self.midi(0xB0 + bp);
        self.midi(0x0A);
        self.midi(v);
    }
}
