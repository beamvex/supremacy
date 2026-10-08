use super::consts::N;
use super::Decoder;

impl Decoder {
    /// `mov [bp],al` / `inc bp` / `and bp,0xFFF` — store an emitted byte in
    /// the ring and advance the write cursor.
    pub(super) fn push(&mut self, b: u8) {
        self.ring[self.r] = b;
        self.r = (self.r + 1) & (N - 1);
    }
}
