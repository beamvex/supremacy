use super::Decoder;
use super::Emit;

impl Decoder {
    /// flag = 1 path (`0x2EC1F`/`0x2FC8D`): `lodsb`, emit the byte, store it
    /// in the ring. `false` on truncated input.
    pub(super) fn literal(&mut self, src: &[u8], i: &mut usize, sink: &mut impl Emit) -> bool {
        let Some(&b) = src.get(*i) else {
            return false;
        };
        *i += 1;
        sink.emit(b);
        self.push(b);
        true
    }
}
