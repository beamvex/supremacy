use super::{Sound, Video};

/// Parsed `GAME` command line — the state written to `[0x9CC6]`, `[0x9CCD]`,
/// `[0x155]` and `[0x156]` by the asm routine.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Config {
    /// Video mode + parameter template (`[0x9CC6]`).
    pub video: Video,
    /// `es:0x84 == 'K'`: hook int 33h and emulate the mouse from the
    /// keyboard (`[0x9CCD] = 1`); any other value = real mouse driver.
    pub keyboard_mouse: bool,
    /// Sound device (`[0x155]`).
    pub sound: Sound,
    /// Timer-ISR variant selector (`[0x156]`).
    pub timer_variant: u16,
}
