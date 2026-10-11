//! `cs:0x8891`'s starfield body — the `cs:0x4EC4` table (12-byte
//! `{x, vx, _, y, vy, _}` records) walked for the ending/title
//! backdrops. The asm table lives in code memory the port doesn't
//! mutate, so the host keeps its own: same move/respawn/mark
//! behavior, one `[0x1294]`-slot pixel per in-bounds star.
//!
//! Per star (file `0x38891`–`0x388D1`): out-of-bounds stars respawn
//! at the `0x9C/0x60` lane with `rand&3 − rand&3` velocity (rerolled
//! while still); every surviving star applies one `0x8881` move then
//! marks if it stayed on screen.

/// Screen bounds the respawn tests (`x < 0x140`, `y < 0xC8`).
const W: u16 = 0x140;
const H: u16 = 0xC8;
/// The shipped table size — `cx` climbs to `0x63`.
const LEN: usize = 0x64;

/// One `cs:0x4EC4` star — position `(x, y)`, velocity `(vx, vy)`.
#[derive(Clone, Copy)]
struct Star {
    x: u16,
    vx: i16,
    y: u16,
    vy: i16,
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
    /// `0x8881` move, `put` the survivors.
    pub fn step(&mut self, cx: usize, put: &mut dyn FnMut(usize, usize)) {
        for i in 0..(cx + 1).min(LEN) {
            let mut s = self.tab[i];
            if s.x >= W || s.y >= H {
                s = self.spawn();
            }
            s.x = s.x.wrapping_add(s.vx.cast_unsigned());
            s.y = s.y.wrapping_add(s.vy.cast_unsigned());
            self.tab[i] = s;
            if s.x < W && s.y < H {
                put(usize::from(s.x), usize::from(s.y));
            }
        }
    }

    /// The `0x88DE` respawn — `(0x9C,0x60) ± 7` spawn lane, velocity in
    /// `−3..=3` rerolled while `vx == vy == 0` (file `0x388DE`–`0x38916`).
    fn spawn(&mut self) -> Star {
        let (x, y) = (0x9C + (self.next() & 7), 0x60 + (self.next() & 7));
        loop {
            let (vx, vy) = (self.dice(), self.dice());
            if vx != 0 || vy != 0 {
                return Star { x, vx, y, vy };
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
