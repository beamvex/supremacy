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
    /// dot (the same `prev`-is-`1` test `0x2F59B` uses) so the snow is
    /// gone before the text starts.
    pub fn clear(&mut self, scr: &mut Screen) {
        if scr.mode != Video::Mcga {
            return;
        }
        for s in self.tab.iter_mut() {
            if s.prev != 0 && scr.buf.get(usize::from(s.prev)) == Some(&1) {
                scr.buf[usize::from(s.prev)] = 0;
            }
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
/// The other modes' twins (`0x3061E`/`0x3061F`/`0x310CD`) keep the
/// `put_pixel` approximation.
fn mark(scr: &mut Screen, s: &mut Star, ink: u8) {
    if scr.mode != Video::Mcga {
        scr.put_pixel(usize::from(s.x), usize::from(s.y), ink);
        return;
    }
    if scr.buf.get(usize::from(s.prev)) == Some(&1) {
        scr.buf[usize::from(s.prev)] = 0;
    }
    let di = (usize::from(s.y) * 0x140 + usize::from(s.x)) & 0xFFFF;
    s.prev = u16::try_from(di).unwrap_or(0);
    if scr.buf.get(di) == Some(&0) {
        scr.buf[di] = 1;
    }
}
