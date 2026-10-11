use super::{Ports, Tandy};

/// `cs:0x5C` — the SN76489 10-bit divisor table (`note·2`).
const FREQ_TAB: u16 = 0x5C;
/// `cs:0x7E2` — tremolo `{wait, val}` pairs.
const TREM_TAB: u16 = 0x7E2;
/// `cs:0x847` — vibrato sequence bytes.
const VIB_TAB: u16 = 0x847;
/// `cs:0x87C` — envelope `{add, wait}` pairs.
const ENV_TAB: u16 = 0x87C;

impl<P: Ports> Tandy<P> {
    /// File `0x1DAE` — one channel's render dispatch: `≥ 0xD0`-latched
    /// channels (2 and 3) take the sfx path while `[0x24] & 0x81` is
    /// nonzero, and the `test`'s sign bit decides which — set → the
    /// `0x1CD8` engine, clear → a `jns` to the `0x1CBC` `ret`. Both
    /// leave the render loop early (`true`); only the sfx `end`-hit
    /// fall-through to `music` carries on to the next channel.
    pub(super) fn voice(&mut self, c: usize) -> bool {
        if self.chans[c].vlatch < 0xD0 {
            self.music(c);
            return false;
        }
        let masked = self.sfx_on & 0x81;
        if masked == 0 {
            self.music(c);
        } else if masked & 0x80 != 0 {
            return self.sfx(c);
        }
        masked != 0
    }

    /// File `0x1DBE` — the music render: unarmed → `vlatch|0xF` mute
    /// (`0x1D4C`); else env step, the `stc; adc [0x26]` volume fold
    /// (carry keeps the wrapped sum, else `0`), vibrato, transpose
    /// (ch3 skips `[0x2B]`), then the channel's tone write.
    pub(super) fn music(&mut self, c: usize) {
        if self.mus == 0 {
            self.ports.outb(0xC0, self.chans[c].vlatch | 0xF);
            return;
        }
        self.env(c);
        let (v, ov) = self.chans[c].evol.carrying_add(self.mvol, true);
        let latch = self.chans[c].vlatch;
        self.vol(latch, if ov { v } else { 0 });
        let mut al = self.vib(c).wrapping_add(self.chans[c].note);
        if c != 3 {
            al = al.wrapping_add(self.gtrans);
        }
        al = al.wrapping_add(self.chans[c].trans);
        self.tone(c, al);
    }

    /// `0x1D56` — the tone tail: `loop` lands ch3 (`cx` = 0) on the
    /// `0x1D58` noise-register write (`0xE0 | note&0xF`); 0–2 run
    /// tremolo/sweep then the `cs:0x5C[note] + run` 10-bit divisor.
    fn tone(&mut self, c: usize, al: u8) {
        if c == 3 {
            self.ports.outb(0xC0, al & 0xF | 0xE0);
            return;
        }
        let run = self.trem_run(c);
        let idx = u16::from(al.wrapping_shl(1));
        let dx = self.d16(FREQ_TAB + idx).wrapping_add(run);
        let tl = self.chans[c].tlatch;
        self.twrite(tl, dx);
    }

    /// `0x1D8C` — the `cs:0x7E2` tremolo/`+0x12` sweep accumulation:
    /// returns the new `run` (signed table value + sweep delta when
    /// `scount` wraps and flag `8` holds).
    fn trem_run(&mut self, c: usize) -> u16 {
        let d = u16::from_ne_bytes(i16::from(i8::from_ne_bytes([self.trem(c)])).to_ne_bytes());
        let ch = &mut self.chans[c];
        let mut ax = ch.run.wrapping_add(d);
        ch.scount = ch.scount.wrapping_sub(1);
        if ch.scount == 0 {
            ch.scount = 1;
            if ch.flags & 8 != 0 {
                ax = ax.wrapping_add(ch.sdelta);
            }
        }
        ch.run = ax;
        ax
    }

    /// `0x1D67` — the tremolo seq: `{wait, val}` pairs at `cs:0x7E2`;
    /// `val` is the per-tick `run` increment. A `0` wait byte loops
    /// the cursor to the index stored in `val`'s place.
    fn trem(&mut self, c: usize) -> u8 {
        let mut bl = self.chans[c].tcur;
        let al = self.d8(TREM_TAB + 1 + u16::from(bl));
        self.chans[c].twait = self.chans[c].twait.wrapping_sub(1);
        if self.chans[c].twait != 0 {
            return al;
        }
        bl = bl.wrapping_add(2);
        let mut ah = self.d8(TREM_TAB + u16::from(bl));
        if ah == 0 {
            bl = self.d8(TREM_TAB + 1 + u16::from(bl));
            ah = self.d8(TREM_TAB + u16::from(bl));
        }
        self.chans[c].tcur = bl;
        self.chans[c].twait = ah;
        al
    }

    /// `0x1DD9` — the vibrato byte: positive values `advance` the
    /// cursor and offset the note; `0xFF` ends it (no offset, cursor
    /// held); other negatives give `val + 0x80` and loop the cursor
    /// to `+0x1A`. The `+0x1B` cursor indexes `cs:0x847 + cursor + 1`.
    fn vib(&mut self, c: usize) -> u8 {
        let idx = self.chans[c].vcur.wrapping_add(1);
        let mut al = self.d8(VIB_TAB + u16::from(idx));
        if al & 0x80 == 0 {
            self.chans[c].vcur = idx;
        } else {
            al = al.wrapping_add(1);
            if al == 0 {
                return 0;
            }
            al = al.wrapping_add(0x7F);
            self.chans[c].vcur = self.chans[c].inst;
        }
        al
    }

    /// `0x1E1A` — one envelope step: `evol += cs:0x87C[ecur]` each
    /// tick; `ewait` expiry reloads from `+1` and walks the cursor
    /// two bytes. `ewait 0` leaves the envelope frozen.
    pub(super) fn env(&mut self, c: usize) {
        if self.chans[c].ewait == 0 {
            return;
        }
        let add = self.d8(ENV_TAB + u16::from(self.chans[c].ecur));
        let ch = &mut self.chans[c];
        ch.evol = ch.evol.wrapping_add(add);
        ch.ewait -= 1;
        let (reload, cur) = (ch.ewait == 0, ch.ecur);
        if reload {
            let w = self.d8(ENV_TAB + 1 + u16::from(cur));
            let ch = &mut self.chans[c];
            ch.ewait = w;
            ch.ecur = cur.wrapping_add(2);
        }
    }

    /// `0x1D3D` — attenuation write: `~v >> 4 | latch` to `0xC0`.
    pub(super) fn vol(&mut self, latch: u8, v: u8) {
        self.ports.outb(0xC0, latch | (0xFF - v) >> 4);
    }

    /// `0x1E05` — the 10-bit tone pair: `latch | (d & 0xF)` then
    /// `d >> 6` (the `shr dx,2; shr al,2` top-six-bits).
    pub(super) fn twrite(&mut self, latch: u8, d: u16) {
        self.ports.outb(0xC0, latch | (d & 0xF).to_le_bytes()[0]);
        self.ports.outb(0xC0, (d >> 6).to_le_bytes()[0]);
    }
}
