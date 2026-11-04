//! The surface screen's docking-bay actions — the `0xABA4` list's
//! `surf_slot` bay picks (`cs:0x424B`/`0x42F5`/`0x42FF`), the
//! `surf_assign` cargo deploys (`cs:0x4AA6`/`0x4A9E`/`0x4A96`/
//! `0x4A8E`) and the shared table rebuilds: `0x57BC` fills the
//! `ds:0x4FD0` cargo slots from the fleet records hosted by
//! `[0x9180]`, `0x4190` fills the `ds:0x9AB1` surface cells from the
//! records hosted by `[0x917C]` (files `0x34190`–`0x34308`,
//! `0x34A8E`–`0x34AC7`, `0x357BC`–`0x35866`).

use super::super::consts::{
    FLEET_BASE, FLEET_STRIDE, MACH_BASE, MACH_STRIDE, REC_CUR, SEL_MACH, SEL_REC,
};
use super::super::mach::M_TYPE;
use super::super::rec::{F_LINK_A, F_LINK_B, F_LINK_C};
use super::super::vops::Ctx;
use super::super::{num, text};
use super::{dfield, galmsg, stact};

/// `u16` casts for the `ds:` slot constants.
macro_rules! a {
    ($k:expr) => {
        u16::try_from($k).unwrap_or(0)
    };
}

/// `ds:` base of the four cargo-slot dwords `0x57BC` fills.
const CARGO: u16 = 0x4FD0;
/// `ds:` base of the 24-cell surface table `0x4190` fills.
const SURF: u16 = 0x9AB1;
/// Fleet records scanned per pass (`cx = 0x17` seed + `jns` tail).
const FLEET_N: u16 = 0x18;

/// The `0xABA4` records' `+0xC` words — both windows' forms.
/// Anything else drops to the `Call::Native` sink.
pub(super) fn dispatch(c: &mut Ctx, t: u16) {
    match t {
        0x424B | 0xAF9B => slot(c, a!(REC_CUR), a!(F_LINK_A)),
        0x42F5 | 0xB045 => slot(c, a!(SEL_REC), a!(F_LINK_B)),
        0x42FF | 0xB04F => slot(c, a!(SEL_REC), a!(F_LINK_C)),
        0x4AA6 | 0xB7F6 => assign(c, 0),
        0x4A9E | 0xB7EE => assign(c, 1),
        0x4A96 | 0xB7E6 => assign(c, 2),
        0x4A8E | 0xB7DE => assign(c, 3),
        _ => stact::dispatch(c, t),
    }
}

/// `cs:0x424B`/`0x42F5`/`0x42FF` — `surf_slot 1/2/3`: the bay byte at
/// `[sel]+f` names a machine; `+0x1E == 1` runs `0x42A7` (`0x3424B`+).
fn slot(c: &mut Ctx, sel: u16, f: u16) {
    let rec = c.vm.r16(c.st, sel);
    let b = c.vm.r8(c.st, rec.wrapping_add(f));
    if b == 0 {
        return refuse(c, 0x69B1, 0xB);
    }
    let m = a!(MACH_BASE).wrapping_add(u16::from(b).wrapping_sub(1).wrapping_mul(a!(MACH_STRIDE)));
    if c.vm.r8(c.st, m + a!(M_TYPE)) != 1 {
        type_name(c);
        return refuse(c, 0x754E, 8);
    }
    c.vm.w16(c.st, a!(SEL_MACH), m);
    select(c, m);
}

/// `cs:0x4AA6`/`0x4A9E`/`0x4A96`/`0x4A8E` — `surf_assign 0..3`: the
/// `ds:0x4FD0+4n` unit lands on `[0x9184]` (`+0x2`), `+0x6` `0x4`
/// dropped; empty takes `0x4257` (file `0x34A8E`–`0x34AC7`).
fn assign(c: &mut Ctx, n: u16) {
    let u = c.vm.r16(c.st, CARGO.wrapping_add(n.wrapping_mul(4)));
    if u == 0 {
        return refuse(c, 0x69B1, 0xB);
    }
    let rec = c.vm.r16(c.st, a!(REC_CUR));
    c.vm.w16(c.st, u + 2, rec);
    let v = c.vm.r8(c.st, u + 6) & 0xFB;
    c.vm.w8(c.st, u + 6, v);
    cargo(c);
    planet(c);
}

/// The `0x42A7`/`0x3FFF` select tail — the machine's `+0` name at
/// `(0x14,0xE)`, `[0x91DD]` armed, `0x42C3` and `0x57BC`+`0x5994`.
pub(super) fn select(c: &mut Ctx, m: u16) {
    text::print_str(c, m, 0x14, 0xE);
    c.vm.w8(c.st, 0x91DD, 0xFF);
    type_name(c);
    cargo(c);
    dfield::bays(c);
}

/// `cs:0x42C3` — `[0x9180]`'s `+0x1E` type → the `0x79A4` name table
/// (stride `0x16`) printed at `(6,0x50)` (file `0x342C3`–`0x342F4`).
pub(super) fn type_name(c: &mut Ctx) {
    let m = c.vm.r16(c.st, a!(SEL_MACH));
    let t = u16::from(c.vm.r8(c.st, m + a!(M_TYPE))).wrapping_sub(1);
    text::print_str(c, 0x79A4u16.wrapping_add(t.wrapping_mul(0x16)), 6, 0x50);
}

/// `cs:0x57BC` — rebuild the `ds:0x4FD0` cargo table: slots take the
/// fleet records hosted by `[0x9180]` (the `+0x6` `0x2` bit) down
/// `(0x14,0x1F)`; leftovers blank (file `0x357BC`–`0x35866`).
pub(super) fn cargo(c: &mut Ctx) {
    let m = c.vm.r16(c.st, a!(SEL_MACH));
    let (mut bp, mut bx, mut dx, mut n) = (CARGO, 0x14u16, 0x1Fu16, 0u8);
    clear(c, CARGO, 4);
    for i in 0..FLEET_N {
        let u = a!(FLEET_BASE).wrapping_add(i.wrapping_mul(a!(FLEET_STRIDE)));
        if c.vm.r16(c.st, u + 2) != m || c.vm.r8(c.st, u + 6) & 2 == 0 {
            continue;
        }
        c.vm.w16(c.st, bp, u);
        let end = row(c, i + 1, c.vm.r16(c.st, u), bx, dx);
        bp = bp.wrapping_add(4);
        (bx, dx) = advance(end, dx, false);
        n = n.wrapping_add(1);
    }
    erase(c, 4u8.wrapping_sub(n), bx, dx, false);
}

/// `cs:0x4190` — rebuild the `ds:0x9AB1` surface table: cells take
/// the fleet records hosted by `[0x917C]` (a `+0x6` `0x8` bit skips)
/// column-major from `(3,0x74)`; leftovers blank (file `0x34190`–
/// `0x3424A`).
pub(super) fn planet(c: &mut Ctx) {
    let rec = c.vm.r16(c.st, a!(SEL_REC));
    let (mut bp, mut bx, mut dx, mut n) = (SURF, 3u16, 0x74u16, 0u8);
    clear(c, SURF, FLEET_N);
    for i in 0..FLEET_N {
        let u = a!(FLEET_BASE).wrapping_add(i.wrapping_mul(a!(FLEET_STRIDE)));
        if c.vm.r16(c.st, u + 2) != rec || c.vm.r8(c.st, u + 6) & 8 != 0 {
            continue;
        }
        c.vm.w16(c.st, bp, u);
        let end = row(c, i + 1, c.vm.r16(c.st, u), bx, dx);
        bp = bp.wrapping_add(4);
        (bx, dx) = advance(end, dx, true);
        n = n.wrapping_add(1);
    }
    erase(c, 0x18u8.wrapping_sub(n), bx, dx, true);
}

/// The `0x57C2`/`0x4196` clear — `n` dword slots zeroed.
fn clear(c: &mut Ctx, base: u16, n: u16) {
    for i in 0..n {
        c.vm.w16(c.st, base + i * 4, 0);
        c.vm.w16(c.st, base + i * 4 + 2, 0);
    }
}

/// The `0x57EC`/`0x41BA` row print — `num2(idx)`, the `0x604E`
/// ordinal, a pad, `num3(v)`; returns `bx` past the 8-char field.
fn row(c: &mut Ctx, idx: u16, v: u16, bx: u16, dx: u16) -> u16 {
    let bx = num::num2(c, idx, bx, dx);
    let bx = suffix(c, idx, bx, dx);
    num::num3(c, v, bx.wrapping_add(1), dx)
}

/// `cs:0x604E` — the index's ordinal suffix: `1`/`0x15` → `0x68A2`
/// "ST", `2`/`0x16` → `0x68A5` "ND", `3`/`0x17` → `0x68A8` "RD", else
/// `0x68AB` "TH" (file `0x3604E`–`0x36074`).
fn suffix(c: &mut Ctx, idx: u16, bx: u16, dx: u16) -> u16 {
    let s = match idx {
        1 | 0x15 => 0x68A2,
        2 | 0x16 => 0x68A5,
        3 | 0x17 => 0x68A8,
        _ => 0x68AB,
    };
    text::print_str(c, s, bx, dx)
}

/// The `0x5843`/`0x421F` blank tail — `n` `0x68B6`+`0x68B7` erases.
fn erase(c: &mut Ctx, mut n: u8, mut bx: u16, mut dx: u16, grid: bool) {
    while n != 0 {
        let end = text::print_str(c, 0x68B6, bx, dx);
        let end = text::print_str(c, 0x68B7, end.wrapping_add(1), dx);
        (bx, dx) = advance(end, dx, grid);
        n = n.wrapping_sub(1);
    }
}

/// The `−8`/`+0xA` step on the post-print `bx` — `0x57BC` word ops,
/// `0x4190` `bl`/`dl` byte ops (`dl == 0xC4` → `dx = 0x74`, `bl + 0xA`).
fn advance(bx: u16, dx: u16, grid: bool) -> (u16, u16) {
    if !grid {
        return (bx.wrapping_sub(8), dx.wrapping_add(0xA));
    }
    let (bl, dl) = (bx.wrapping_sub(8) & 0xFF, dx.wrapping_add(0xA) & 0xFF);
    if dl == 0xC4 {
        return (bl.wrapping_add(0xA) & 0xFF, 0x74);
    }
    (bl, dl)
}

/// `0x4257`/`0x425D` — refuse `si` at `(bx,0x59)`, wait, `0x68BB` erase.
pub(super) fn refuse(c: &mut Ctx, s: u16, bx: u16) {
    text::print_str(c, s, bx, 0x59);
    galmsg::wait(c);
    text::print_str(c, 0x68BB, 7, 0x59);
}
