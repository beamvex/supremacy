//! `AdLib` note/key/frequency ops — `0x2F01` `note_on`,
//! `0x2F64` `note_off` (`0x2F70` the tail `all_off` reuses), `0x2ECF`
//! `freq_apply`, `0x3240` the shared key-write, `0x3273` the A0/B0
//! register tail.

use super::{Ar, Ports};

impl<P: Ports> Ar<P> {
    /// `0x2F01` — key `note` on for record `c`'s voice. The entry
    /// `ah` is the record index; a nonzero `+0x14` (`op4`) overrides
    /// it. `note < 0xC` clamps the table index to 0. Voices `0..6`
    /// key on with bit `0x20`; voice 6 keys then sets its rhythm
    /// mask bit; voice 7 only sets its mask bit; voice 8 keys both
    /// slots (`note` on 8, `note+7` on 7) then masks.
    pub(super) fn note_on_a(&mut self, c: usize, note: u8) {
        let al = note.saturating_sub(0xC);
        let ah = self.voice(c);
        match ah {
            0..=5 => {
                self.key_write(ah, al, 0x20);
                return;
            }
            6 => self.key_write(6, al, 0),
            8 => {
                self.key_write(8, al, 0);
                self.key_write(7, al.wrapping_add(7), 0);
            }
            _ => {}
        }
        let m = self.d8(0x1C99 + u16::from(ah) - 6);
        self.st8s(0x7B, self.st8(0x7B) | m);
        self.rhythm_write();
    }

    /// `0x2F64` — key-off: the `op4`-resolved voice goes through
    /// [`noff_tail`][Self::`noff_tail`].
    pub(super) fn note_off_a(&mut self, c: usize) {
        let ah = self.voice(c);
        self.noff_tail(ah);
    }

    /// `0x2F70` — the note-off tail: voices `0..6` rewrite their
    /// latched note with the key bit clear; rhythm voices `6..`
    /// clear their `cs:0x1C99` mask bit in `[0x1C98]` and re-emit
    /// `0xBD`. `all_off` drives `ah = 0..0xB` through it.
    pub(super) fn noff_tail(&mut self, ah: u8) {
        if ah < 6 {
            let n = self.notec[usize::from(ah)];
            self.key_write(ah, n, 0);
        } else {
            let m = !self.d8(0x1C99 + u16::from(ah) - 6);
            self.st8s(0x7B, self.st8(0x7B) & m);
            self.rhythm_write();
        }
    }

    /// `0x2ECF` — re-emit A0/B0 for record `c`'s voice from the
    /// record's `fnum`/`block` and the latched `keyc` bit.
    /// `jna` — voices above 6 skip (no A/B register).
    pub(super) fn freq_apply(&mut self, c: usize) {
        let ah = self.voice(c);
        if ah > 6 {
            return;
        }
        let ch = self.chans[c];
        let bx = ch.fnum | u16::from(ch.block) << 12;
        let cl = self.keyc[usize::from(ah)];
        self.freq_tail(ah, bx, cl);
    }

    /// `0x3240` — key-write for driver voice `ch`: latch `key`/`note`
    /// in `keyc`/`notec`, look up the `cs:0x1D60` `fnum`/block table at
    /// `note`, store `{pitch, block, fnum}` on the `[0x1FD0]` record
    /// (`self.chan` — whatever apply last left there), then A0/B0.
    pub(super) fn key_write(&mut self, ch: u8, note: u8, key: u8) {
        self.keyc[usize::from(ch)] = key;
        self.notec[usize::from(ch)] = note;
        let bx = self.d16(0x1D60 + u16::from(note) * 2);
        let r = &mut self.chans[self.chan];
        r.pitch = bx;
        r.fnum = bx & 0x3FF;
        r.block = (bx >> 12) as u8;
        self.freq_tail(ch, bx, key);
    }

    /// `0x3273` — `A0+ch = bx` low; `B0+ch = `fnum`-hi | block<<2 |
    /// key` (`bh & 3 | bh>>2 & 0x1C | cl`).
    fn freq_tail(&mut self, ch: u8, bx: u16, key: u8) {
        self.opl_reg(0xA0 + ch, (bx & 0xFF) as u8);
        let bh = (bx >> 8) as u8;
        self.opl_reg(0xB0 + ch, (bh & 3) | ((bh >> 2) & 0x1C) | key);
    }

    /// The `+0x14`-resolved voice id: `op4` when nonzero else `c`.
    pub(super) fn voice(&self, c: usize) -> u8 {
        if self.chans[c].op4 != 0 {
            self.chans[c].op4
        } else {
            u8::try_from(c).unwrap_or(u8::MAX)
        }
    }
}
