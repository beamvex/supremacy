use crate::lzss::Emit;

/// VRAM sink for the interleaved modes — the CGA (`0x30695`/`0x30745`) and
/// TGA (`0x31148`/`0x31202`) decoder copies.
///
/// Both share one shape: after `w` emitted bytes the write cursor backs up
/// (`di −= w`), steps one bank forward (`di += step`), and on wrapping past
/// `span` drops back `span − pitch`. CGA: `step 0x2000`, `span 0x4000`,
/// `pitch 0x50`; TGA: `step 0x2000`, `span 0x8000`, `pitch 0xA0`.
pub struct Banked<'a> {
    /// Destination bytes (the banked VRAM buffer or a stand-in).
    pub buf: &'a mut [u8],
    /// Write cursor — `es:di` as a flat index.
    pub di: usize,
    /// Row width `w` in bytes (`ax` on decoder entry).
    pub width: usize,
    /// Bank size / cursor step per row (`0x2000` both modes).
    pub step: usize,
    /// Address that forces the pitch wrap (`0x4000` CGA, `0x8000` TGA).
    pub span: usize,
    /// Bytes advanced within bank 0 on wrap (`0x50` CGA, `0xA0` TGA).
    pub pitch: usize,
    /// XOR overlay draw (`mode_flag` 1).
    pub xor: bool,
    row_left: usize,
}

impl<'a> Banked<'a> {
    /// Set up a draw at `buf[di]` with the mode's bank geometry.
    #[must_use]
    pub fn new(
        buf: &'a mut [u8],
        di: usize,
        width: usize,
        step: usize,
        span: usize,
        pitch: usize,
        xor: bool,
    ) -> Self {
        Self {
            buf,
            di,
            width,
            step,
            span,
            pitch,
            xor,
            row_left: width,
        }
    }

    /// The row-advance sequence (`di −= w; di += step; wrap → pitch`).
    fn wrap(&mut self) {
        self.row_left = self.width;
        self.di += self.step - self.width;
        if self.di >= self.span {
            self.di -= self.span - self.pitch;
        }
    }
}

impl Emit for Banked<'_> {
    /// `stosb` or `xor [es:di],al`, then the countdown/`di` bank advance.
    fn emit(&mut self, b: u8) {
        if self.xor {
            self.buf[self.di] ^= b;
        } else {
            self.buf[self.di] = b;
        }
        self.di += 1;
        self.row_left -= 1;
        if self.row_left == 0 {
            self.wrap();
        }
    }
}
