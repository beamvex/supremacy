use super::{Decoder, Emit};

impl Decoder {
    /// Decode one stream from `src[*i]` into `sink`, leaving `*i` just past
    /// the `00 00` terminator token.
    ///
    /// `eat_tail` reproduces the only difference between the EGA routines and
    /// the others: at `0x2FCF7`/`0x2FD9A` the EGA decoder executes a final
    /// `lodsb` before `ret`, consuming the zero byte that follows the
    /// terminator so `si` lands on the next plane's stream.
    pub fn run(&mut self, src: &[u8], i: &mut usize, sink: &mut impl Emit, eat_tail: bool) {
        while self.step(src, i, sink) {}
        if eat_tail {
            *i += 1;
        }
    }
}
