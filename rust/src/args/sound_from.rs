use super::Sound;

impl Sound {
    /// Port of the compare chain at `0x32042`-`0x32073`, returning the sound
    /// index (`al` → `[0x155]`) and the timer-ISR variant selector (`bx` →
    /// `[0x156]`) that picks which int-8 handler the PIT hook installs.
    /// Unrecognised letters take the `0x32071` default arm.
    #[must_use]
    pub fn from_arg(c: u8) -> (Self, u16) {
        match c {
            b'T' => (Self::Tandy, 0xD5),
            b'A' => (Self::AdLib, 0x1D0),
            b'R' => (Self::Roland, 0x1D0),
            _ => (Self::PcSpeaker, 0),
        }
    }
}
