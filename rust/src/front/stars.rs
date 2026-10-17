//! `cs:0x8891`'s starfield body — the `cs:0x4EC4` table (12-byte
//! `{x, vx, _, y, vy, prev}` records) walked for the ending/title
//! backdrops. The asm table lives in code memory the port doesn't
//! mutate, so the host keeps its own: same move/respawn/mark
//! behavior, one `[0x1294]`-slot pixel per in-bounds star.
//!
//! Per star (file `0x38891`–`0x388D1`): out-of-bounds stars respawn
//! at the `0x9C/0x60` lane with `rand&3 − rand&3` velocity (rerolled
//! while still); every surviving star applies one `0x8881` move then
//! marks if it stayed on screen. The MCG `[0x1294]` mark (`0x2F59B`)
//! clears the star's *previous* pixel when it's still the star colour
//! and only sets the new one onto an empty cell — the dots move
//! without trails and never overwrite the credits text.

use crate::args::Video;
use crate::video::Screen;

/// Screen bounds the respawn tests (`x < 0x140`, `y < 0xC8`).
const W: u16 = 0x140;
const H: u16 = 0xC8;
/// The shipped table size — `cx` climbs to `0x63`.
const LEN: usize = 0x64;

/// One `cs:0x4EC4` star — position `(x, y)`, velocity `(vx, vy)`,
/// `prev` = the `[cs:si+8]` offset of the pixel it last marked.
#[derive(Clone, Copy)]
struct Star {
    x: u16,
    vx: i16,
    y: u16,
    vy: i16,
    prev: u16,
}

/// The host-side star table plus its respawn RNG.
pub struct Stars {
    tab: Box<[Star; LEN]>,
    rng: u32,
}

impl Stars {
    /// `0x64` stars, all spawned (the asm ships the table zeroed — every
    /// record reads out-of-bounds and respawns on the first walk).
    #[must_use]
    pub fn new(seed: u32) -> Self {
        let mut s = Self {
            tab: Box::new(
                [Star {
                    x: 0,
                    vx: 0,
                    y: 0,
                    vy: 0,
                    prev: 0,
                }; LEN],
            ),
            rng: seed | 1,
        };
        for i in 0..LEN {
            let star = s.spawn();
            s.tab[i] = star;
        }
        s
    }

    /// One `0x8891` walk over `cx + 1` stars — respawn out-of-bounds,
    /// `0x8881` move, `[0x1294]` mark the survivors. `plot = false`
    /// suppresses the mark — the `menu_text` crawl runs with the snow
    /// swept rather than let dots scatter over the glyphs (GAM-46).
    pub fn step(&mut self, cx: usize, scr: &mut Screen, ink: u8, plot: bool) {
        for i in 0..(cx + 1).min(LEN) {
            let mut s = self.tab[i];
            if s.x >= W || s.y >= H {
                let prev = s.prev; // respawn doesn't touch `[cs:si+8]`
                s = self.spawn();
                s.prev = prev;
            }
            s.x = s.x.wrapping_add(s.vx.cast_unsigned());
            s.y = s.y.wrapping_add(s.vy.cast_unsigned());
            if plot && s.x < W && s.y < H {
                mark(scr, &mut s, ink);
            }
            self.tab[i] = s;
        }
    }

    /// The `0x4E3C` init the crawl runs at page arm — wipe every live
    /// dot (the same `prev`-still-`dot` test `mark` uses) so the snow is
    /// gone before the text starts, in every mode (GAM-59 — the non-MCG
    /// early-return left the warm-up's dots over the whole crawl).
    pub fn clear(&mut self, scr: &mut Screen, ink: u8) {
        let dot = dot_ink(scr.mode, ink);
        for s in self.tab.iter_mut() {
            erase(scr, s.prev, dot);
            s.prev = 0;
        }
    }

    /// The `0x88DE` respawn — `(0x9C,0x60) ± 7` spawn lane, velocity in
    /// `−3..=3` rerolled while `vx == vy == 0` (file `0x388DE`–`0x38916`).
    fn spawn(&mut self) -> Star {
        let (x, y) = (0x9C + (self.next() & 7), 0x60 + (self.next() & 7));
        loop {
            let (vx, vy) = (self.dice(), self.dice());
            if vx != 0 || vy != 0 {
                return Star {
                    x,
                    vx,
                    y,
                    vy,
                    prev: 0,
                };
            }
        }
    }

    /// `rand & 3 − rand & 3` — the velocity roll.
    fn dice(&mut self) -> i16 {
        (self.next() & 3).cast_signed() - (self.next() & 3).cast_signed()
    }

    /// Cheap LCG standing in for `0xE93C` (visual fidelity only — the
    /// asm's exact star positions are RNG-sequence dependent anyway).
    fn next(&mut self) -> u16 {
        self.rng = self.rng.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
        u16::try_from(self.rng >> 16).unwrap_or(0)
    }
}

/// The `[0x1294]` mark. MCG (`0x2F59B`): clear `es:[prev]` when it's
/// still the `1` star colour, then set `es:[y·0x140 + x] = 1` only
/// when the cell is empty — moving dots, no trails, no text clobber.
/// The other modes' twins (`0x3061E`/`0x3061F`/`0x310CD`) run the same
/// logic in pixel space — `prev` is the packed `y·0x140 + x` pixel
/// index for every mode (identical to the MCG byte offset).
fn mark(scr: &mut Screen, s: &mut Star, ink: u8) {
    let dot = dot_ink(scr.mode, ink);
    erase(scr, s.prev, dot);
    let (x, y) = (usize::from(s.x), usize::from(s.y));
    s.prev = u16::try_from(y * usize::from(W) + x).unwrap_or(0);
    if scr.peek(x, y) == 0 {
        scr.put_pixel(x, y, dot);
    }
}

/// The star colour as `peek` reports it — the MCG `[0x1294]` writes a
/// literal `1`; the other modes plot the caller's `ink` clipped to the
/// packing (CGA 2bpp, EGA/TGA nibbles).
fn dot_ink(mode: Video, ink: u8) -> u8 {
    match mode {
        Video::Mcga => 1,
        Video::Cga => ink & 3,
        _ => ink & 0xF,
    }
}

/// Erase the `prev` pixel when it still holds the star colour —
/// `prev` is a `y·0x140 + x` pixel index (`0` = never plotted).
fn erase(scr: &mut Screen, prev: u16, dot: u8) {
    let (x, y) = (usize::from(prev) % 0x140, usize::from(prev) / 0x140);
    if prev != 0 && scr.peek(x, y) == dot {
        scr.put_pixel(x, y, 0);
    }
}
