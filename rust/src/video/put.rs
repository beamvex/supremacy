use super::Screen;
use crate::args::Video;

const W: usize = 320;
const H: usize = 200;

impl Screen {
    /// Write one pixel in the mode's layout — the write-side inverse of
    /// [`Screen::pixel_at`], used by the frontend's glyph put and popup
    /// frame (the `[0x1268]`/`[0x1262]` slots are opaque merges, so only
    /// the covered bits change):
    ///
    /// - MCG: `buf[y*0x140 + x] = color`.
    /// - EGA: bit `7-(x&7)` of `buf[p*0x1F40 + y*0x28 + (x>>3)]` per
    ///   plane `p`, set from `color` bit `p` (the map-mask write the
    ///   asm's `call 0x8498` loop performs).
    /// - CGA: bits `6-2*(x&3)` of `buf[(y>>1)*0x50 + (y&1)*0x2000 + (x>>2)]`.
    /// - TGA: nibble of `buf[(y>>2)*0xA0 + (y&3)*0x2000 + (x>>1)]`,
    ///   high for even `x`.
    pub fn put_pixel(&mut self, x: usize, y: usize, color: u8) {
        if x >= W || y >= H {
            return;
        }
        match self.mode {
            Video::Mcga => self.buf[y * W + x] = color,
            Video::Ega => self.ega_put(x, y, color),
            Video::Cga => self.cga_put(x, y, color),
            Video::Tga => self.tga_put(x, y, color),
        }
    }

    /// EGA plane write — OR/XOR-free merge of `color`'s four plane bits
    /// into the shared byte, like the `[0x1268]` nibble merge.
    fn ega_put(&mut self, x: usize, y: usize, color: u8) {
        let (ofs, bit) = (y * 0x28 + (x >> 3), 7 - (x & 7));
        for p in 0..4 {
            let b = &mut self.buf[p * 0x1F40 + ofs];
            *b = (*b & !(1 << bit)) | (((color >> p) & 1) << bit);
        }
    }

    /// CGA 2bpp merge into the banked byte.
    fn cga_put(&mut self, x: usize, y: usize, color: u8) {
        let i = (y >> 1) * 0x50 + (y & 1) * 0x2000 + (x >> 2);
        let s = 6 - 2 * (x & 3);
        self.buf[i] = (self.buf[i] & !(3 << s)) | ((color & 3) << s);
    }

    /// TGA 4bpp merge into the nibble of the 4-bank byte.
    fn tga_put(&mut self, x: usize, y: usize, color: u8) {
        let i = (y >> 2) * 0xA0 + (y & 3) * 0x2000 + (x >> 1);
        let s = if (x & 1) == 1 { 0 } else { 4 };
        self.buf[i] = (self.buf[i] & !(0xF << s)) | ((color & 0xF) << s);
    }
}
