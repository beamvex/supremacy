//! `Crawl` bodies — the stream walk (`arm`/`pull`/`line`/`tile`, the
//! `0x2F629` block decode into the staging `block`, parametrized by
//! the mode's [`lay`]). The per-frame scroll lives in
//! [`super::scroll`] (`step`/`emit` — the `[0x1290]`/`[0x1292]`/
//! `0x8983` stand-in, GAM-46/GAM-59).

use crate::args::Video;

use super::lay::{self, vrows};
use super::{cga, ega, mcg, tga, Crawl, STREAM};

/// How a block's op loop ended.
enum Flow {
    /// `0xF0` — new line armed, keep going.
    More,
    /// `0xFF` — suspend; the cursor stays on the next block header.
    Suspend,
    /// `0x00` — stream end (the `ended` latch).
    End,
}

impl Crawl {
    /// `Call::Text` with a fresh cursor — the `cs:0xF672` set plus the
    /// first block pull; `fill` is pre-marked consumed since the caller
    /// already painted `block` into the framebuffer in place.
    pub(super) fn arm(&mut self, img: &[u8], cur: u16, mode: Video) {
        self.cur = cur;
        self.armed = true;
        self.ended = false;
        self.drained = 0;
        self.pull(img, mode);
        self.fill = self.brows;
    }

    /// `0x2F656`+ — walk one block into `block` (cleared first, sized
    /// to the mode's VRAM span) and leave `fill` at 0 for `step` to
    /// feed a row at a time.
    pub(super) fn pull(&mut self, img: &[u8], mode: Video) {
        let mut buf = std::mem::take(&mut self.block);
        buf.resize(mode.vram_len(), 0);
        buf.fill(0);
        self.brows = self.walk(img, &mut buf, mode).min(lay::ROWS);
        self.block = buf;
        self.fill = 0;
    }

    /// The block header (`fb14` write-only, `fb15` held) then line
    /// records until `0xFF`/`0x00`; returns the visual rows `block`
    /// covers.
    fn walk(&mut self, img: &[u8], buf: &mut [u8], mode: Video) -> usize {
        let lay = lay::lay(mode);
        self.di = STREAM + usize::from(self.cur);
        self.fb = u16::from(byte(img, self.di + 1));
        self.di += 2;
        let mut si = lay.org;
        loop {
            si = si.wrapping_add(u16::from(byte(img, self.di) >> lay.xsh));
            self.di += 1;
            match self.line(img, buf, &mut si, mode) {
                Flow::More => continue,
                Flow::Suspend => {
                    self.cur = u16::try_from(self.di - STREAM).unwrap_or(0);
                }
                Flow::End => {
                    self.cur = 0;
                    self.ended = true;
                }
            }
            return vrows(si, mode);
        }
    }

    /// The inner op loop — `More` on the `0xF0` newline,
    /// `Suspend`/`End` on the terminators; `si` keeps the asm's u16
    /// wrap.
    fn line(&mut self, img: &[u8], buf: &mut [u8], si: &mut u16, mode: Video) -> Flow {
        let lay = lay::lay(mode);
        loop {
            let op = byte(img, self.di);
            self.di += 1;
            match op {
                0x00 => return Flow::End,
                0xFF => return Flow::Suspend,
                0xF0 => {
                    let arg = u16::from(byte(img, self.di)) >> lay.xsh;
                    self.di += 1;
                    *si = si.wrapping_sub(arg).wrapping_add(lay.row_add) ^ lay.bank_xor;
                    return Flow::More;
                }
                _ => Self::tile(img, buf, op, si, mode),
            }
        }
    }

    /// One tile op — `0x20` clears, anything else blits `op − 0x41`;
    /// `si` advances the mode's tile stride either way.
    fn tile(img: &[u8], buf: &mut [u8], op: u8, si: &mut u16, mode: Video) {
        if op == 0x20 {
            clear(buf, *si, mode);
        } else {
            blit(buf, img, op.wrapping_sub(0x41), *si, mode);
        }
        *si = si.wrapping_add(lay::lay(mode).tile);
    }
}

/// The mode's `0x20` space clear.
fn clear(buf: &mut [u8], si: u16, mode: Video) {
    match mode {
        Video::Mcga => mcg::clear(buf, si),
        Video::Ega => ega::clear(buf, si),
        Video::Cga => cga::clear(buf, si),
        Video::Tga => tga::clear(buf, si),
    }
}

/// The mode's glyph blit.
fn blit(buf: &mut [u8], img: &[u8], idx: u8, si: u16, mode: Video) {
    match mode {
        Video::Mcga => mcg::blit(buf, img, idx, si),
        Video::Ega => ega::blit(buf, img, idx, si),
        Video::Cga => cga::blit(buf, img, idx, si),
        Video::Tga => tga::blit(buf, img, idx, si),
    }
}

fn byte(img: &[u8], i: usize) -> u8 {
    img.get(i).copied().unwrap_or(0)
}
