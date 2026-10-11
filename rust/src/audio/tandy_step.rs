use super::{Ports, Tandy};

/// `cs:0x762` — the `0xA0..0xBF`-byte → vibrato base table.
const INST_TAB: u16 = 0x762;
/// `cs:0x7E2` — tremolo `{wait, val}` sequence pairs.
const TREM_TAB: u16 = 0x7E2;
/// `cs:0x85F` — `esel → ecur` table.
const ENV_TAB: u16 = 0x85F;
/// Decode bound — the asm loops forever on a malformed stream.
const LIM: usize = 0x400;

impl<P: Ports> Tandy<P> {
    /// `0x10B2` — one channel's stream step. `wait` counts down;
    /// while it holds, `0x10E9`'s sustain slide runs. On expiry the
    /// decode loop reads bytes: `< 0x80` note, `0x80..0x8D` opcodes
    /// (`cs:0x36` — `0x8E..` runs off the table, so the port stops),
    /// `0xA0..0xBF` vibrato base, `0xC0..0xDF` envelope `select`,
    /// `0xE0..` wait reload. `0x86` ends the module outright.
    pub(super) fn step(&mut self, c: usize) {
        let ch = &mut self.chans[c];
        ch.wait = ch.wait.wrapping_sub(1);
        if ch.wait != 0 {
            self.slide(c);
            return;
        }
        let mut dl = 0u8;
        let mut si = self.chans[c].ptr;
        for _ in 0..LIM {
            let al = self.d8(si);
            si = si.wrapping_add(1);
            if al < 0x80 && self.note(c, al, dl, si) {
                break;
            }
            if (0x80..0xA0).contains(&al) && self.opcode(c, al, &mut dl, &mut si) {
                break;
            }
            if al >= 0xA0 {
                self.tail_class(c, al);
            }
        }
    }

    /// `0x10E9` — the sustain slide: `0x40` walks `+0x16` ±1,
    /// `0x80` picks down.
    fn slide(&mut self, c: usize) {
        let ch = &mut self.chans[c];
        if ch.flags & 0x40 == 0 {
            return;
        }
        if ch.flags & 0x80 != 0 {
            ch.note = ch.note.wrapping_sub(1);
        } else {
            ch.note = ch.note.wrapping_add(1);
        }
    }

    /// The `0xA0..` stream byte: `0xE0..` reload (`+0x09`),
    /// `0xC0..0xDF` envelope `select` (`+0x0A`), `0xA0..0xBF` vibrato
    /// base via `cs:0x762` (`+0x1A`) — the twin `+0x20` adds.
    fn tail_class(&mut self, c: usize, al: u8) {
        let (v, c1) = al.overflowing_add(0x20);
        if c1 {
            self.chans[c].reload = v.wrapping_add(1);
            return;
        }
        let (v, c2) = v.overflowing_add(0x20);
        if c2 {
            self.chans[c].esel = v;
        } else {
            self.chans[c].inst = self.d8(INST_TAB + u16::from(v));
        }
    }

    /// `0x105E` — a note byte `< 0x80`: tremolo pair reload (`+0x13`
    /// → `cs:0x7E2`), tie check on the OLD note's bit7 (set → skip
    /// the envelope/vibrato `reset`), then the `0x1095` tail. Returns
    /// `true` — a note always ends the decode.
    fn note(&mut self, c: usize, al: u8, dl: u8, si: u16) -> bool {
        let ch = &mut self.chans[c];
        ch.tcur = ch.tsel;
        let (tied, inst) = (ch.note & 0x80 != 0, ch.inst);
        ch.note = al;
        ch.vcur = inst;
        let tw = self.d8(TREM_TAB + u16::from(self.chans[c].tsel));
        self.chans[c].twait = tw;
        if tied {
            self.tail(c, dl, si);
            return true;
        }
        if self.chans[c].vlatch < 0xD0 || self.sfx_on == 0 {
            let e = self.d8(ENV_TAB + u16::from(self.chans[c].esel));
            self.chans[c].ecur = e;
            self.chans[c].ewait = 1;
        }
        self.tail(c, dl, si);
        true
    }

    /// `0x1095` — the shared decode tail: `run`/`wait`/`flags`/`ptr`.
    pub(super) fn tail(&mut self, c: usize, dl: u8, si: u16) {
        let ch = &mut self.chans[c];
        ch.run = ch.nadd;
        ch.wait = ch.reload;
        ch.flags = dl;
        ch.ptr = si;
    }
}
