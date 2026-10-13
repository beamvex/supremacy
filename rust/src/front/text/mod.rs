//! `menu_text` (`cs:0x2F629`, file `0x2F629`–`0x2F6C9`) — the paged
//! text-stream renderer the title attract and the ending sequences
//! call through `[0x1296]`.
//!
//! The stream lives in the image's stack region — `ss = 0x1F6F`
//! (EXEPACK `real_ss`), so `ss:di` = file `0x202F0 + di`. Each page is
//! a `{fb14, fb15}` header then line records `{xoff, ops…}`: `0x00`
//! ends the stream (the caller latches `[0x9CCB]`), `0xFF` suspends
//! with the cursor left on the next page header, `0xF0`+arg starts a
//! new line (`si −= arg`, `si += 0x1F40`), `0x20` runs the `0x2F519`
//! tile clear and any other byte is a 16×24 glyph tile the `0x2F4E3`
//! blit draws from `cs:0x21C4 + (op−0x41)·0x180`.
//!
//! Only the MCG variant is ported — the EGA/CGA/TGA `menu_text` twins
//! (`0x30520`/`0x30FFE`/`0x31CAA`, different `si` spaces and glyph
//! formats) drop their tile ops into the caller's `undrawn` tally like
//! the pre-render walk did (GAM-46).

use crate::args::Video;
use crate::video::Screen;

mod mcg;

/// `ss:` stream base in the unpacked image — `ss = 0x1F6F`, file
/// `0x1F6F0 + 0xC00`.
const STREAM: usize = 0x202F0;
/// The page's `es:` origin — `si = 0x3840` (row 45) each call.
const ORG: u16 = 0x3840;

/// How a line's op loop ended.
enum Flow {
    /// `0xF0` — new line armed, keep going.
    More,
    /// `0xFF` — suspend; the cursor stays on the next page header.
    Suspend,
    /// `0x00` — stream end (`ax |= 0x100`).
    End,
}

/// One page at `img[STREAM + *cur]` (`u16::MAX` keeps `*cur`).
/// Returns `(ax, undrawn)` — `ax` = `0x100` when the `0x00` terminator
/// ran, OR'd with the page's `fb15` byte (the scroll variants test
/// it); `undrawn` counts tile ops the mode's `menu_text` doesn't model.
pub fn page(scr: &mut Screen, img: &[u8], cur: &mut u16, arg: u16) -> (u16, usize) {
    if arg != u16::MAX {
        *cur = arg;
    }
    let mut di = STREAM + usize::from(*cur);
    let fb = u16::from(byte(img, di + 1));
    di += 2;
    let (mut undrawn, mut si) = (0usize, ORG);
    loop {
        si = si.wrapping_add(u16::from(byte(img, di)));
        di += 1;
        match line(scr, img, &mut di, &mut si, &mut undrawn) {
            Flow::End => return (0x100 | fb, undrawn),
            Flow::Suspend => {
                *cur = u16::try_from(di - STREAM).unwrap_or(0);
                return (fb, undrawn);
            }
            Flow::More => {}
        }
    }
}

/// The inner op loop (file `0x2F64C`–`0x2F691`) — `More` on the `0xF0`
/// newline, `Suspend`/`End` on the terminators.
fn line(scr: &mut Screen, img: &[u8], di: &mut usize, si: &mut u16, undrawn: &mut usize) -> Flow {
    loop {
        let op = byte(img, *di);
        *di += 1;
        match op {
            0x00 => return Flow::End,
            0xFF => return Flow::Suspend,
            0xF0 => {
                *si = si
                    .wrapping_sub(u16::from(byte(img, *di)))
                    .wrapping_add(0x1F40);
                *di += 1;
                return Flow::More;
            }
            _ => tile(scr, img, op, si, undrawn),
        }
    }
}

/// One tile op — `0x20` clears, anything else blits `op − 0x41`;
/// non-MCG modes just count the op (`si += 0x10` either way).
fn tile(scr: &mut Screen, img: &[u8], op: u8, si: &mut u16, undrawn: &mut usize) {
    if scr.mode == Video::Mcga {
        if op == 0x20 {
            mcg::clear(&mut scr.buf, *si);
        } else {
            mcg::blit(&mut scr.buf, img, op.wrapping_sub(0x41), *si);
        }
    } else {
        *undrawn += 1;
    }
    *si = si.wrapping_add(0x10);
}

fn byte(img: &[u8], i: usize) -> u8 {
    img.get(i).copied().unwrap_or(0)
}
