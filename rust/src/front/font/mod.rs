//! The shipped 4×6 font — replaces the GAM-9 debug glyphs.
//!
//! All four `[0x1268]` puts share one shape: `ch − 0x20` indexes a
//! 60-entry `u16` pointer table, the pointer + a per-mode base locates
//! the packed record, and six rows land at the mode's `es:di` address.
//! The tables ship in the image's data segment (`ds:` = file
//! `0x106A0 + ofs`), so [`draw`] reads them straight out of `img`:
//!
//! | mode | routine | ptr table | data | record |
//! |---|---|---|---|---|
//! | MCG | `0x2EECE` | `ds:0x3009` | `ds:0x30F9` | 24 B, opaque 8bpp |
//! | EGA | `0x2FF33` | `ds:0x3081` | `ds:0x3699` | 6 B ×4 planes (`+0x168`) |
//! | CGA | `0x30887` | `ds:0x2F19` | `ds:0x3C51` | 6 B, opaque 2bpp |
//! | TGA | `0x3133B` | `ds:0x2F91` | `ds:0x3DB9` | 12 B, opaque 4bpp |
//!
//! Colours are baked into the records (the asm takes no ink argument);
//! codes `0x20..=0x5B` are the real glyphs — past that the index runs
//! into the next mode's table and draws garbage exactly like the DOS
//! puts. The row tables they index (`ds:0x28D9`/`0x2A69`/`0x2BF9`/
//! `0x2D89`) hold the same values [`Video::di_at`] computes, so each
//! mode's row address goes through it.

use crate::args::Video;
use crate::video::Screen;

mod cga;
mod ega;
mod mcg;
mod tga;

/// File offset of `ds:0` — dgroup segment `0xFAA` (entry and the puts'
/// callers do `mov ds,0xFAA`) × 16 plus the `0xC00`-byte MZ header.
const DS: usize = 0x106A0;

/// `sub al,0x20` index + table read — `ds:tab[idx]` + `base`, `None`
/// when `img` is too short to hold the entry (never for the real exe).
fn src(img: &[u8], tab: usize, base: usize, ch: u8) -> Option<usize> {
    let i = DS + tab + usize::from(ch.wrapping_sub(0x20)) * 2;
    let p = u16::from_le_bytes([*img.get(i)?, *img.get(i + 1)?]);
    Some(base + usize::from(p))
}

/// One `ds:` byte of glyph data.
fn ds8(img: &[u8], ofs: usize) -> Option<u8> {
    img.get(DS + ofs).copied()
}

/// The `[0x1268]` glyph put — `ch` as `al`, `x4` the 4-pixel column
/// (`bx`), `y` the pixel row (`dx`). No clipping in the asm; writes past
/// the modelled VRAM span are dropped instead.
pub fn draw(scr: &mut Screen, img: &[u8], ch: u8, x4: u16, y: u16) {
    match scr.mode {
        Video::Mcga => mcg::draw(&mut scr.buf, img, ch, x4, y),
        Video::Ega => ega::draw(&mut scr.buf, img, ch, x4, y),
        Video::Cga => cga::draw(&mut scr.buf, img, ch, x4, y),
        Video::Tga => tga::draw(&mut scr.buf, img, ch, x4, y),
    }
}
