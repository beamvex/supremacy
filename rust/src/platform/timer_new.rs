use super::Timer;

/// Chain interval — `mov byte [0x1BFE],3` in the ISR (file `0x35D6`).
pub(crate) const CHAIN_EVERY: u8 = 3;

impl Timer {
    /// Power-on state: BIOS divisor (`0` → 18.2 Hz) and the phase the ISR
    /// resets to.
    #[must_use]
    pub fn new() -> Self {
        Self {
            divisor: 0,
            phase: CHAIN_EVERY,
        }
    }
}

impl Default for Timer {
    fn default() -> Self {
        Self::new()
    }
}
