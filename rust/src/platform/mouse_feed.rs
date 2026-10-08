use super::{Mouse, MouseEvent};

impl Mouse {
    /// Feed one scancode through the emulated/driven button mapping —
    /// files `0x2A18E` (real driver) and `0x2A1CD` (`K` mode).
    ///
    /// `K` mode: `Enter` `0x1C` → left down, `Esc` `0x01` → right down,
    /// their breaks `0x9C`/`0x81` → release. Real driver: synthetic
    /// `0x7E`/`0x7D` = left/right down, `0xFE`/`0xFD` = release.
    /// Anything else (including the arrow-nav codes) returns `None`.
    pub fn feed_scan(&mut self, scan: u8) -> Option<MouseEvent> {
        let ev = if self.keyboard_emulated {
            match scan {
                0x1C => MouseEvent::LeftDown,
                0x01 => MouseEvent::RightDown,
                0x81 | 0x9C => MouseEvent::Release,
                _ => return None,
            }
        } else {
            match scan {
                0x7E => MouseEvent::LeftDown,
                0x7D => MouseEvent::RightDown,
                0xFE | 0xFD => MouseEvent::Release,
                _ => return None,
            }
        };
        self.buttons = match ev {
            MouseEvent::LeftDown => 1,
            MouseEvent::RightDown => 2,
            MouseEvent::Release => 0,
        };
        Some(ev)
    }
}
