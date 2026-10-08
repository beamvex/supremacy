use super::Mouse;

impl Mouse {
    /// Fresh state for the parsed `keyboard_mouse` config flag.
    #[must_use]
    pub fn new(keyboard_emulated: bool) -> Self {
        Self {
            x: 0,
            y: 0,
            buttons: 0,
            keyboard_emulated,
        }
    }

    /// Update the full int33-style state at once.
    pub fn set(&mut self, x: u16, y: u16, buttons: u8) {
        self.x = x;
        self.y = y;
        self.buttons = buttons;
    }
}
