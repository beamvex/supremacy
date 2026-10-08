use super::{Decoder, Emit};

impl Decoder {
    /// One iteration of the decode loop (`jmp 0xEC0C`): a flag bit then the
    /// literal or match it selects. `false` on terminator or truncated input.
    pub(super) fn step(&mut self, src: &[u8], i: &mut usize, sink: &mut impl Emit) -> bool {
        match self.next_flag(src, i) {
            Some(true) => self.literal(src, i, sink),
            Some(false) => self.copy_match(src, i, sink),
            None => false,
        }
    }
}
