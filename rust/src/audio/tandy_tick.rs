use super::{Ports, Tandy};

impl<P: Ports> Tandy<P> {
    /// `ah=0` — file `0x1B6B`: `[0x2A] += [0x25]` gates the whole
    /// music block — a carry jumps straight to the `0x61` tail, so
    /// overflow ticks render nothing; while `[0x23]` holds,
    /// `[0x29] += [0x27]` overflow steps all four channels; then the
    /// `0x1DAE` render pass and the `0x61 | 0x60` Tandy sound-enable
    /// write.
    pub fn tick(&mut self) {
        let (beat, over) = self.beat.overflowing_add(self.pace);
        self.beat = beat;
        if !over {
            if self.mus != 0 {
                let (acc, fire) = self.step_acc.overflowing_add(self.tempo);
                self.step_acc = acc;
                if fire {
                    for c in 0..4 {
                        if self.step(c) {
                            return; // `0x86` ran the tick tail itself
                        }
                    }
                }
            }
            self.render();
        }
        self.gate();
    }

    /// File `0x1B93` — the render loop: `cx` counts 4→0 inside the
    /// per-channel code, which is why ch3's tail writes the noise
    /// register instead of a divisor pair; the sfx path's `ret` pops
    /// the whole render call, so it ends the loop early.
    pub(super) fn render(&mut self) {
        for c in 0..4 {
            if self.voice(c) {
                return;
            }
        }
    }

    /// File `0x1B9B` — the `0x61` enable bits the tick tail always
    /// writes.
    pub(super) fn gate(&mut self) {
        let v = self.ports.inb(0x61);
        self.ports.outb(0x61, v | 0x60);
    }
}
