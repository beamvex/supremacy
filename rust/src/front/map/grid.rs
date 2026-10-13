//! The `[0x127C]`/`[0x127E]`/`[0x1280]` star-grid painters (MCG
//! `0x2F398`/`0x2F3CB`/`0x2F3FE`).
//!
//! The painters fill a screen region with a sparse star grid —
//! `(bx & 0x101)` per pixel pair, `bx += 0x3039`, `bx` reseeded per row
//! by the `cs:0xE93C` LCG (`s = 13s + 7` over the `cs:0x55A2`/`0x55A4`
//! pair, returning `hi ^ lo`). `[0x127C]` paints the shell's bottom-right
//! galaxy minimap; the `[0x1298]` dirty-mark and `0xA479` housekeeping
//! tails are bookkeeping the port drops.

use super::{cs_put, cs_word};

/// `cs:` LCG seed cells (`0xE93C`).
const RNG_SEED: u16 = 0x55A2;

/// The `0x2F398`/`0x2F3CB`/`0x2F3FE` painter body — `rows` lines of
/// `pairs` `(bx & 0x101)` words from `di`, `tail` byte skip per row.
pub fn grid(buf: &mut [u8], img: &mut [u8], di: u16, rows: u16, pairs: u16, tail: u16) {
    let mut di = usize::from(di);
    for _ in 0..rows {
        let mut bx = rng(img);
        for _ in 0..pairs {
            stosw(buf, di, bx & 0x101);
            di = di.wrapping_add(2);
            bx = bx.wrapping_add(0x3039);
        }
        di = di.wrapping_add(usize::from(tail));
    }
}

/// `cs:0xE93C` — `s = 13·s + 7` over the `cs:0x55A2`/`0x55A4` seed
/// pair; returns `hi16(s) ^ lo16(s)` (the `cs:0x55A6`+ scratch is
/// transient — only the seed pair persists).
fn rng(img: &mut [u8]) -> u16 {
    let s = u32::from(cs_word(img, RNG_SEED)) | u32::from(cs_word(img, RNG_SEED + 2)) << 16;
    let s = s.wrapping_mul(13).wrapping_add(7);
    cs_put(img, RNG_SEED, u16::try_from(s & 0xFFFF).unwrap_or(0));
    cs_put(img, RNG_SEED + 2, u16::try_from(s >> 16).unwrap_or(0));
    u16::try_from(s >> 16).unwrap_or(0) ^ u16::try_from(s & 0xFFFF).unwrap_or(0)
}

/// `es:` word store (the painters' `stosw`).
fn stosw(buf: &mut [u8], i: usize, v: u16) {
    if let Some(p) = buf.get_mut(i) {
        *p = u8::try_from(v & 0xFF).unwrap_or(0);
    }
    if let Some(p) = buf.get_mut(i + 1) {
        *p = u8::try_from(v >> 8).unwrap_or(0);
    }
}
