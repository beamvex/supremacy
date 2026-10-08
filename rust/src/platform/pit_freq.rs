/// 8253 input clock — 1.193182 MHz (4.77 MHz ÷ 4).
pub const PIT_HZ: u64 = 1_193_182;

use super::Timer;

impl Timer {
    /// Tick rate in millihertz for the programmed divisor. A divisor of `0`
    /// encodes 65536 on the real counter (~18.2 Hz).
    #[must_use]
    pub fn freq_mhz(&self) -> u64 {
        let div = if self.divisor == 0 {
            0x1_0000
        } else {
            u64::from(self.divisor)
        };
        PIT_HZ * 1000 / div
    }
}
