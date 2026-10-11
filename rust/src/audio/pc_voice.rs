use super::{Pc, Ports};

/// Envelope-sequence table — `cs:0x6B9`, walked by the `+0x0B` cursor.
const SEQ_TAB: u16 = 0x6B9;
/// The PIT divisor lookup — `cs:0x70`, indexed by the transposed
/// (seq + note + gtrans + trans) ×2 byte.
const DIV_TAB: u16 = 0x70;

impl<P: Ports> Pc<P> {
    /// File `0xFBD`–`0x1046` — render one channel to a PIT divisor:
    /// walk the `cs:0x6B9` envelope sequence (negative bytes fold the
    /// cursor back to the `+0x09` base; `0xFF` sustains the raw note),
    /// transpose into `cs:0x70` for the divisor, apply tremolo/sweep,
    /// then the `flags & 3` half-mute gate.
    pub(super) fn voice(&mut self, c: usize) -> u16 {
        let next = self.chans[c].seq.wrapping_add(1);
        let mut al = self.d8(SEQ_TAB + u16::from(next));
        if al & 0x80 == 0 {
            self.chans[c].seq = next;
        } else {
            al = al.wrapping_add(1);
            if al != 0 {
                al = al.wrapping_add(0x7F);
                self.chans[c].seq = self.chans[c].env;
            }
        }
        let idx = al
            .wrapping_add(self.chans[c].note)
            .wrapping_add(self.gtrans)
            .wrapping_add(self.chans[c].trans)
            << 1;
        let mut cx = self.d16(DIV_TAB + u16::from(idx));
        let mut flags = self.chans[c].flags ^ 1;
        if flags & 0x10 != 0 {
            cx = self.tremolo(c, cx, &mut flags);
        }
        cx = self.sweep_add(c, cx, flags);
        self.chans[c].flags = flags;
        if flags & 3 == 3 {
            cx = 2;
        }
        cx
    }

    /// File `0xFF3`–`0x1020` — tremolo: depth `& flags` (a `0` depth
    /// acts as `1`), `+0x11` walks `±` toward the `+0x0F` limit and
    /// flips the `0x20` direction bit there; the centred value adds to
    /// the divisor.
    fn tremolo(&mut self, c: usize, cx: u16, flags: &mut u8) -> u16 {
        let depth = self.chans[c].tdep.max(1) & *flags;
        let lim = self.chans[c].tlim;
        let cur = if *flags & 0x20 == 0 {
            self.chans[c].tcur.wrapping_sub(depth)
        } else {
            self.chans[c].tcur.wrapping_add(depth)
        };
        self.chans[c].tcur = cur;
        if cur == lim {
            *flags ^= 0x20;
        }
        let delta = i16::from(cur.wrapping_sub(lim >> 1).cast_signed());
        cx.wrapping_add(delta.cast_unsigned())
    }

    /// File `0x1027`–`0x1038` — the sweep add: when the `+0x0E` counter
    /// runs out, `+0x13` accumulates into `+0x0C` and onto the divisor.
    fn sweep_add(&mut self, c: usize, cx: u16, flags: u8) -> u16 {
        if flags & 8 == 0 {
            return cx;
        }
        let ch = &mut self.chans[c];
        ch.scount = ch.scount.wrapping_sub(1);
        if ch.scount != 0 {
            return cx;
        }
        ch.scount = 1;
        ch.sweep = ch.sweep.wrapping_add(ch.sdelta);
        cx.wrapping_add(ch.sweep)
    }
}
