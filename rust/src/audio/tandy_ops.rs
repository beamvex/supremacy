use super::{Ports, Tandy};

impl<P: Ports> Tandy<P> {
    /// The `cs:0x36` opcode table — 14 real entries (`0x80..0x8D`);
    /// `0x8E..` indexes past it onto the dispatch/`freq` tables — the
    /// asm jumps wild, the port ends the decode instead. Returns
    /// `true` when the channel's decode is done for this step:
    /// `0x80`/`0x88` run the `0x1095` tail, `0x86` ends the module.
    pub(super) fn opcode(&mut self, c: usize, op: u8, dl: &mut u8, si: &mut u16) -> bool {
        match op {
            0x80 => self.op_mute(c),
            0x81 => self.byte(si, |s, v| s.chans[c].tsel = v),
            0x82 => self.op_sweep(c, dl, si),
            0x83 => *dl |= 0xC0,
            0x84 => *dl |= 0x40,
            0x85 => self.op_next(c, si),
            0x86 => return self.op_end(),
            0x87 => self.byte(si, |s, v| s.gtrans = v),
            0x88 => {}
            0x89 => self.op_nadd(c, si),
            0x8A => self.byte(si, |s, v| s.chans[c].trans = v),
            0x8B => self.op_list(c, si),
            0x8C => self.op_tempo(si),
            0x8D => self.byte(si, |s, v| s.mvol = v),
            _ => return true,
        }
        if matches!(op, 0x80 | 0x88) {
            self.tail(c, *dl, *si);
            return true;
        }
        false
    }

    /// One `lodsb` operand into a field.
    fn byte(&mut self, si: &mut u16, f: impl Fn(&mut Self, u8)) {
        let v = self.d8(*si);
        *si = si.wrapping_add(1);
        f(self, v);
    }

    /// `0x104B` — `0x80` mute: freeze the envelope (`ewait = 0`)
    /// unless the sfx engine owns the channel (`latch ≥ 0xD0` while
    /// `[0x24]` armed) — then tail.
    fn op_mute(&mut self, c: usize) {
        if self.chans[c].vlatch < 0xD0 || self.sfx_on == 0 {
            self.chans[c].ewait = 0;
        }
    }

    /// `0xFC0` — `0x82` sweep: word `+0x0E`, byte `+0x12`, flag `8`.
    fn op_sweep(&mut self, c: usize, dl: &mut u8, si: &mut u16) {
        self.chans[c].sdelta = self.d16(*si);
        *si = si.wrapping_add(2);
        self.chans[c].scount = self.d8(*si);
        *si = si.wrapping_add(1);
        *dl |= 8;
    }

    /// `0xFA2` — `0x85` next stream: `list+2`, `0` word wraps to base.
    fn op_next(&mut self, c: usize, si: &mut u16) {
        let mut s = self.chans[c].list.wrapping_add(2);
        if self.d16(s) == 0 {
            s = self.chans[c].base;
        }
        self.chans[c].list = s;
        *si = self.d16(s);
    }

    /// `0x1FF7` — `0x86` end: `stop`, then one render + gate pass
    /// (the asm pops the step frame and pushes the `0x243` tick tail).
    fn op_end(&mut self) -> bool {
        self.stop();
        self.render();
        self.gate();
        true
    }

    /// `0xFD3` — `0x89` note-add word → `+0x18` (`run`'s reload).
    fn op_nadd(&mut self, c: usize, si: &mut u16) {
        self.chans[c].nadd = self.d16(*si);
        *si = si.wrapping_add(2);
    }

    /// `0xFE7` — `0x8B` new list: the operand word IS a list
    /// pointer — base/cursor both replaced, stream = `[list]`.
    fn op_list(&mut self, c: usize, si: &mut u16) {
        let s = self.d16(*si);
        self.chans[c].base = s;
        self.chans[c].list = s;
        *si = self.d16(s);
    }

    /// `0xFF3` — `0x8C` tempo operand → `[0x27]`/`[0x28]`.
    fn op_tempo(&mut self, si: &mut u16) {
        self.tempo = self.d8(*si);
        self.tempo0 = self.tempo;
        *si = si.wrapping_add(1);
    }
}
