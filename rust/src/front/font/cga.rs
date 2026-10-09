//! CGA `0x30887` — opaque 2bpp copy: one packed byte per row into the
//! interleaved banks (`movsb`, then the `di ^= 0x2000` / `+0x50` row
//! advance — the same values [`Video::Cga::di_at`] yields per row).

use super::{ds8, src};
use crate::args::Video;

/// `ds:0x2F19` pointer table / `ds:0x3C51` glyph data.
const TAB: usize = 0x2F19;
const BASE: usize = 0x3C51;

/// Blit glyph `ch` at `(x4, y)` — 1 byte × 6 rows, colours baked.
pub(super) fn draw(buf: &mut [u8], img: &[u8], ch: u8, x4: u16, y: u16) {
    let Some(si) = src(img, TAB, BASE, ch) else {
        return;
    };
    for r in 0..6usize {
        let di = Video::Cga.di_at(usize::from(x4), usize::from(y) + r);
        if let (Some(b), Some(d)) = (ds8(img, si + r), buf.get_mut(di)) {
            *d = b;
        }
    }
}
