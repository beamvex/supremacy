//! `cs:0x1D35` — the restart tail both endings `jmp` to (file
//! `0x31D35`–`0x31D73`).
//!
//! The asm: `[0x9C9A]=0`, `Present(0)`, the `0xA3BC` hook,
//! `Slot(0x1258)`, the `*IN.BIN`/`*IN.GPH` pair, `Slot(0x125C)`,
//! `Present(0)`, `0xA36A`, `0x85B1` (the title attract — an input-wait
//! slideshow loop whose exit leads back here), then `[0x9C9A]=1`,
//! `0xA3BC`, the base `*.BIN`/`*.GPH` pair, `0xA36A` and
//! `jmp 0x305B` (new-game init). The attract isn't ported — the port
//! emits its `Native` marker and keeps the `[0x9CDA]`/`[0x9CCB]` exit
//! poll as [`Ph::Title`][super::seq::End].

use super::super::consts::{BUTTONS, KEY_PEND};
use super::super::init;
use super::super::vhost::Call;
use super::super::vops::Ctx;
use super::{load_bin, load_gph, CELL_BIN, CELL_GPH, CELL_IN_BIN, CELL_IN_GPH, END_CODE};

/// `u16` casts for the `ds:` slot constants.
macro_rules! a {
    ($k:expr) => {
        u16::try_from($k).unwrap_or(0)
    };
}

/// `0x1D35`–`0x1D5C` — run after the ending's `stage_save` + refresh +
/// `Present(9)` tail: `[0x9C9A]=0`, the `*IN` asset pair, then the
/// `0x85B1` title attract marker with its entry latch clears (file
/// `0x385C0`–`0x385CB`).
pub(super) fn restart_head(c: &mut Ctx) {
    c.vm.w8(c.st, END_CODE, 0);
    c.host.svc(Call::Present(0));
    c.host.svc(Call::Mouse(0x14, 0x1E, 0));
    c.host.svc(Call::Slot(0x1258, 0));
    load_bin(c, CELL_IN_BIN);
    load_gph(c, CELL_IN_GPH);
    c.host.svc(Call::Slot(0x125C, 0));
    c.host.svc(Call::Present(0));
    c.host.svc(Call::Mouse(0x14, 0x1F, 0));
    title_enter(c);
}

/// `0x1D60`–`0x1D71` — `[0x9C9A]=1`, the `0xA3BC` hook, the base
/// `*.BIN`/`*.GPH` pair, `0xA36A`, then `jmp 0x305B` (new-game init;
/// its `0x8432` stage restore puts the saved `ds:` block back).
pub(super) fn restart_tail(c: &mut Ctx) {
    c.vm.w8(c.st, END_CODE, 1);
    c.host.svc(Call::Mouse(0x14, 0x1E, 0));
    load_bin(c, CELL_BIN);
    load_gph(c, CELL_GPH);
    c.host.svc(Call::Mouse(0x14, 0x1F, 0));
    init::act_newgame(c);
}

/// `0x85B1`'s entry — the `Native` marker for the unported slideshow,
/// then `[0x9CDA]=0`, `[0x9CCB]=0` and the channel-A key flush the asm
/// does before polling.
fn title_enter(c: &mut Ctx) {
    c.host.svc(Call::Native(0x85B1));
    c.vm.w16(c.st, a!(BUTTONS), 0);
    c.vm.w8(c.st, a!(KEY_PEND), 0);
    c.host.key_flush();
}
