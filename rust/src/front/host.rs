//! The frontend `VmHost` — replays the game's service calls against a
//! real [`Screen`], [`Palette`], asset set and the unpacked exe image
//! (for `cs:` reads), plus the [`Pump`] input cells.
//!
//! Decoded template slots (MCGA column of `dispatch_tables.txt`):
//!
//! - `[0x125A]` `0x2EB8A` → [`Screen::draw_image`] (the `Image` call) —
//!   all four mode variants (MCG linear, EGA plane streams, CGA/TGA
//!   banked) live in [`crate::video`].
//! - `[0x125C]` `0x2EB6F` → the `rep stosw` region fill — clears `es`.
//! - `[0x1258]`/`[0x1260]` `0x2EDBB`/`0x2EDA8` → `int 10/AX=1012` over
//!   all 256 regs from the `cs:0x1384` buffer (MCGA only — the other
//!   modes' slots are `int 10/AX=1002` palette writes and the CGA
//!   `AH=5` page select, no-ops under the fixed `Palette::{ega16,cga}`
//!   model).
//! - `[0x125E]` `0x2ED30`/`[0x128E]` `0x2F540` → `Palette::present`/
//!   `pal_fade` — the row-table + `ds:0x4095` composite with the
//!   `ds:[0x9CA1]` shade, dispatched via `svc_st` so the routine's
//!   `ds:` read resolves (GAM-41).
//! - `[0x1262]`/`[0x1268]` → the popup frame through
//!   [`Screen::put_pixel`] / the glyph put via `font::draw`, which
//!   replays the mode's packed/planar record merge byte-for-byte.
//! - `Call::Sound`/`Call::Chan`/`Call::Sfx` → [`Audio`] — the ported
//!   `int 0x80` driver over the synth bus; `window` renders it to a
//!   stream (GAM-31).
//! - `Call::Load` → [`Loader`] (`front/load.rs`) — the int-21 asset
//!   loaders' `ds:0x1208`+ cell → `{stem}{sfx}.{BIN,GPH}` swap
//!   (GAM-33).
//! - `Call::Text` → `front/text.rs` — the `menu_text` page walk;
//!   `Call::Stars` → `front/stars.rs` — the `cs:0x4EC4` starfield.
//! - `[0x1264]`/`[0x1266]` stay stubs — the refresh-list copies are
//!   presentation details the window's per-frame blit supersedes
//!   (GAM-7/GAM-9).
//!
//! `cs:` word reads map `+0x292B0` — the real code-segment base, not
//! the `file − 0x30000` label space the docs name functions with. The
//! `VmHost` impl and per-call bodies live in [`svc`].

mod svc;

use std::path::Path;

use crate::args::Sound;
use crate::assets::AssetSet;
use crate::palette::Palette;
use crate::video::Screen;

use super::audio::{Audio, BOOT_RATE};
use super::{load::Loader, map::Map, stars::Stars, Pump};

/// A live vsync hook — the frontend's stand-in for the async
/// int-9/int-33 ISRs plus the retrace the `0x2CA9` routine waits on.
/// `pump_input` runs it before each `ds:` mirror so blocking loops
/// (the screen `frame_loop`s, dialogs, `map_click`'s waits — all of
/// which never return to `window::run`'s present call) still see
/// fresh host input *and* present their framebuffer mid-step.
pub type VsyncSrc = Box<dyn FnMut(&mut Host)>;

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
    /// Unpacked `GAME.EXE` bytes — source for `cs_word` and the `ds:`
    /// font tables `font::draw` slices.
    pub img: Vec<u8>,
    /// The sound backend — `Call::Sound`/`Chan`/`Sfx` map onto the
    /// ported `int 0x80` driver; `window` renders `snd` to a stream.
    pub snd: Audio,
    /// Ink for the `[0x1262]` popup-frame outline.
    pub ink: u8,
    /// Calls dropped because the port doesn't model them yet.
    pub dropped: usize,
    /// `cs:0xF672` mirror — the `menu_text` stream cursor `Call::Text`
    /// resumes on.
    text_cur: u16,
    /// The host-side `cs:0x4EC4` star table `Call::Stars` walks.
    stars: Stars,
    /// The `[0x1278]` `map_draw` `ds:`-cell mirror (`[0x91D0]`/`[0x9168]`/
    /// `[0x9174]` + the record/route save cells).
    map: Map,
    /// `Call::Load`'s dir + set stem (`set_assets` installs it — the
    /// `ds:0x1208`+ cell → `{stem}{sfx}.{BIN,GPH}` mapping).
    loader: Option<Loader>,
    /// Live vsync hook — `window::run` installs one; `pump_input`
    /// runs it before each `ds:` mirror.
    vsync_src: Option<VsyncSrc>,
}

impl Host {
    /// A host for `mode` with the given assets/image/palette; `snd` is
    /// the `[0x155]` device selector (`S` arg).
    #[must_use]
    pub fn new(
        scr: Screen,
        pal: Palette,
        set: AssetSet,
        img: Vec<u8>,
        k_mode: bool,
        snd: Sound,
    ) -> Self {
        Self {
            scr,
            pal,
            pump: Pump::new(k_mode),
            set,
            snd: Audio::new(snd, &img, BOOT_RATE),
            img,
            ink: 0x0F,
            dropped: 0,
            text_cur: 0,
            stars: Stars::new(0x00C0_FFEE),
            map: Map::new(),
            loader: None,
            vsync_src: None,
        }
    }

    /// Point `Call::Load` at the asset dir with set stem `stem` — the
    /// endings' `*DTH`/`*WIN` and the restart's `*IN`/base pairs.
    pub fn set_assets(&mut self, dir: &Path, stem: &str) {
        self.loader = Some(Loader::new(dir.to_path_buf(), stem));
    }

    /// Install (or clear) the live vsync hook `pump_input` runs —
    /// the frontend's stand-in for the async int-9/int-33 ISRs and
    /// the retrace the `0x2CA9` wait models.
    pub fn set_vsync(&mut self, src: Option<VsyncSrc>) {
        self.vsync_src = src;
    }
}
