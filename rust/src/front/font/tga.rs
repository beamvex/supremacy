//! TGA `0x3133B` — opaque 4bpp copy: `movsw` per row into the four-bank
//! layout (`di = … + x4*2`, row advance `+0x2000` wrapping past
//! `0x8000` by `−0x8000 + 0xA0` — [`Video::Tga::di_at`] per row).

use super::{ds8, src};
use crate::args::Video;

/// `ds:0x2F91` pointer table / `ds:0x3DB9` glyph data.
const TAB: usize = 0x2F91;
const BASE: usize = 0x3DB9;

/// Blit glyph `ch` at `(x4, y)` — 2 bytes × 6 rows, colours baked.
pub(super) fn draw(buf: &mut [u8], img: &[u8], ch: u8, x4: u16, y: u16) {
    let Some(si) = src(img, TAB, BASE, ch) else {
        return;
    };
    for r in 0..6usize {
        let di = Video::Tga.di_at(usize::from(x4) * 2, usize::from(y) + r);
        for c in 0..2 {
            if let (Some(b), Some(d)) = (ds8(img, si + r * 2 + c), buf.get_mut(di + c)) {
                *d = b;
            }
        }
    }
}
