use super::Timer;

impl Timer {
    /// `prog_pit` (file `0x353D`): channel 0, mode 3, `ax` as the lo/hi
    /// divisor. `timer_uninstall` passes `0` to restore 18.2 Hz.
    pub fn program(&mut self, divisor: u16) {
        self.divisor = divisor;
    }
}
