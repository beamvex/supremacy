use super::timer_new::CHAIN_EVERY;
use super::Timer;

impl Timer {
    /// One PIT pulse through the int-8 ISR (file `0x35BF`): the PIC is EOI'd
    /// every pulse (`out 0x20,0x20`), and every 3rd pulse — when the
    /// `[0x1BFE]` down-counter runs out — the saved BIOS handler is chained
    /// via `pushf`/`call far`. Returns `true` on the chain pulses.
    pub fn tick(&mut self) -> bool {
        self.phase -= 1;
        if self.phase == 0 {
            self.phase = CHAIN_EVERY;
            return true;
        }
        false
    }
}
