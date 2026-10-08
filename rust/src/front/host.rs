//! The frontend `VmHost` — replays the game's service calls against a
//! real [`Screen`], [`Palette`], asset set and the unpacked exe image
//! (for `cs:` reads), plus the [`Pump`] input cells.
//!
//! Decoded template slots (MCGA column of `dispatch_tables.txt`):
//!
//! - `[0x125A]` `0x2EB8A` → [`Screen::draw_image`] (the `Image` call).
//! - `[0x125C]` `0x2EB6F` → the `rep stosw` region fill — clears `es`.
//! - `[0x1260]` `0x2EDA8` → `int 10/AX=1012` DAC reg 0 from `cs:0x1384`.
//! - `[0x1268]` `0x2EECE` → glyph put — the debug [`font`].
//! - `[0x1264]`/`[0x125E]`/`[0x1266]` and the sound calls stay stubs —
//!   the refresh-list copy and palette flushes are presentation details
//!   the window's per-frame blit supersedes (GAM-7/GAM-9).
//!
//! `cs:` word reads map `+0x30000` like the rest of the port.

use crate::assets::AssetSet;
use crate::game::{Call, VmHost};
use crate::palette::Palette;
use crate::video::Screen;

use super::{font, Pump};

/// `cs:` → file offset for the game-code segment (`0x5C1E` region).
const CS_OFS: usize = 0x30000;
/// `cs:` → file offset for the MCGA video-template segment —
/// `cs:0x58DA` = file `0x2EB8A`, `cs:0x5C1E` = file `0x2EECE`
/// (`dispatch_tables.txt`); template `cs:` refs like the `0x1384` DAC
/// triplet live in this space, not the game-code one.
const TPL_OFS: usize = 0x292B0;
/// `cs:0x1384` in template space — the 3-byte DAC triplet `0x2EDA8`
/// programs reg 0 with (file `0x2A634`; ships zeroed = black).
const DAC0: usize = 0x1384 + TPL_OFS;

/// The concrete host — owns the visible framebuffer and input state.
pub struct Host {
    /// Mode-laid-out VRAM the calls draw into.
    pub scr: Screen,
    /// Active 256-colour palette (DAC reg 0 reprogrammed on `0x1260`).
    pub pal: Palette,
    /// Cursor/button/key state mirrored into `ds:` each frame.
    pub pump: Pump,
    /// The decoded image set `[0x125A]` indexes.
    pub set: AssetSet,
    /// Unpacked `GAME.EXE` bytes — source for `cs_word`.
    pub img: Vec<u8>,
    /// Text colour for the debug font.
    pub ink: u8,
    /// Calls dropped because the port doesn't model them yet.
    pub dropped: usize,
}

impl Host {
    /// A host for `mode` with the given assets/image/palette.
    #[must_use]
    pub fn new(scr: Screen, pal: Palette, set: AssetSet, img: Vec<u8>, k_mode: bool) -> Self {
        Self {
            scr,
            pal,
            pump: Pump::new(k_mode),
            set,
            img,
            ink: 0x0F,
            dropped: 0,
        }
    }

    /// The `[0x1262]` popup frame — `(ax,bx)`/`(cx,dx)` are the opposing
    /// corners in `(4px-col, row)` units, order unspecified; normalised
    /// and drawn as a 1px outline (approximation of `0x2EDCE`).
    fn rect(&mut self, ax: u16, bx: u16, cx: u16, dx: u16) {
        if self.scr.mode != crate::args::Video::Mcga {
            return;
        }
        let (l, r) = (ax.min(cx).min(79) * 4, ax.max(cx).min(79) * 4 + 3);
        let (t, bot) = (bx.min(dx).min(199), bx.max(dx).min(199));
        for x in usize::from(l)..=usize::from(r) {
            self.scr.buf[usize::from(t) * 320 + x] = self.ink;
            self.scr.buf[usize::from(bot) * 320 + x] = self.ink;
        }
        for y in usize::from(t)..=usize::from(bot) {
            self.scr.buf[y * 320 + usize::from(l)] = self.ink;
            self.scr.buf[y * 320 + usize::from(r)] = self.ink;
        }
    }

    /// `int 33` — `ax = 4` sets the position, `ax = 8` the y range.
    fn int33(&mut self, ax: u16, cx: u16, dx: u16) {
        match ax {
            4 => self.pump.move_to(cx, dx),
            8 => self.pump.set_range(cx, dx),
            _ => self.dropped += 1,
        }
    }

    /// The `0x2EDA8` DAC write — template `cs:0x1384` triplet, 6-bit→8.
    fn dac0(&mut self) {
        let c = self.img.get(DAC0..DAC0 + 3).unwrap_or(&[]);
        for (k, &v) in c.iter().enumerate() {
            self.pal.colors[0][k] = u8::try_from(u16::from(v) * 255 / 63).unwrap_or(0xFF);
        }
    }
}

impl VmHost for Host {
    fn svc(&mut self, call: Call) -> u16 {
        match call {
            Call::Image(i) => self.scr.draw_image(&self.set, usize::from(i)),
            Call::Glyph(ch, bx, dx) => {
                font::draw(&mut self.scr, ch, bx.wrapping_mul(4), dx, self.ink);
            }
            Call::Rect(a, b, x, d) => self.rect(a, b, x, d),
            Call::Mouse(ax, cx, dx) => self.int33(ax, cx, dx),
            Call::Slot(0x125C, _) => self.scr.buf.fill(0),
            Call::Slot(0x1260, _) => self.dac0(),
            Call::Menu
            | Call::Dialog
            | Call::Refresh
            | Call::Present(_)
            | Call::Flash
            | Call::PlanetPanel
            | Call::Blit(..)
            | Call::Screen(_)
            | Call::Slot(..)
            | Call::Native(_)
            | Call::Ui(_)
            | Call::Draw(..)
            | Call::Sound(_)
            | Call::Chan(..)
            | Call::Sfx(..) => self.dropped += 1,
        }
        0
    }

    fn cs_word(&mut self, ofs: u16) -> u16 {
        let i = CS_OFS + usize::from(ofs);
        u16::from_le_bytes([
            self.img.get(i).copied().unwrap_or(0),
            self.img.get(i + 1).copied().unwrap_or(0),
        ])
    }
}
