//! The `0x9EDE` detail-list dispatch and the machine-side actions —
//! `cs:0x297E`/`0x2ADD`/`0x2AEA` `mach_sel 1/2/3`, the `0x4D0B`/
//! `0x4D12` fuel slider and `0x4DBB` `xfer_all_sel`. The STORES
//! movers live in [`dxfer`], the unload/scuttle pair in [`dscut`]
//! (files `0x3297E`–`0x3299B`, `0x32ADD`–`0x32AF6`,
//! `0x34D0B`–`0x34E09`).

use super::super::consts::{ARMED, REC_CUR, REP_DIR, REP_IN, SEL_MACH};
use super::super::mach::M_DEPOSIT;
use super::super::rec::{
    F_ENERGY, F_FOOD, F_FUEL, F_LINK_A, F_LINK_B, F_LINK_C, F_MINERALS, F_POP,
};
use super::super::text;
use super::super::vhost::Call;
use super::super::vops::Ctx;
use super::{dfield, dmach, dscut, dxfer, galmsg, ordact};

/// `u16` casts for the `ds:` slot constants.
macro_rules! a {
    ($k:expr) => {
        u16::try_from($k).unwrap_or(0)
    };
}
/// `u16` casts for the record/machine field constants.
macro_rules! f {
    ($k:expr) => {
        u16::try_from($k).unwrap_or(0)
    };
}

/// The `0x9EDE` records' `+0xC` words — each arm maps both the
/// `0x286B`-space alias the shipped records carry and the `0x6D50`-
/// lower runtime form, like [`super::nav`]. Anything else drops to
/// the `Call::Native` sink.
pub(super) fn dispatch(c: &mut Ctx, t: u16) {
    match t {
        0x297E | 0x96CE => mach_sel(c, f!(F_LINK_A)),
        0x2ADD | 0x982D => mach_sel(c, f!(F_LINK_B)),
        0x2AEA | 0x983A => mach_sel(c, f!(F_LINK_C)),
        0x265F | 0x93AF => dxfer::xfer_in(c, f!(F_FOOD), 0xA, dfield::col_food),
        0x269C | 0x93EC => dxfer::xfer_in(c, f!(F_MINERALS), 0xE, dfield::col_min),
        0x26D9 | 0x9429 => dxfer::xfer_in(c, f!(F_FUEL), 0x10, dfield::col_fuel),
        0x2719 | 0x9469 => dxfer::xfer_in(c, f!(F_ENERGY), 0x12, dfield::col_nrg),
        0x2AF7 | 0x9847 => dxfer::xfer_out(c, f!(F_FOOD), 0xA, dfield::col_food),
        0x2B20 | 0x9870 => dxfer::xfer_out(c, f!(F_MINERALS), 0xE, dfield::col_min),
        0x2B49 | 0x9899 => dxfer::xfer_out(c, f!(F_FUEL), 0x10, dfield::col_fuel),
        0x2B72 | 0x98C2 => dxfer::xfer_out(c, f!(F_ENERGY), 0x12, dfield::col_nrg),
        0x4C9B | 0xB9EB => dxfer::people_in(c),
        0x4CD8 | 0xBA28 => dxfer::people_out(c),
        0x4D0B | 0xBA5B => fuel_slider(c, 0xFF),
        0x4D12 | 0xBA62 => fuel_slider(c, 0),
        0x4DBB | 0xBB0B => xfer_all_sel(c),
        0x4E0A | 0xBB5A => dscut::unload_all(c),
        0x2B9B | 0x98EB => dscut::scuttle(c),
        _ => drop(c.host.svc(Call::Native(t))),
    }
}

/// `cs:0x297E`/`0x2ADD`/`0x2AEA` — `mach_sel 1/2/3`: the bay byte at
/// `[0x9184] + f` → the `0x299C` select tail; an empty bay takes the
/// `0x69B1`/`0x6F2D`/`0x68C7` msg path (files `0x3297E`–`0x3299B`,
/// `0x32ADD`–`0x32AF6`).
fn mach_sel(c: &mut Ctx, f: u16) {
    let rec = c.vm.r16(c.st, a!(REC_CUR));
    let b = c.vm.r8(c.st, rec.wrapping_add(f));
    if b != 0 {
        return dmach::sel_mach(c, b);
    }
    refuse(c, 0x69B1);
}

/// `cs:0x4D0B`/`0x4D12` — the fuel slider: `[0x91F8] = dir`; a
/// `0xFFFF` type `+0xC` refuses outright, else the `[0x91F5]`–
/// `[0x91F8]` accelerator paces held presses and the `0xC −
/// [0x91F6]` step moves machine `+0x26` ⇄ planet `+0x32` (`dir` set =
/// machine → planet); tail `0x26FE` (file `0x34D0B`–`0x34DBA`).
fn fuel_slider(c: &mut Ctx, dir: u8) {
    c.vm.w8(c.st, a!(REP_DIR), dir);
    let trec = dmach::type_rec(c);
    let cap = c.vm.r16(c.st, trec + 0xC);
    if cap == 0xFFFF {
        return;
    }
    c.vm.w8(c.st, a!(ARMED), 0);
    if !ordact::accel(c) {
        return;
    }
    let p = c.vm.r16(c.st, a!(REC_CUR)).wrapping_add(f!(F_FUEL));
    let m = c.vm.r16(c.st, a!(SEL_MACH)).wrapping_add(f!(M_DEPOSIT));
    let step = 0xCu16.wrapping_sub(u16::from(c.vm.r8(c.st, a!(REP_IN))));
    let mut bx = step;
    if dir != 0 {
        bx = bx.min(c.vm.r16(c.st, m));
        if dxfer::CAP.wrapping_sub(bx) < c.vm.r16(c.st, p) {
            bx = dxfer::CAP.wrapping_sub(c.vm.r16(c.st, p));
        }
        let v = c.vm.r16(c.st, m).wrapping_sub(bx);
        c.vm.w16(c.st, m, v);
        let v = c.vm.r16(c.st, p).wrapping_add(bx);
        c.vm.w16(c.st, p, v);
    } else {
        bx = bx.min(c.vm.r16(c.st, p));
        if cap.wrapping_sub(bx) < c.vm.r16(c.st, m) {
            bx = cap.wrapping_sub(c.vm.r16(c.st, m));
        }
        let v = c.vm.r16(c.st, p).wrapping_sub(bx);
        c.vm.w16(c.st, p, v);
        let v = c.vm.r16(c.st, m).wrapping_add(bx);
        c.vm.w16(c.st, m, v);
    }
    dfield::col_fuel(c);
}

/// `cs:0x4DBB` — `xfer_all_sel`: `0xFFFE` type `+0xA` → `0x753B`, a
/// live `+0x14` → `0x6A25`, a borrowing `+0x2A` debit → `0x6A51` (the
/// `0x4DD7` msg tail each); else `+0x14 = type +0xA` off the
/// population, `0x2604`, `0x6A3E` (file `0x34DBB`–`0x34E09`).
fn xfer_all_sel(c: &mut Ctx) {
    let t = dmach::type_rec(c);
    if c.vm.r16(c.st, t + 0xA) == 0xFFFE {
        return refuse(c, 0x753B);
    }
    let m = c.vm.r16(c.st, a!(SEL_MACH));
    if c.vm.r16(c.st, m + 0x14) != 0 {
        return refuse(c, 0x6A25);
    }
    let amt = c.vm.r16(c.st, t + 0xA);
    let p = c.vm.r16(c.st, a!(REC_CUR)).wrapping_add(f!(F_POP));
    let pop = c.vm.r16(c.st, p);
    let Some(left) = pop.checked_sub(amt) else {
        return refuse(c, 0x6A51);
    };
    c.vm.w16(c.st, p, left);
    c.vm.w16(c.st, m + 0x14, amt);
    dfield::pop(c);
    refuse(c, 0x6A3E);
}

/// `0x32DD7` — the msg tail the guards share: `si` at `(0xA,0x52)`,
/// the `0x6F2D` wait, then `0x68C7`.
fn refuse(c: &mut Ctx, s: u16) {
    text::print_str(c, s, 0xA, 0x52);
    galmsg::wait(c);
    text::print_str(c, 0x68C7, 0xA, 0x52);
}
