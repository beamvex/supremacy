//! `cs:0x1D35` — the restart tail both endings `jmp` to (file
//! `0x31D35`–`0x31D73`).
//!
//! The asm: `[0x9C9A]=0`, `Present(0)`, the `0xA3BC` hook,
//! `Slot(0x1258)`, the `*IN.BIN`/`*IN.GPH` pair, `Slot(0x125C)`,
//! `Present(0)`, `0xA36A`, `0x85B1` (the title attract — now the
//! frame-stepped [`title`][super::super::title] machine parked in
//! `Vm::title`), then `[0x9C9A]=1`, `0xA3BC`, the base `*.BIN`/`*.GPH`
//! pair, `0xA36A` and `jmp 0x305B` (new-game init).

use super::super::init;
use super::super::title;
use super::super::vhost::Call;
use super::super::vops::Ctx;
use super::{load_bin, load_gph, CELL_BIN, CELL_GPH, CELL_IN_BIN, CELL_IN_GPH, END_CODE};

/// `0x1D35`–`0x1D5C` — run after the ending's `0x2F91` stage restore +
/// refresh + `Present(9)` tail: `[0x9C9A]=0`, the `*IN` asset pair,
/// then `title::enter` — the `0x85B1` attract entry.
pub(crate) fn restart_head(c: &mut Ctx) {
    c.vm.w8(c.st, END_CODE, 0);
    c.host.svc_st(Call::Present(0), c.vm, c.st);
    c.host.svc(Call::Mouse(0x14, 0x1E, 0));
    c.host.svc(Call::Slot(0x1258, 0));
    load_bin(c, CELL_IN_BIN);
    load_gph(c, CELL_IN_GPH);
    c.host.svc(Call::Slot(0x125C, 0));
    c.host.svc_st(Call::Present(0), c.vm, c.st);
    c.host.svc(Call::Mouse(0x14, 0x1F, 0));
    title::enter(c);
}

/// `0x1D60`–`0x1D71` — `[0x9C9A]=1`, the `0xA3BC` hook, the base
/// `*.BIN`/`*.GPH` pair, `0xA36A`, then `jmp 0x305B` (new-game init;
/// its `0x8432` stage save snapshots the configured `ds:` block — the
/// preset the picker just wrote survives because nothing restores
/// over it).
pub(crate) fn restart_tail(c: &mut Ctx) {
    c.vm.w8(c.st, END_CODE, 1);
    c.host.svc(Call::Mouse(0x14, 0x1E, 0));
    load_bin(c, CELL_BIN);
    load_gph(c, CELL_GPH);
    c.host.svc(Call::Mouse(0x14, 0x1F, 0));
    init::act_newgame(c);
}
