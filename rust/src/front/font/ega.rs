//! EGA `0x2FF33` — four map-mask passes: each plane's 6-row record is
//! nibble-merged into `es:di` (the high nibble for an even `x4`, the low
//! for odd — stored bytes carry the row's 4 pixels in both nibbles).
//! `ds:0x3699` is four `0x168`-byte plane blocks (60 glyphs × 6 rows).

use super::{ds8, src};
use crate::args::Video;

/// `ds:0x3081` pointer table / `ds:0x3699` glyph data.
const TAB: usize = 0x3081;
const BASE: usize = 0x3699;
/// Stride between a glyph's plane records (60 glyphs × 6 rows).
const PSTEP: usize = 0x168;
/// One VRAM plane in the [`crate::video::Screen`] model.
const PLANE: usize = 0x1F40;

/// Merge glyph `ch` at `(x4, y)` into all four planes.
pub(super) fn draw(buf: &mut [u8], img: &[u8], ch: u8, x4: u16, y: u16) {
    let Some(si) = src(img, TAB, BASE, ch) else {
        return;
    };
    let col = usize::from(x4) / 2;
    for p in 0..4 {
        for r in 0..6usize {
            let di = p * PLANE + Video::Ega.di_at(col, usize::from(y) + r);
            merge(buf, img, di, si + p * PSTEP + r, x4 & 1 == 0);
        }
    }
}

/// `and`/`or` one glyph byte into the destination nibble — `hi` keeps
/// the glyph byte's high nibble (`and al,0xF0` path), `!hi` the low.
fn merge(buf: &mut [u8], img: &[u8], di: usize, si: usize, hi: bool) {
    let (Some(g), Some(d)) = (ds8(img, si), buf.get_mut(di)) else {
        return;
    };
    *d = if hi {
        (*d & 0x0F) | (g & 0xF0)
    } else {
        (*d & 0xF0) | (g & 0x0F)
    };
}
