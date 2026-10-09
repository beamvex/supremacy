//! The galaxy-screen menu entry — `cs:0x3906`/`0x3909` (file
//! `0x33906`–`0x3395A`).

use super::super::consts::{ARMED, PANEL_ON, PANEL_REQ, REDRAW, SEL_REC};
use super::super::dialog;
use super::super::vhost::Call;
use super::super::vops::{faction_rec, Ctx};
use super::install;

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
