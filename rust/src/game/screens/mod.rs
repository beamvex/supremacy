//! Menu-screen entries — the hotspot actions that `jmp` into one of
//! the per-mode frame loops (`uimode` 1–6 plus the galaxy screen).
//!
//! Every action starts with `pop ax` — the menu hit-test `jmp`s here,
//! so the slot under the stack top is the caller's return address and
//! the action abandons it instead of `ret`ing (the port just returns
//! into the enclosing `menu::service`). Most come in pairs `N`/`N+3`
//! — entry at `N` skips the `call word near [0x1274]` window slot the
//! `N+3` variant runs. Each routine does its screen setup, runs the
//! screen's inline `{vsync, …, menu}` loop until `[0x9CDA] & 2`, then
//! `jmp 0x2CC5`s — [`shell::shell_enter`] — back to the menu screen.
//!
//! The galaxy entry `cs:0x3906` is the exception: its loop is the
//! port's `Phase::Galaxy`, keyed off `uimode == 5` by the runner.

mod build;
mod detail;
mod fleet;
mod galaxy;
mod machd;
mod misc;
mod orders;
mod status;
mod surface;

pub use build::enter_build;
pub use detail::{enter_detail, enter_detail_sel, enter_detail_win};
pub use fleet::enter_fleet;
pub use galaxy::enter_galaxy;
pub use machd::enter_detail_mach;
pub use misc::{cell_pick, map_click, mode_pick, report, restart, snd_toggle, stub};
pub use orders::enter_orders;
pub use status::{enter_status, enter_status_win};
pub use surface::enter_surface;

use super::consts::{FRAME, HOT_COUNT, HOT_LIST, HOT_SEL, SND_ON};
use super::vhost::Call;
use super::vops::Ctx;
use super::{selrec, shell, UIMODE};

/// The screen-entry half of [`super::dispatch_action`] — the
/// per-mode entries in the same two-window pair layout (the `+3`
/// variant opens with the `0x1274` window slot first).
pub fn dispatch(c: &mut Ctx, t: u16) {
    match t {
        0x3906 | 0xA656 => enter_galaxy(c, false),
        0x3909 | 0xA659 => enter_galaxy(c, true),
        0x2426 | 0x9176 => enter_detail_sel(c),
        0x2429 | 0x9179 => enter_detail(c),
        0x247F | 0x91CF => enter_detail_win(c),
        0x31F2 | 0x9F42 => enter_orders(c, false),
        0x31F5 | 0x9F45 => enter_orders(c, true),
        0x4BA7 | 0xB8F7 => enter_build(c, false),
        0x4BAA | 0xB8FA => enter_build(c, true),
        0x5F12 | 0xCC62 => enter_fleet(c, false),
        0x5F15 | 0xCC65 => enter_fleet(c, true),
        0x5867 | 0xC5B7 => enter_status(c),
        0x587A | 0xC5CA => enter_status_win(c),
        _ => misc(c, t),
    }
}

/// The standalone actions and the record-list nav — the
/// `Call::Native` miss lives here.
fn misc(c: &mut Ctx, t: u16) {
    match t {
        0x23C2 | 0x9112 => map_click(c),
        0x2D97 | 0x9AE7 => report(c),
        0x4036 | 0xAD86 => enter_surface(c),
        0x8427 | 0x8430 | 0xF177 | 0xF180 => restart(c),
        0x854A | 0xF29A => snd_toggle(c),
        0x8544 | 0xF294 => stub(c),
        0x8539 | 0xF289 => cell_pick(c),
        0x80D2 | 0xEE22 => mode_pick(c, 1),
        0x80D8 | 0xEE28 => mode_pick(c, 2),
        0x80DE | 0xEE2E => mode_pick(c, 3),
        0x80E4 | 0xEE34 => mode_pick(c, 4),
        0x80EA | 0xEE3A => mode_pick(c, 5),
        _ => nav(c, t),
    }
}

/// The selection/record nav arms.
fn nav(c: &mut Ctx, t: u16) {
    match t {
        0x243D | 0x918D => enter_detail_mach(c),
        0x5DA1 | 0xCAF1 => selrec::sel_prev(c),
        0x5DD3 | 0xCB23 => selrec::sel_next(c),
        0x5E5A | 0xCBAA => selrec::sel_faction(c),
        _ => drop(c.host.svc(Call::Native(t))),
    }
}

/// `u16` casts for the `ds:` slot constants.
macro_rules! a {
    ($k:expr) => {
        u16::try_from($k).unwrap_or(0)
    };
}

/// Shared hotspot-list install: `[0x9CC8]=[sel]`, `[0x9194]=list`,
/// `[0x91A8]=n`, `uimode = mode`.
fn install(c: &mut Ctx, sel: u16, list: u16, n: u16, mode: u8) {
    let s = c.vm.r16(c.st, sel);
    c.vm.w16(c.st, a!(HOT_SEL), s);
    c.vm.w16(c.st, a!(HOT_LIST), list);
    c.vm.w16(c.st, a!(HOT_COUNT), n);
    c.vm.w8(c.st, a!(UIMODE), mode);
}

/// The shared sound-select block — `Chan(0x8A5B)`, `Chan(0x8A3B,al)`,
/// `Sound(cl)`, suppressed on `[0x91D3] == 0xFF` (the `0x2CC5` idiom).
fn snd(c: &mut Ctx, al: u8, cl: u8) {
    if c.vm.r8(c.st, a!(SND_ON)) == 0xFF {
        return;
    }
    c.host.svc(Call::Chan(0x8A5B, 0));
    c.host.svc(Call::Chan(0x8A3B, al));
    c.host.svc(Call::Sound(cl));
}

/// `inc [0x91D4]` — the per-loop frame counter.
fn bump(c: &mut Ctx) {
    let f = c.vm.r8(c.st, a!(FRAME)).wrapping_add(1);
    c.vm.w8(c.st, a!(FRAME), f);
}

/// The shared screen exit — stash `[0x9CC8]` at `sel`, then the
/// `jmp 0x2CC5` back to the shell.
fn leave(c: &mut Ctx, sel: u16) {
    let s = c.vm.r16(c.st, a!(HOT_SEL));
    c.vm.w16(c.st, sel, s);
    shell::shell_enter(c);
}
