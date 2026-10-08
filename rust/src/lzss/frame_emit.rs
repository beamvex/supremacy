use super::{Emit, Frame};

impl Emit for Frame<'_> {
    /// One output byte: `stosb` or `xor [es:di],al`/`inc di`, then the
    /// `dec [cs:0x5380]` row countdown and `di += stride - w` wrap.
    fn emit(&mut self, b: u8) {
        if self.xor {
            self.buf[self.di] ^= b;
        } else {
            self.buf[self.di] = b;
        }
        self.di += 1;
        self.row_left -= 1;
        if self.row_left == 0 {
            self.row_left = self.width;
            self.di += self.stride - self.width;
        }
    }
}
