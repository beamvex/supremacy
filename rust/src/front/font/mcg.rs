//! MCG `0x2EECE` — opaque 8bpp copy: `movsw ×2` per row into
//! `es:di = y*0x140 + x4*4`, row advance `di += 0x13C` (320 − 4).

use super::{ds8, src};
use crate::args::Video;

/// `ds:0x3009` pointer table / `ds:0x30F9` glyph data.
const TAB: usize = 0x3009;
const BASE: usize = 0x30F9;

/// Blit glyph `ch` at `(x4, y)` — 4 bytes × 6 rows, colours baked.
pub(super) fn draw(buf: &mut [u8], img: &[u8], ch: u8, x4: u16, y: u16) {
    let Some(si) = src(img, TAB, BASE, ch) else {
        return;
    };
    for r in 0..6usize {
        let di = Video::Mcga.di_at(usize::from(x4) * 4, usize::from(y) + r);
        for c in 0..4 {
            if let (Some(b), Some(d)) = (ds8(img, si + r * 4 + c), buf.get_mut(di + c)) {
                *d = b;
            }
        }
    }
}
