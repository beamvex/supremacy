use super::Keyboard;

impl Keyboard {
    /// Fresh state: empty stack (`count = -1` like the `0xFFFF` the game
    /// writes to `cs:0x8B7C`), no pending code.
    #[must_use]
    pub fn new() -> Self {
        Self {
            buf: [0; 10],
            count: -1,
            last: 0,
            pending: false,
            mods: 0,
            reboot: false,
        }
    }

    /// The int-9 ISR — file `0x31D74`: classify modifier make/break codes
    /// (`0x31DB5`), latch `[0x9CCC]`/`[0x9CCB]`, stack-push the code.
    pub fn irq(&mut self, scan: u8) {
        self.modifiers(scan);
        self.last = scan;
        self.pending = true;
        self.push(scan);
    }

    /// Stack push — file `0x31E4A`/ISR tail: skipped when `count == 9`
    /// (BIOS-style drop-on-full), else `buf[++count] = scan`.
    pub fn push(&mut self, scan: u8) {
        if self.count < 9 {
            self.count += 1;
            let i = usize::try_from(self.count).unwrap_or(0);
            self.buf[i] = scan;
        }
    }

    /// Modifier make/break tracking — file `0x31DB5`. Ctrl `0x1D`/`0x9D`,
    /// Alt `0x38`/`0xB8`, Del `0x53`/`0xD3`; all three held → reboot.
    fn modifiers(&mut self, scan: u8) {
        let bit = match scan {
            0x1D | 0x9D => 1,
            0x38 | 0xB8 => 2,
            0x53 | 0xD3 => 4,
            _ => return,
        };
        if scan & 0x80 == 0 {
            self.mods |= bit;
        } else {
            self.mods &= !bit;
        }
        self.reboot |= self.mods == 7;
    }
}

impl Default for Keyboard {
    fn default() -> Self {
        Self::new()
    }
}
