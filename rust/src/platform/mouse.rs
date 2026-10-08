/// int-33h mouse state (`AX=3`-style query result) plus the `K`-mode flag.
///
/// With arg 2 = `K` (`[0x9CCD] = 1`) the game swaps the int-33h IVT entry
/// against saved `ds:[0x14D]`/`[0x14F]` (file `0x2A0DF`) and synthesises
/// button events from keyboard scancodes — see [`Mouse::feed_scan`].
/// With a real driver the same path is fed synthetic codes `0x7E`/`0x7D`/
/// `0xFE`/`0xFD` by the driver's own handler (`0x2A18E`).
pub struct Mouse {
    /// Cursor column in pixels (int33 `cx`, mirrored at `ds:[0x9CD6]`).
    pub x: u16,
    /// Cursor row in pixels (int33 `dx`, `ds:[0x9CD8]`).
    pub y: u16,
    /// Button bitmask — bit 0 left, bit 1 right (`ds:[0x9CDA]`).
    pub buttons: u8,
    /// `true` = `K` argument given: emulated, not a real driver.
    pub keyboard_emulated: bool,
}

/// A button event produced by [`Mouse::feed_scan`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MouseEvent {
    /// Left button pressed (`[0x9CDA] = 1`).
    LeftDown,
    /// Right button pressed (`[0x9CDA] = 2`).
    RightDown,
    /// Button released (`[0x9CDA] = 0`).
    Release,
}
