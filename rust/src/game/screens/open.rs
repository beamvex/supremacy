//! The record-open actions — `cs:0x3F18`/`0x3F41` (records `0xAC68`/
//! `0xAC91`, file `0x33F18`–`0x33F90`) select the clicked list row
//! into `[0x917C]` and `jmp` the surface entry at `0x4071`; `cs:0x3F91`
//! (records `0xACE1`) opens the selected machine's `0x5085` host the
//! same way, with the `0x3FE0` head that routes `[0x9180]` machine
//! kinds through `0x4092`/`0x3FFF`/`0x40B7`; `cs:0x4BA1` (`0xB8F1`)
//! copies `[0x917C]` into `[0x9184]` and falls into `0x4BA7`.

use super::super::consts::{
    LINK_REC, REC_BASE, REC_CUR, REC_STRIDE, SEL_MACH, SEL_REC, SEQ_AUX, SEQ_PTR,
};
use super::super::vhost::Call;
use super::super::vops::{faction_rec, kind_of, Ctx};
use super::super::{selrec, shell, text};
use super::detail::S_REFUSE;
use super::machd::mach_host;
use super::{build::enter_build, surface};

/// `u16` casts for the `ds:` slot constants.
macro_rules! a {
    ($k:expr) => {
        u16::try_from($k).unwrap_or(0)
    };
}

/// `cs:0x3F18` — the row-select head: `0x914C` link `+2` clamped to
/// `[0x9158] .. faction_end` → `[0x917C]`, then the `0x3F41` tail
/// (file `0x33F18`–`0x33F40`).
pub fn order_sel(c: &mut Ctx) {
    clr_seq(c);
    let link = c.vm.r16(c.st, a!(LINK_REC));
    let si = c.vm.r16(c.st, link + 2);
    let lo = c.vm.r16(c.st, a!(REC_BASE));
    let hi = faction_rec(c).wrapping_add(a!(REC_STRIDE));
    if si >= lo && si < hi {
        c.vm.w16(c.st, a!(SEL_REC), si);
    }
    order_open(c);
}

/// `cs:0x3F41` — the open tail: sequencer clears, `0x5EE6` resync,
/// then the `0x3F50` kind dispatch (file `0x33F41`–`0x33F90`).
pub fn order_open(c: &mut Ctx) {
    clr_seq(c);
    selrec::sync_sel(c);
    sel_dispatch(c);
}

/// `0x3F50` — `[0x917C]` (or `[0x9184]` when clear) through the
/// admit guard → `0x4071` surface body or the `0x2434` refuse.
fn sel_dispatch(c: &mut Ctx) {
    let mut rec = c.vm.r16(c.st, a!(SEL_REC));
    if rec == 0 {
        rec = c.vm.r16(c.st, a!(REC_CUR));
    }
    if admissible(c, rec) {
        surface::open_entry(c);
    } else {
        refuse(c);
    }
}

/// The `0x3F61`/`0x3FB3` guard — colonised (`0xA`) admits, kinds
/// `4`/`1` refuse, else the `+0x18`/`+0x1C`/`+0x20` bays or a nonzero
/// `+0x2E` admit.
fn admissible(c: &mut Ctx, rec: u16) -> bool {
    let kind = kind_of(c, rec);
    if kind == 0xA {
        return true;
    }
    if kind == 4 || kind == 1 {
        return false;
    }
    let aboard = c.vm.r8(c.st, rec + 0x18) | c.vm.r8(c.st, rec + 0x1C) | c.vm.r8(c.st, rec + 0x20);
    aboard != 0 || c.vm.r16(c.st, rec + 0x2E) != 0
}

/// `0x2434` — the `0x7402` refuse then the `jmp 0x2CC5` shell re-entry.
fn refuse(c: &mut Ctx) {
    text::enqueue(c, S_REFUSE);
    shell::shell_enter(c);
}

/// `cs:0x3F91` — machine-open: `0x9180`/launched guards fall back to
/// `0x3F41`, the `0x5085` host through the same guard, then the
/// `0x3FE0` kind-of-entry fan (file `0x33F91`–`0x34035`).
pub fn mach_open(c: &mut Ctx) {
    clr_seq(c);
    let m = c.vm.r16(c.st, a!(SEL_MACH));
    if m == 0 || c.vm.r8(c.st, m + 0x1F) & 4 != 0 {
        order_open(c);
        return;
    }
    let host = mach_host(c, m);
    if !admissible(c, host) {
        return refuse(c);
    }
    c.vm.w16(c.st, a!(SEL_REC), host);
    selrec::sync_sel(c);
    let m = c.vm.r16(c.st, a!(SEL_MACH));
    if m == 0 {
        surface::resume_entry(c);
    } else if c.vm.r8(c.st, m + 0x1E) == 1 {
        dac(c, m);
    } else {
        surface::open_at(c);
    }
}

/// `0x3FFF` — the DAC variant: the surface draw head, the machine's
/// name at `(0x14,0xE)`, `[0x91DD] = 0xFF`, the `0x42C3`/`0x57BC`/
/// `0x5994` natives, then `jmp 0x40B7` (file `0x33FFF`–`0x34035`).
fn dac(c: &mut Ctx, m: u16) {
    surface::draw(c);
    text::print_str(c, m, 0x14, 0xE);
    c.vm.w8(c.st, 0x91DD, 0xFF);
    for f in [0x42C3u16, 0x57BC, 0x5994] {
        c.host.svc(Call::Native(f));
    }
    surface::resume_entry(c);
}

/// `cs:0x4BA1` — the orders "build" row: `[0x9184] = [0x917C]` then
/// falls into the `0x4BA7` build entry (file `0x34BA1`–`0x34BA6`).
pub fn order_build(c: &mut Ctx) {
    let rec = c.vm.r16(c.st, a!(SEL_REC));
    c.vm.w16(c.st, a!(REC_CUR), rec);
    enter_build(c, false);
}

/// The `[0x9198]`/`[0x919A]` sequencer clears every head runs.
fn clr_seq(c: &mut Ctx) {
    c.vm.w16(c.st, a!(SEQ_PTR), 0);
    c.vm.w16(c.st, a!(SEQ_AUX), 0);
}
