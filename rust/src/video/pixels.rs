use super::Screen;
use crate::args::Video;

const W: usize = 320;
const H: usize = 200;

impl Screen {
    /// Expand the framebuffer to 320×200 palette-index pixels, row-major —
    /// the inverse of each mode's bank/packing layout (same mapping
    /// `tools/extract_png.py::expand` uses on decoded records).
    #[must_use]
    pub fn pixels(&self) -> Vec<u8> {
        let mut px = vec![0; W * H];
        for y in 0..H {
            for x in 0..W {
                px[y * W + x] = self.pixel_at(x, y);
            }
        }
        px
    }

    fn pixel_at(&self, x: usize, y: usize) -> u8 {
        match self.mode {
            Video::Mcga => self.buf[y * W + x],
            Video::Ega => self.ega_pixel(x, y),
            Video::Cga => {
                let b = self.buf[(y >> 1) * 0x50 + (y & 1) * 0x2000 + (x >> 2)];
                (b >> (6 - 2 * (x & 3))) & 3
            }
            Video::Tga => {
                let b = self.buf[(y >> 2) * 0xA0 + (y & 3) * 0x2000 + (x >> 1)];
                if (x & 1) == 1 {
                    b & 0xF
                } else {
                    b >> 4
                }
            }
        }
    }

    fn ega_pixel(&self, x: usize, y: usize) -> u8 {
        let mut v = 0;
        for p in 0..4 {
            let b = self.buf[p * 0x1F40 + y * 0x28 + (x >> 3)];
            v |= ((b >> (7 - (x & 7))) & 1) << p;
        }
        v
    }
}
