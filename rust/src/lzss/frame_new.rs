use super::Frame;

impl<'a> Frame<'a> {
    /// Set up a draw at `buf[di]`, `width` bytes per row, `stride` bytes
    /// between rows. `width == stride` gives a contiguous fill.
    #[must_use]
    pub fn new(buf: &'a mut [u8], di: usize, width: usize, stride: usize, xor: bool) -> Self {
        Self {
            buf,
            di,
            width,
            stride,
            xor,
            row_left: width,
        }
    }
}
