//! `Crawl` bodies — the typed [`stream`] block walk (`arm`/`pull`),
//! parametrized by the mode's [`lay`]. The per-frame scroll lives in
//! [`super::scroll`].
//!
//! GAM-80: the `0x2F629` stream decode is compiled into `stream::Blk`
//! tables — `tiles` are the literal op bytes (`0x20` clear, else glyph
//! `op − 0x41`), `brk` is the `0xF0` line-break arg.

use crate::args::Video;

use super::lay::{self, vrows};
use super::{cga, ega, mcg, stream, tga, Crawl};

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

    /// The block header (`fb15` held) then line records; `blk.end`
    /// latches `ended`, otherwise `cur` takes `blk.nxt`. Returns the
    /// visual rows `block` covers.
    fn walk(&mut self, img: &[u8], buf: &mut [u8], mode: Video) -> usize {
        let lay = lay::lay(mode);
        let mut si = lay.org;
        let Some(blk) = stream::at(self.cur) else {
            self.cur = 0;
            self.ended = true;
            return vrows(si, mode);
        };
        self.fb = u16::from(blk.fb);
        for l in blk.lines {
            si = si.wrapping_add(u16::from(l.xoff) >> lay.xsh);
            for &op in l.tiles {
                Self::tile(img, buf, op, &mut si, mode);
            }
            if let Some(arg) = l.brk {
                si = si
                    .wrapping_sub(u16::from(arg) >> lay.xsh)
                    .wrapping_add(lay.row_add)
                    ^ lay.bank_xor;
            }
        }
        if blk.end {
            self.cur = 0;
            self.ended = true;
        } else {
            self.cur = blk.nxt;
        }
        vrows(si, mode)
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
