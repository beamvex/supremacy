use super::consts::N;
use super::token::token;
use super::{Decoder, Emit};

impl Decoder {
    /// flag = 0 path (`0x2EC42`/`0x2FCAF`): `lodsw` token; a zero token
    /// followed by a `00` byte is the stream terminator. Otherwise copy
    /// `(t >> 8 & 0xF) + 3` bytes from ring position
    /// `(t & 0xFF) | ((t & 0xF000) >> 4)`.
    pub(super) fn copy_match(&mut self, src: &[u8], i: &mut usize, sink: &mut impl Emit) -> bool {
        let Some(t) = token(src, i) else {
            return false;
        };
        if t == 0 && src.get(*i).is_none_or(|&b| b == 0) {
            return false;
        }
        let mut pos = usize::from(t & 0xFF | ((t & 0xF000) >> 4));
        for _ in 0..((t >> 8) & 0xF) + 3 {
            let b = self.ring[pos];
            sink.emit(b);
            self.push(b);
            pos = (pos + 1) & (N - 1);
        }
        true
    }
}
