use super::{Pc, Ports, Speaker};

impl<P: Ports> Pc<P> {
    /// `ah=0` — file `0xDE3`: the `int 8`-rate service. On `[0x29]`
    /// non-overflow it steps both channels on `[0x28]` carry, renders
    /// each voice, fades/muxes to one divisor, then lets an armed sfx
    /// sweep at `0xE41` possibly override it, and programs PIT ch2 via
    /// the `0xE9F` write ([`Speaker::note`]).
    pub fn tick(&mut self) {
        let mut cx = 2;
        let (beat, over) = self.beat.overflowing_add(self.pace);
        self.beat = beat;
        if !over && self.mus != 0 {
            let (acc, step) = self.step_acc.overflowing_add(self.tempo);
            self.step_acc = acc;
            if step && (self.step(0) || self.step(1)) {
                return; // `0x87` popped the step frame — the ISR epilogue runs
            }
            cx = self.mix();
        }
        if self.sfx_on != 0 {
            cx = self.sweep(cx);
        }
        self.program(cx);
    }

    /// File `0xE01`–`0xE40` — render both voices, decay ch0's `[0x126]`
    /// fade, rotate/shift ch1's `[0x13E]` mux byte, carry keeps ch1.
    fn mix(&mut self) -> u16 {
        self.div0 = self.voice(0);
        let mut cx = self.voice(1);
        let ch0 = if self.chans[0].fade != 0 {
            self.chans[0].fade -= 1;
            self.div0
        } else {
            2
        };
        let ch1 = &mut self.chans[1].fade;
        let carry = if self.mux_ror != 0 {
            let c = *ch1 & 1 != 0;
            *ch1 = ch1.rotate_right(1);
            c
        } else {
            let c = *ch1 & 1 != 0;
            *ch1 >>= 1;
            c
        };
        if !carry {
            cx = ch0;
        }
        cx
    }

    /// File `0xE41`–`0xE97` — the sfx sweep engine; returns the divisor
    /// that wins the speaker (sfx `dx` when the `[0x45D]` rotate drops
    /// a bit, the music `cx` it was handed otherwise).
    fn sweep(&mut self, cx: u16) -> u16 {
        if self.sfx.end != 0 {
            self.sfx.end -= 1;
            if self.sfx.end == 0 {
                self.sfx_on = 0;
                return cx;
            }
        }
        self.sfx.phase = self.sfx.phase.wrapping_sub(1);
        if self.sfx.phase == 0 {
            self.sfx.phase = self.sfx.rphase;
            let c = self.sfx.sel & 1 != 0;
            self.sfx.sel = self.sfx.sel.rotate_right(1);
            self.sfx.cur += if c { self.sfx.da } else { self.sfx.db };
        }
        let mut dx = self.sfx.cur.wrapping_add(self.sfx.add);
        self.sfx.cnt = self.sfx.cnt.wrapping_sub(1);
        if self.sfx.cnt == 0 {
            self.sfx.cnt = self.sfx.rcnt;
            dx = self.sfx.alt;
        }
        self.sfx.cur = dx;
        let c = self.sfx.apply & 1 != 0;
        self.sfx.apply = self.sfx.apply.rotate_right(1);
        if c {
            cx
        } else {
            dx.cast_unsigned()
        }
    }

    /// File `0xE99`–`0xEBB` — the `0xE9F` note program plus the `0x61`
    /// gate bits; the same sequence [`Speaker::note`] models, so the
    /// `[0x2E]` cache is shared.
    fn program(&mut self, cx: u16) {
        let mut spk = Speaker {
            ports: &mut self.ports,
            last: self.last,
        };
        spk.note(cx);
        self.last = spk.last;
    }
}
