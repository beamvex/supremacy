use super::consts::{FLAG_HI, SENTINEL};
use super::Decoder;

impl Decoder {
    /// `shr dx,1` — consume one flag bit. When the `0x100` sentinel has
    /// shifted out, reload `dl` from the stream and `dh = 0xFF`
    /// (`0x2EC0C`/`0x2FC7A`). Returns the consumed bit (`true` = literal).
    pub(super) fn next_flag(&mut self, src: &[u8], i: &mut usize) -> Option<bool> {
        self.flags >>= 1;
        if self.flags & SENTINEL == 0 {
            self.flags = u16::from(*src.get(*i)?) | FLAG_HI;
            *i += 1;
        }
        Some(self.flags & 1 != 0)
    }
}
