use super::Keyboard;

impl Keyboard {
    /// Stack pop — file `0x31E2E`: `count == -1` (`0xFFFF`) leaves `al`
    /// untouched (modelled as `None`), else `al = buf[count--]`. This is
    /// a **LIFO**, not the BIOS FIFO — the newest code comes back first.
    pub fn pop(&mut self) -> Option<u8> {
        if self.count < 0 {
            return None;
        }
        let s = self.buf[usize::try_from(self.count).unwrap_or(0)];
        self.count -= 1;
        Some(s)
    }

    /// Consume the `[0x9CCB]`/`[0x9CCC]` latch — the path the K-mode
    /// mouse emulation (`0x2A1CD`) and menu wait loops poll.
    pub fn take_pending(&mut self) -> Option<u8> {
        self.pending.then(|| {
            self.pending = false;
            self.last
        })
    }
}
