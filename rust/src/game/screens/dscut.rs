//! The detail screen's destructive pair — `cs:0x4E0A` `unload_all`
//! (the meter-animated stock flush) and `cs:0x2B9B` `scuttle` (the
//! `0x7CBD`-gated `Y` confirm, the `0x9991` fleet relink, the
//! refund/salvage writes and the `0x2493` re-entry) (files
//! `0x32B9B`–`0x32CA8`, `0x34E0A`–`0x34EA9`).

use super::super::consts::{
    FLEET_BASE, FLEET_COUNT, FLEET_STRIDE, MACH_BASE, MACH_STRIDE, REC_BASE, REC_CUR, REC_STRIDE,
    SEL_MACH, SEL_REC,
};
use super::super::keymap::{KEYMAP, KEYMAP_N};
use super::super::mach::{M_DEPOSIT, M_HOST, M_TIMER, M_TYPE};
use super::super::rec::{F_CREDITS, F_ENERGY, F_FOOD, F_FUEL, F_LINK_A, F_MINERALS, F_POP};
use super::super::text;
use super::super::vhost::Call;
use super::super::vops::Ctx;
use super::{bump, dbars, detail, dfield, dmach, dxfer};

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

/// `cs:0x4E0A` — `unload_all`: each pass runs the `xfer_out`s whose
/// planet stock has room, then `0x2759` + a frame bump; the tail
/// re-enters at the first still-pending stock until every machine
/// field is empty or its planet side is full (file
/// `0x34E0A`–`0x34EA9`).
pub(super) fn unload_all(c: &mut Ctx) {
    let mut at = 0;
    loop {
        if at <= 0 {
            let p = c.vm.r16(c.st, a!(SEL_REC));
            if c.vm.r16(c.st, p.wrapping_add(f!(F_FOOD))) < dxfer::CAP {
                dxfer::xfer_out(c, f!(F_FOOD), 0xA, dfield::col_food);
            }
        }
        if at <= 1 {
            let p = c.vm.r16(c.st, a!(SEL_REC));
            if c.vm.r16(c.st, p.wrapping_add(f!(F_MINERALS))) < dxfer::CAP {
                dxfer::xfer_out(c, f!(F_MINERALS), 0xE, dfield::col_min);
            }
        }
        if at <= 2 {
            let p = c.vm.r16(c.st, a!(SEL_REC));
            if c.vm.r16(c.st, p.wrapping_add(f!(F_FUEL))) < dxfer::CAP {
                dxfer::xfer_out(c, f!(F_FUEL), 0x10, dfield::col_fuel);
            }
        }
        if at <= 3 {
            let p = c.vm.r16(c.st, a!(SEL_REC));
            if c.vm.r16(c.st, p.wrapping_add(f!(F_ENERGY))) < dxfer::CAP {
                dxfer::xfer_out(c, f!(F_ENERGY), 0x12, dfield::col_nrg);
            }
        }
        if at <= 4 {
            let p = c.vm.r16(c.st, a!(SEL_REC));
            if c.vm.r16(c.st, p.wrapping_add(f!(F_POP))) < dxfer::CAP {
                dxfer::people_out(c);
            }
        }
        dbars::meters(c);
        bump(c);
        let (p, m) = (c.vm.r16(c.st, a!(SEL_REC)), c.vm.r16(c.st, a!(SEL_MACH)));
        at = if pending(c, p, m, F_FOOD, 0xA) {
            0
        } else if pending(c, p, m, F_MINERALS, 0xE) {
            1
        } else if pending(c, p, m, F_FUEL, 0x10) {
            2
        } else if pending(c, p, m, F_ENERGY, 0x12) {
            3
        } else if pending(c, p, m, F_POP, 0x24) {
            4
        } else {
            return;
        };
    }
}

/// `planet f < 0x7530 && machine f != 0` — the tail's re-entry test
/// (file `0x34E5B`–`0x34EA8`).
fn pending(c: &mut Ctx, p: u16, m: u16, pf: usize, mf: u16) -> bool {
    c.vm.r16(c.st, p.wrapping_add(f!(pf))) < dxfer::CAP && c.vm.r16(c.st, m + mf) != 0
}

/// `cs:0x2B9B` — `scuttle`: the `0x7346` prompt, then the `0x1E2E`/
/// `0x7CBD` key loop until a mapped char — `Y` proceeds, anything
/// else reprints `0x68C7` and `ret`s (file `0x32B9B`–`0x32BCD`).
pub(super) fn scuttle(c: &mut Ctx) {
    text::print_str(c, 0x7346, 0xA, 0x52);
    c.host.key_flush();
    loop {
        c.host.pump_input(c.vm, c.st);
        let al = u8::try_from(c.host.svc(Call::KeyPop) & 0xFF).unwrap_or(0);
        if al == 0 {
            continue;
        }
        let Some(out) = keymap_out(c, al) else {
            continue;
        };
        if out != b'Y' {
            text::print_str(c, 0x68C7, 0xA, 0x52);
            return;
        }
        break;
    }
    raze(c);
}

/// The `0x32BB5` table scan — `Some(out)` on the first `0x7CBD`
/// entry whose code byte matches `al`.
fn keymap_out(c: &mut Ctx, al: u8) -> Option<u8> {
    for i in 0..u16::try_from(KEYMAP_N).unwrap_or(0) {
        let e = a!(KEYMAP).wrapping_add(i.wrapping_mul(2));
        if c.vm.r8(c.st, e) == al {
            return Some(c.vm.r8(c.st, e + 1));
        }
    }
    None
}

/// `0x32BCE`–`0x32CA8` — the confirmed half: `0x4E0A`, the fleet
/// slots' `+2` relinked from the machine to its host record (`and
/// [s+6],0xFD`), `+0xC` → `+0x36`, `+0x26` → `+0x32`, `+0x14` →
/// `+0x2A` (both `0x7530`-capped), the type `+0xE`/`+0x10` salvage,
/// the bay byte cleared, `+0x1E`/`[0x9180]` reset, `jmp 0x2493`.
fn raze(c: &mut Ctx) {
    unload_all(c);
    let m = c.vm.r16(c.st, a!(SEL_MACH));
    let hi = u16::from(c.vm.r8(c.st, m + f!(M_HOST)));
    let host =
        c.vm.r16(c.st, a!(REC_BASE))
            .wrapping_add(hi.wrapping_mul(a!(REC_STRIDE)));
    let mut si = a!(FLEET_BASE);
    for _ in 0..=FLEET_COUNT {
        if c.vm.r16(c.st, si + 2) == m {
            c.vm.w16(c.st, si + 2, host);
            let fl = c.vm.r8(c.st, si + 6);
            c.vm.w8(c.st, si + 6, fl & 0xFD);
        }
        si = si.wrapping_add(a!(FLEET_STRIDE));
    }
    let p = c.vm.r16(c.st, a!(REC_CUR));
    let refund = c.vm.r16(c.st, m + f!(M_TIMER));
    let cur = c.vm.r16(c.st, p.wrapping_add(f!(F_CREDITS)));
    c.vm.w16(
        c.st,
        p.wrapping_add(f!(F_CREDITS)),
        cur.wrapping_add(refund),
    );
    let fuel =
        c.vm.r16(c.st, p.wrapping_add(f!(F_FUEL)))
            .wrapping_add(c.vm.r16(c.st, m + f!(M_DEPOSIT)));
    c.vm.w16(c.st, p.wrapping_add(f!(F_FUEL)), fuel.min(dxfer::CAP));
    c.vm.w16(c.st, m + f!(M_DEPOSIT), 0);
    let pay = c.vm.r16(c.st, m + 0x14);
    if pay != 0xFFFE {
        let v = c.vm.r16(c.st, p.wrapping_add(f!(F_POP))).wrapping_add(pay);
        c.vm.w16(c.st, p.wrapping_add(f!(F_POP)), v.min(dxfer::CAP));
    }
    c.vm.w16(c.st, m + 0x14, 0);
    let trec = dmach::type_rec(c);
    for (tf, pf) in [(0xEu16, f!(F_ENERGY)), (0x10, f!(F_MINERALS))] {
        let v =
            c.vm.r16(c.st, p.wrapping_add(pf))
                .wrapping_add(c.vm.r16(c.st, trec + tf));
        c.vm.w16(c.st, p.wrapping_add(pf), v);
    }
    let bay = u8::try_from(m.wrapping_sub(a!(MACH_BASE)) / a!(MACH_STRIDE))
        .unwrap_or(0)
        .wrapping_add(1);
    let mut si = p.wrapping_add(f!(F_LINK_A));
    for _ in 0..2u8 {
        if c.vm.r8(c.st, si) == bay {
            break;
        }
        si = si.wrapping_add(4);
    }
    c.vm.w8(c.st, si, 0);
    c.vm.w8(c.st, m + f!(M_TYPE), 0);
    c.vm.w16(c.st, a!(SEL_MACH), 0);
    detail::body(c);
}
