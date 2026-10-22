//! `menu_text` (`cs:0x2F629`, file `0x2F629`–`0x2F6C9`) — the
//! text-stream renderer the title attract and the `0xFA` endings call
//! through `[0x1296]`, plus the scroll the port runs for them.
//!
//! The stream lives in the image's stack region — `ss = 0x1F6F`
//! (EXEPACK `real_ss`), so `ss:di` = file `0x202F0 + di`. Each block
//! is a `{fb14, fb15}` header then line records `{xoff, ops…}`: `0x00`
//! ends the stream, `0xFF` suspends with the cursor left on the next
//! block header, `0xF0`+arg starts a new line (`si −= arg`,
//! `si += row_add`), `0x20` runs the tile clear and any other
//! byte is a 16×24 glyph tile blitted from the mode's font.
//!
//! All four variants are ported — the MCG `0x2F629` and the
//! EGA/CGA/TGA twins (`0x30520`/`0x30FFE`/`0x31CAA`) share the stream
//! format but each has its own `si` space, xoff shift, tile stride
//! and glyph table ([`lay`]); the painters live in [`mcg`], [`ega`],
//! [`cga`] and [`tga`].
//!
//! The shipped binary *pages* — its `[0x1290]`/`[0x1292]` scroll slots
//! and `cs:0x8983` are `ret`s in every video mode, so each block
//! repaints in place over the previous one (GAM-46's "prints over
//! itself"). The port instead runs the sequence as a continuous
//! upward crawl — [`Crawl`] — like the non-DOS originals: a fresh
//! `Call::Text` arm renders the first block into the framebuffer at
//! once (the asm's in-place paint) and sweeps the starfield, each
//! `Call::Scroll` shifts the VRAM image up and feeds the queued
//! block's rows in at the bottom (mode-native — [`scroll`]), and
//! `u16::MAX` `Call::Text` calls pull the next block once the current
//! one has fed in. `done` (`ax |= 0x100`) latches once the `0x00`
//! terminator has fed a full screen of trailing blank — every block
//! is visible en route (GAM-59: previously the non-MCG modes consumed
//! the pacing but never drew, so the text was invisible).

use crate::video::Screen;

mod cga;
mod ega;
mod lay;
mod mcg;
mod scroll;
mod tga;
mod walk;

/// `ss:` stream base in the unpacked image — `ss = 0x1F6F`, file
/// `0x1F6F0 + 0xC00`.
const STREAM: usize = 0x202F0;

/// The `menu_text` crawl — the `cs:0xF672` cursor plus the scroll
/// state `[0x1290]`/`[0x1292]`/`0x8983` stand in for.
pub struct Crawl {
    /// `cs:0xF672` mirror — the stream cursor `u16::MAX` calls resume.
    cur: u16,
    /// `di` mid-pull — the stream op being walked.
    di: usize,
    /// `cs:0xFB15` of the last block pulled (the hold/scroll variant).
    fb: u16,
    /// A `Call::Text` cursor is live until `reset` (`[0x125C]`).
    armed: bool,
    /// The `0x00` terminator ran — only the blank tail is left to feed.
    ended: bool,
    /// The pending block rendered in the mode's `si`/`buf` space —
    /// its rows feed in at the framebuffer bottom while the crawl runs.
    block: Vec<u8>,
    /// Visual rows `block` covers.
    brows: usize,
    /// `block` rows already fed in at the bottom.
    fill: usize,
    /// Blank tail rows fed since `ended` — `done` at a screen height.
    drained: usize,
}

impl Crawl {
    /// Disarmed — nothing painted, nothing scrolling.
    #[must_use]
    pub fn new() -> Self {
        Self {
            cur: 0,
            di: 0,
            fb: 0,
            armed: false,
            ended: false,
            block: Vec::new(),
            brows: 0,
            fill: 0,
            drained: 0,
        }
    }

    /// `[0x125C]` screen blank — kill any live crawl.
    pub fn reset(&mut self) {
        *self = Self::new();
    }

    /// `Call::Text` — a fresh `arg` arms the sequence (the first block
    /// paints into `scr` at once, the asm's in-place paint); `u16::MAX`
    /// pulls the next block once the current one has fed in. Returns
    /// `ax` = `fb15 | done << 8` — `done` is the `0x9CCB` stream end
    /// once the trailing blank has scrolled a full screen.
    pub fn call(&mut self, scr: &mut Screen, img: &[u8], arg: u16) -> u16 {
        if arg != u16::MAX {
            self.arm(img, arg, scr.mode);
            scroll::paint(scr, &self.block, self.brows);
        } else if self.armed && !self.ended && self.fill >= self.brows {
            self.pull(img, scr.mode);
        }
        self.fb | u16::from(self.done()) << 8
    }

    /// The `0x9CCB` end report — stream ended and a screen-height tail
    /// of blank rows has fed since.
    fn done(&self) -> bool {
        self.ended && self.drained >= lay::ROWS
    }

    /// High while a `Call::Text` cursor is live (arm → the `[0x125C]`
    /// reset) — star marks stay suppressed for the crawl's whole
    /// length, so the snow stays swept from the arm on rather than
    /// only once a scroll step ran (GAM-60: the gap let `Call::Stars`
    /// replot before the first step, and pages that arm but never
    /// scroll — ending A's hold, the post-seq `0x58F2` input wait —
    /// replotted it for their duration).
    #[must_use]
    pub fn sweeping(&self) -> bool {
        self.armed
    }
}

impl Default for Crawl {
    fn default() -> Self {
        Self::new()
    }
}
