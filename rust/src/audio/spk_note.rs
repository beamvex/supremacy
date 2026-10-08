use super::{Ports, Speaker};

const PIT_CMD: u16 = 0x43;
const PIT_CH2: u16 = 0x42;
const GATE: u16 = 0x61;

impl<P: Ports> Speaker<P> {
    /// Program channel 2 and open the gate — file `0x0E9F`:
    /// `out 0x43 = 0xB6` (ch 2, lo/hi, mode 3 square), divisor lo/hi to
    /// `0x42`, then `in 0x61 | 3` back out (bits 0–1 = timer-2 gate +
    /// speaker data enable). The `cmp [0x2E],cx / jz` skip becomes the
    /// `divisor == self.last` early exit.
    pub fn note(&mut self, divisor: u16) {
        if divisor != self.last {
            let [lo, hi] = divisor.to_le_bytes();
            self.ports.outb(PIT_CMD, 0xB6);
            self.ports.outb(PIT_CH2, lo);
            self.ports.outb(PIT_CH2, hi);
            self.last = divisor;
        }
        let b = self.ports.inb(GATE);
        self.ports.outb(GATE, b | 3);
    }

    /// Close the speaker gate (port-`0x61` bits 0–1 clear). The driver
    /// reaches silence by keying channels off rather than one canonical
    /// `and 0xFC` site, so this is the modelled counterpart of `note`.
    pub fn silence(&mut self) {
        let b = self.ports.inb(GATE);
        self.ports.outb(GATE, b & !3);
        self.last = 0;
    }
}
