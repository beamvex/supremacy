//! The galaxy-screen menu entry — `cs:0x3906`/`0x3909` (file
//! `0x33906`–`0x3395A`) and its sidebar view-step actions
//! `cs:0x6648`/`0x6693` (records `0xD398`/`0xD3E3`).

use super::super::consts::{ARMED, PANEL_ON, PANEL_REQ, REDRAW, SEL_REC, TYPE_SEL};
use super::super::dialog;
use super::super::vhost::Call;
use super::super::vops::{faction_rec, Ctx};
use super::{install, snd};

/// `u16` casts for the `ds:` slot constants.
macro_rules! a {
    ($k:expr) => {
        u16::try_from($k).unwrap_or(0)
    };
}

/// `cs:0x3906` — enter the galaxy loop: the `0x2CC5`-pattern sound
/// block, the faction record into `[0x917C]`, `uimode 5`, the `0xA81C`
/// 3-record hotspot install and the redraw flags — then `0x395B`,
/// which the port runs as `Phase::Galaxy` (the runner keys the
/// transition off `uimode`). The `0x3909` variant (`win`) first runs
/// the `[0x1274]` window slot; both abandon the menu caller's frame
/// with the leading `pop ax`.
pub fn enter_galaxy(c: &mut Ctx, win: bool) {
    if win {
        c.host.svc(Call::Slot(0x1274, 0));
    }
    dialog::snd_block(c);
    c.vm.w16(c.st, a!(SEL_REC), faction_rec(c));
    c.vm.w8(c.st, a!(ARMED), 0);
    install(c, 0xA81A, 0xA81C, 3, 5);
    c.vm.w8(c.st, a!(REDRAW), 0xFF);
    c.vm.w8(c.st, a!(PANEL_REQ), 0xFF);
    c.vm.w8(c.st, a!(PANEL_ON), 0);
}

/// `cs:0x6648` — the "previous type" sidebar action (file
/// `0x36648`–`0x36692`): the `al=3`/`cl=0xA` chirp, then
/// `[0x91ED]−1` wrapping `0 → 6` and re-stepping past `1`.
pub fn view_prev(c: &mut Ctx) {
    snd(c, 3, 0xA);
    let s = c.vm.r8(c.st, a!(TYPE_SEL));
    let n = if s == 0 {
        6
    } else {
        let n = s.wrapping_sub(1);
        if n == 1 {
            0
        } else {
            n
        }
    };
    stepped(c, n);
}

/// `cs:0x6693` — the "next type" sidebar action (file
/// `0x36693`–`0x366DD`): the same chirp, `[0x91ED]+1` wrapping
/// `6 → 0` and re-stepping past `1`.
pub fn view_next(c: &mut Ctx) {
    snd(c, 3, 0xA);
    let s = c.vm.r8(c.st, a!(TYPE_SEL));
    let n = if s == 6 {
        0
    } else {
        let n = s.wrapping_add(1);
        if n == 1 {
            2
        } else {
            n
        }
    };
    stepped(c, n);
}

/// The `0x6672`/`0x66BD` tail — store the new `[0x91ED]` and arm the
/// `[0x91EE]`/`[0x91EF]` redraw pair.
fn stepped(c: &mut Ctx, n: u8) {
    c.vm.w8(c.st, a!(TYPE_SEL), n);
    c.vm.w8(c.st, a!(REDRAW), 0xFF);
    c.vm.w8(c.st, a!(PANEL_REQ), 0xFF);
}
