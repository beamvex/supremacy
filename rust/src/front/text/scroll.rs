//! `Crawl`'s per-frame scroll — `step` shifts `scr.buf` up `px`
//! visual rows and feeds the pending `block` in at the bottom; past
//! the `0x00` terminator the blank tail counts toward `done`. All
//! work is in the mode's own `buf` space so the staged blocks feed
//! verbatim (GAM-59 — the EGA/CGA/TGA variants the GAM-46 port only
//! paced).

use crate::args::Video;
use crate::video::Screen;

use super::lay::{rofs, ROWS as H};
use super::Crawl;

/// Bank count and bytes per bank-line for the interleaved modes.
fn banks(mode: Video) -> (usize, usize) {
    match mode {
        Video::Cga => (2, 0x50),
        _ => (4, 0xA0),
    }
}

impl Crawl {
    /// `Call::Scroll` — shift `scr` up `px` rows (`shift`) then feed
    /// the bottom `px` rows from `block` (`emit`). The pending
    /// block's `fill` cursor feeds its rows top-down.
    pub fn step(&mut self, scr: &mut Screen, px: usize) {
        if !self.armed {
            return;
        }
        shift(scr, px);
        for y in (H - px)..H {
            self.emit(scr, y);
        }
    }

    /// Feed visual row `fill` of `block` into `buf`'s visual row `y` —
    /// or a blank row once it ran out; the `ended` tail rows count
    /// toward `done`.
    fn emit(&mut self, scr: &mut Screen, y: usize) {
        if self.fill < self.brows {
            feed(&mut scr.buf, &self.block, scr.mode, y, self.fill);
            self.fill += 1;
        } else {
            blank(&mut scr.buf, scr.mode, y);
            self.drained += usize::from(self.ended);
        }
    }
}

/// Shift `scr.buf` up `px` visual rows in the mode's own space.
fn shift(scr: &mut Screen, px: usize) {
    match scr.mode {
        Video::Mcga => scr.buf.copy_within(px * 0x140.., 0),
        Video::Ega => shift_planes(&mut scr.buf, px),
        _ => shift_banks(&mut scr.buf, px, scr.mode),
    }
}

/// EGA — the same `px`-row shift inside each `0x1F40` plane.
fn shift_planes(buf: &mut [u8], px: usize) {
    for p in 0..4usize {
        let r = p * 0x1F40;
        buf.copy_within(r + px * 0x28..r + 0x1F40, r);
    }
}

/// CGA/TGA — visual row `y` lives at `(y % b)·0x2000 + (y / b)·pitch`;
/// scroll by copying each destination row from `y + px` (a clone —
/// the banks interleave, so rows cross regions when `px % b ≠ 0`).
fn shift_banks(buf: &mut [u8], px: usize, mode: Video) {
    let (b, pitch) = banks(mode);
    let old = buf.to_vec();
    for y in 0..H - px {
        let (s, d) = (rofs(y + px, b, pitch), rofs(y, b, pitch));
        buf[d..d + pitch].copy_from_slice(&old[s..s + pitch]);
    }
}

/// Copy `block`'s visual row `src` into `buf`'s visual row `dst` — a
/// line, four plane rows or one bank-line by mode.
fn feed(buf: &mut [u8], blk: &[u8], mode: Video, dst: usize, src: usize) {
    match mode {
        Video::Mcga => {
            let (s, d) = (src * 0x140, dst * 0x140);
            buf[d..d + 0x140].copy_from_slice(&blk[s..s + 0x140]);
        }
        Video::Ega => {
            for p in 0..4usize {
                let (s, d) = (p * 0x1F40 + src * 0x28, p * 0x1F40 + dst * 0x28);
                buf[d..d + 0x28].copy_from_slice(&blk[s..s + 0x28]);
            }
        }
        _ => {
            let (b, pitch) = banks(mode);
            let (s, d) = (rofs(src, b, pitch), rofs(dst, b, pitch));
            buf[d..d + pitch].copy_from_slice(&blk[s..s + pitch]);
        }
    }
}

/// Clear visual row `y` in `buf` — the ran-out feed.
fn blank(buf: &mut [u8], mode: Video, y: usize) {
    match mode {
        Video::Mcga => buf[y * 0x140..(y + 1) * 0x140].fill(0),
        Video::Ega => {
            for p in 0..4usize {
                buf[p * 0x1F40 + y * 0x28..p * 0x1F40 + (y + 1) * 0x28].fill(0);
            }
        }
        _ => {
            let (b, pitch) = banks(mode);
            let d = rofs(y, b, pitch);
            buf[d..d + pitch].fill(0);
        }
    }
}

/// The arm's in-place paint — copy `block`'s first `brows` visual
/// rows into `scr.buf` (per region for the planar/banked modes).
pub(super) fn paint(scr: &mut Screen, blk: &[u8], brows: usize) {
    match scr.mode {
        Video::Mcga => {
            let n = brows * 0x140;
            scr.buf[..n].copy_from_slice(&blk[..n]);
        }
        Video::Ega => {
            let n = (brows * 0x28).min(0x1F40);
            for p in 0..4usize {
                let r = p * 0x1F40;
                scr.buf[r..r + n].copy_from_slice(&blk[r..r + n]);
            }
        }
        mode => {
            let (b, pitch) = banks(mode);
            let n = (brows.div_ceil(b) * pitch).min(0x2000);
            for r in 0..b {
                let o = r * 0x2000;
                scr.buf[o..o + n].copy_from_slice(&blk[o..o + n]);
            }
        }
    }
}
