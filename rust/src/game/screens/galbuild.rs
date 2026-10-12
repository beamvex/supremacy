//! The galaxy-screen build action — `cs:0x642E` (records `0xD17E`,
//! file `0x3642E`–`0x36610`): gates the `[0x9154]` type record through
//! the kind-`4`/difficulty/`0x91BC` checks, the faction bays/funds/
//! stocks guards and the `0x6611` free-slot scan, runs the `0xE7CD`
//! name edit into the unit record, then takes a bay, deducts the
//! costs, re-syncs the selection and reports via the `0x6491`
//! message tail (`0x68D8` banner + overlay print + `0x6F2D` wait +
//! `0x3AE9` panel reprint).

use super::super::cells::{IN_DONE, LIST_MAX};
use super::super::consts::{DAY, DIFFICULTY, MACH_BASE, PANEL_REQ, REINF_REC, SEL_REC, TYPE_REC};
use super::super::vops::{faction_rec, Ctx};
use super::super::{namedit, selrec, text};
use super::galmsg::{msg, overlay};
use super::snd;

/// `u16` casts for the `ds:` slot constants.
macro_rules! a {
    ($k:expr) => {
        u16::try_from($k).unwrap_or(0)
    };
}

/// `[0x91E0]` — the `0x6611` table-full flag.
const MACH_FULL: u16 = 0x91E0;
/// Unit-record stride (`0x6611`/`0x6632`).
const MACH_STRIDE: u16 = 0x28;
/// Unit-table slot count (`cx = 0x1F` + the `jns` iteration).
const MACH_SLOTS: u8 = 0x20;
/// The `0x68D8` print-row banner both `msg` flavours draw first.
const S_BANNER: u16 = 0x68D8;
/// The shared message-row y.
const MSG_Y: u16 = 0x92;

/// `cs:0x642E` — the galaxy build button (file `0x3642E`–`0x36610`).
pub fn gal_build(c: &mut Ctx) {
    snd(c, 0x1B, 0x18);
    if let Some(s) = refused(c) {
        msg(c, s);
        return;
    }
    let Some((m, _)) = mach_free(c) else {
        msg(c, 0x6D58);
        return;
    };
    c.vm.w16(c.st, a!(REINF_REC), m);
    text::print_str(c, 0x6D8E, 0x12, MSG_Y);
    namedit::type_edit(c, m, 0x39, MSG_Y, 8);
    if c.vm.r8(c.st, a!(IN_DONE)) != 0 {
        overlay(c, 0x6B24, 0x39); // the `jmp 0x64AA` cancel path
        return;
    }
    commit(c);
}

/// The `0x644A`–`0x6500` gate chain — kind-`4` day/dup checks, then
/// the faction bays/funds/stocks checks; `Some` = the refuse string.
fn refused(c: &mut Ctx) -> Option<u16> {
    let t = c.vm.r16(c.st, a!(TYPE_REC));
    if c.vm.r8(c.st, t + 0x12) == 4 {
        if c.vm.r8(c.st, a!(DIFFICULTY)) == 0 && c.vm.r16(c.st, a!(DAY)) < 0x7DB {
            return Some(0x6DFA);
        }
        if mach_count(c, 4) != 0 {
            return Some(0x6C4A);
        }
    }
    refused_faction(c, t)
}

/// `0x6477`–`0x6500` — bays full `→ 0x6C80`, funds `< +6 → 0x6CB6`,
/// difficulty-gated `+0x30/+0x10` and `+0x34/+0xE` stock checks.
fn refused_faction(c: &mut Ctx, t: u16) -> Option<u16> {
    let f = faction_rec(c);
    if c.vm.r8(c.st, f + 0x18) != 0 && c.vm.r8(c.st, f + 0x1C) != 0 && c.vm.r8(c.st, f + 0x20) != 0
    {
        return Some(0x6C80);
    }
    let funds = u32::from(c.vm.r16(c.st, f + 0x38)) << 16 | u32::from(c.vm.r16(c.st, f + 0x36));
    if funds < u32::from(c.vm.r16(c.st, t + 0x6)) {
        return Some(0x6CB6);
    }
    let d = c.vm.r8(c.st, a!(DIFFICULTY));
    if d >= 2 && c.vm.r16(c.st, f + 0x30) < c.vm.r16(c.st, t + 0x10) {
        return Some(0x6CEC);
    }
    if d != 0 && c.vm.r16(c.st, f + 0x34) < c.vm.r16(c.st, t + 0xE) {
        return Some(0x6D22);
    }
    None
}

/// `0x6547`–`0x65F0` — the accepted path: re-find the slot's index,
/// take the first free faction bay, init the unit fields, deduct.
fn commit(c: &mut Ctx) {
    let (_, idx) = mach_free(c).unwrap_or((0, 0));
    let f = faction_rec(c);
    let bay = if c.vm.r8(c.st, f + 0x18) == 0 {
        0x18
    } else if c.vm.r8(c.st, f + 0x1C) == 0 {
        0x1C
    } else {
        0x20
    };
    c.vm.w8(c.st, f + bay, idx);
    init_unit(c);
    deduct(c);
    text::print_str(c, S_BANNER, 0x12, MSG_Y);
    c.vm.w8(c.st, a!(PANEL_REQ), 0xFF);
    c.vm.w16(c.st, a!(SEL_REC), f);
    selrec::sync_sel(c);
    msg(c, 0x6DC4);
}

/// `0x6571`–`0x65BD` — zero the unit's run fields, copy the type
/// `+8` speed and `+0x12/+0x13` kind/class bytes, host = `[0x91C6]`.
fn init_unit(c: &mut Ctx) {
    let m = c.vm.r16(c.st, a!(REINF_REC));
    let t = c.vm.r16(c.st, a!(TYPE_REC));
    c.vm.w16(c.st, m + 0xA, 0);
    let v = c.vm.r16(c.st, t + 0x8);
    c.vm.w16(c.st, m + 0xC, v);
    for o in [0xEu16, 0x10, 0x12, 0x14, 0x18, 0x22, 0x24, 0x26] {
        c.vm.w16(c.st, m + o, 0);
    }
    let (kind, class) = (c.vm.r8(c.st, t + 0x12), c.vm.r8(c.st, t + 0x13));
    c.vm.w8(c.st, m + 0x1E, kind);
    c.vm.w8(c.st, m + 0x1F, 0);
    c.vm.w8(c.st, m + 0x20, class);
    let host = c.vm.r8(c.st, a!(LIST_MAX));
    c.vm.w8(c.st, m + 0x21, host);
}

/// `0x65BE`–`0x65F0` — the 32-bit `+0x36/+0x38` fund subtract and the
/// difficulty-gated `+0x30`/`+0x34` stock deducts.
fn deduct(c: &mut Ctx) {
    let fac = faction_rec(c);
    let ty = c.vm.r16(c.st, a!(TYPE_REC));
    let funds = u32::from(c.vm.r16(c.st, fac + 0x38)) << 16 | u32::from(c.vm.r16(c.st, fac + 0x36));
    let v = funds.wrapping_sub(u32::from(c.vm.r16(c.st, ty + 0x6)));
    c.vm.w16(c.st, fac + 0x36, u16::try_from(v & 0xFFFF).unwrap_or(0));
    c.vm.w16(c.st, fac + 0x38, u16::try_from(v >> 16).unwrap_or(0));
    let diff = c.vm.r8(c.st, a!(DIFFICULTY));
    if diff >= 2 {
        let v =
            c.vm.r16(c.st, fac + 0x30)
                .wrapping_sub(c.vm.r16(c.st, ty + 0x10));
        c.vm.w16(c.st, fac + 0x30, v);
    }
    if diff != 0 {
        let v =
            c.vm.r16(c.st, fac + 0x34)
                .wrapping_sub(c.vm.r16(c.st, ty + 0xE));
        c.vm.w16(c.st, fac + 0x34, v);
    }
}

/// `cs:0x6611` — first `+0x1E == 0` unit slot → `(slot, index + 1)`;
/// sets `[0x91E0] = 0xFF` and returns `None` on a full table (file
/// `0x36611`–`0x36631`).
fn mach_free(c: &mut Ctx) -> Option<(u16, u8)> {
    c.vm.w8(c.st, MACH_FULL, 0);
    for i in 0..MACH_SLOTS {
        let m = a!(MACH_BASE).wrapping_add(u16::from(i).wrapping_mul(MACH_STRIDE));
        if c.vm.r8(c.st, m + 0x1E) == 0 {
            return Some((m, i.wrapping_add(1)));
        }
    }
    c.vm.w8(c.st, MACH_FULL, 0xFF);
    None
}

/// `cs:0x6632` — count unit slots whose `+0x1E` kind == `al` (file
/// `0x36632`–`0x36647`).
fn mach_count(c: &mut Ctx, al: u8) -> u8 {
    let n = (0..MACH_SLOTS)
        .filter(|&i| {
            let m = a!(MACH_BASE).wrapping_add(u16::from(i).wrapping_mul(MACH_STRIDE));
            c.vm.r8(c.st, m + 0x1E) == al
        })
        .count();
    u8::try_from(n).unwrap_or(0)
}
