//! `Crawl` bodies — the stream walk (`arm`/`pull`/`line`/`tile`, the
//! `0x2F629` page decode into the staging `block`) and the per-frame
//! scroll (`step`/`emit`/`consume` — the `[0x1290]`/`[0x1292]`/`0x8983`
//! stand-in, GAM-46).

use crate::args::Video;
use crate::video::Screen;

use super::{mcg, Crawl, H, ORG, STREAM, W};

/// `0x18` — the 16×24 tile row span (`0x2F4E3`/`0x2F519`).
const TILE_H: usize = 0x18;

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
        self.scrolled = false;
        self.drained = 0;
        self.pull(img, mode);
        self.fill = self.brows;
    }

    /// `0x2F656`+ — walk one block into `block` (cleared first) and
    /// leave `fill` at 0 for `step` to feed a row at a time.
    pub(super) fn pull(&mut self, img: &[u8], mode: Video) {
        let mut buf = std::mem::take(&mut self.block);
        buf.fill(0);
        self.brows = self.walk(img, &mut buf, mode).min(H);
        self.block = buf;
        self.fill = 0;
    }

    /// The block header (`fb14` write-only, `fb15` held) then line
    /// records until `0xFF`/`0x00`; returns the rows `block` covers.
    fn walk(&mut self, img: &[u8], buf: &mut [u8], mode: Video) -> usize {
        self.di = STREAM + usize::from(self.cur);
        self.fb = u16::from(byte(img, self.di + 1));
        self.di += 2;
        let mut si = ORG;
        loop {
            si = si.wrapping_add(u16::from(byte(img, self.di)));
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
            return usize::from(si) / W + TILE_H;
        }
    }

    /// The inner op loop (file `0x2F64C`–`0x2F691`) — `More` on the
    /// `0xF0` newline, `Suspend`/`End` on the terminators; `si` keeps
    /// the asm's u16 wrap.
    fn line(&mut self, img: &[u8], buf: &mut [u8], si: &mut u16, mode: Video) -> Flow {
        loop {
            let op = byte(img, self.di);
            self.di += 1;
            match op {
                0x00 => return Flow::End,
                0xFF => return Flow::Suspend,
                0xF0 => {
                    *si = si
                        .wrapping_sub(u16::from(byte(img, self.di)))
                        .wrapping_add(0x1F40);
                    self.di += 1;
                    return Flow::More;
                }
                _ => self.tile(img, buf, op, si, mode),
            }
        }
    }

    /// One tile op — `0x20` clears, anything else blits `op − 0x41`;
    /// non-MCG modes just count the op (`si += 0x10` either way).
    fn tile(&mut self, img: &[u8], buf: &mut [u8], op: u8, si: &mut u16, mode: Video) {
        if mode == Video::Mcga {
            if op == 0x20 {
                mcg::clear(buf, *si);
            } else {
                mcg::blit(buf, img, op.wrapping_sub(0x41), *si);
            }
        } else {
            self.undrawn += 1;
        }
        *si = si.wrapping_add(0x10);
    }

    /// `Call::Scroll` — shift the framebuffer up `px` rows and feed
    /// `block` rows in at the bottom (`emit`); past the `0x00`
    /// terminator the blank tail counts toward `done`. Non-MCG modes
    /// consume rows for the same pacing (their tiles stay unported).
    pub fn step(&mut self, scr: &mut Screen, px: usize) {
        if !self.armed {
            return;
        }
        if scr.mode == Video::Mcga {
            self.scrolled = true;
            scr.buf.copy_within(px * W.., 0);
            for y in (H - px)..H {
                self.emit(&mut scr.buf[y * W..(y + 1) * W]);
            }
        } else {
            for _ in 0..px {
                self.consume();
            }
        }
    }

    /// Feed one row — the next `block` row, or a cleared row once it
    /// ran out; the `ended` tail rows count toward `done`.
    fn emit(&mut self, row: &mut [u8]) {
        if self.fill < self.brows {
            let s = self.fill * W;
            row.copy_from_slice(&self.block[s..s + W]);
            self.fill += 1;
        } else {
            row.fill(0);
            self.drained += usize::from(self.ended);
        }
    }

    /// The non-MCG `emit` — advance `fill`/`drained` without a blit.
    fn consume(&mut self) {
        if self.fill < self.brows {
            self.fill += 1;
        } else {
            self.drained += usize::from(self.ended);
        }
    }
}

fn byte(img: &[u8], i: usize) -> u8 {
    img.get(i).copied().unwrap_or(0)
}
