use super::{Pc, Ports};

impl<P: Ports> Pc<P> {
    /// The `0xF3D` stream step — one channel's decode. `wait` counts
    /// down; while it holds, the `0xF6F` sustain path applies the
    /// `0x40`/`0x80` pitch slide; on expiry the stream decodes until a
    /// note byte (`< 0x80`), a `0x80`/`0x8B` hold, or `0x87` (end —
    /// its `0xFA1` `pop ax` drops the step frame so the `ret` lands on
    /// the ISR epilogue: the whole `int 0x80` call is over, `true`,
    /// after clearing `[0x23]`/`[0x24]`/`[0x2A]`). Bytes `0xA0..0xDF`
    /// pick an envelope base through `cs:0x5EA`, bytes `0xE0..` set the
    /// `+0x07` tick reload; `0x80..0x9F` jump through the `cs:0x32`
    /// opcode table ([`Pc::opcode`]).
    pub(super) fn step(&mut self, c: usize) -> bool {
        let mut dl = self.chans[c].flags;
        self.chans[c].wait = self.chans[c].wait.wrapping_sub(1);
        if self.chans[c].wait != 0 {
            self.slide(c, dl);
            return false;
        }
        dl &= 0x30;
        let mut si = self.chans[c].ptr;
        for _ in 0..0x400 {
            let al = self.d8(si);
            si += 1;
            if al < 0x80 {
                self.note(c, al, dl, si);
                return false;
            }
            if al < 0xA0 && self.opcode(c, al, &mut dl, &mut si) {
                return al == 0x87;
            }
            if al >= 0xA0 {
                self.load_env(c, al);
            }
        }
        false
    }

    /// `0xF6F` — sustain: `0x40` set slides `+0x15` each tick, `0x80`
    /// picks the direction.
    fn slide(&mut self, c: usize, dl: u8) {
        if dl & 0x40 == 0 {
            return;
        }
        if dl & 0x80 != 0 {
            self.chans[c].note = self.chans[c].note.wrapping_sub(1);
        } else {
            self.chans[c].note = self.chans[c].note.wrapping_add(1);
        }
    }

    /// `0xF75`/`0xF5F` — the `≥ 0xA0` stream byte: `0xE0..` reloads the
    /// tick count (`+0x07`), `0xA0..0xDF` loads the envelope base
    /// (`+0x09`) from `cs:0x5EA + (byte + 0x20)`.
    fn load_env(&mut self, c: usize, al: u8) {
        let (v, carry) = al.overflowing_add(0x20);
        if carry {
            self.chans[c].reload = v.wrapping_add(1);
        } else {
            self.chans[c].env = self.d8(0x5EA + u16::from(v));
        }
    }

    /// `0xF86` + the `0xF97` tail — a note byte: stash the flags copy,
    /// reset the sweep, copy the fade/envelope seeds `+0x08`/`+0x09`
    /// into the live `+0x0A`/`+0x0B`, reload `wait`, store `ptr`.
    fn note(&mut self, c: usize, al: u8, dl: u8, si: u16) {
        let ch = &mut self.chans[c];
        ch.sweep = 0;
        ch.flags = dl;
        ch.note = al;
        ch.fade = ch.mux;
        ch.seq = ch.env;
        ch.wait = ch.reload;
        ch.ptr = si;
    }
}
