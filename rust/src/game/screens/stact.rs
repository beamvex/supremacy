//! The uimode-6 status screen's bay/cell actions — the `0x9D60`
//! list's `bay_pick` bay→held picks (`cs:0x5B16`/`0x5B1F`/`0x5B28`),
//! `cell_load` held→bay docks (`cs:0x5BA2`..`0x5BC7`) and `cell_sel`
//! select toggles (`cs:0x5ABE`..`0x5AE3`), the `0x59EB` `ds:0x5017`
//! held-unit table rebuild and the `0x5B3C` message tail (files
//! `0x35ABE`–`0x35C1D`).

use super::super::consts::{MACH_BASE, MACH_COUNT, MACH_STRIDE, REC_CUR};
use super::super::mach::{M_FLAGS, M_HOST, M_TYPE};
use super::super::rec::{F_LINK_A, F_LINK_B, F_LINK_C, F_SERIAL};
use super::super::text;
use super::super::vhost::Call;
use super::super::vops::Ctx;
use super::{dfield, galmsg};

/// `u16` casts for the `ds:`/record constants.
macro_rules! a {
    ($k:expr) => {
        u16::try_from($k).unwrap_or(0)
    };
}

/// `ds:` base of the six held-unit dwords `0x59EB` fills.
const CELLS: u16 = 0x5017;
/// `[0x91DD]` — the post-change redraw latch `bay_pick` arms.
const ANIM: u16 = 0x91DD;
/// The `0x68B1` blank and `0x69A7` "RUNNING" cell strings.
const S_BLANK: u16 = 0x68B1;
/// `0x69A7` — the `0x20`-flag "RUNNING" word.
const S_RUN: u16 = 0x69A7;
/// Crew field (`+0x14`) `cell_sel` gates on — `0x72EB` refuse.
const M_CREW: u16 = 0x14;

/// The `0x9D60` records' `+0xC` words — both windows' forms.
/// Anything else drops to the `Call::Native` sink.
pub(super) fn dispatch(c: &mut Ctx, t: u16) {
    match t {
        0x5B16 | 0xC866 => bay_pick(c, a!(F_LINK_A)),
        0x5B1F | 0xC86F => bay_pick(c, a!(F_LINK_B)),
        0x5B28 | 0xC878 => bay_pick(c, a!(F_LINK_C)),
        0x5BA2 | 0xC8F2 => cell_load(c, 0),
        0x5BA7 | 0xC8F7 => cell_load(c, 1),
        0x5BAF | 0xC8FF => cell_load(c, 2),
        0x5BB7 | 0xC907 => cell_load(c, 3),
        0x5BBF | 0xC90F => cell_load(c, 4),
        0x5BC7 | 0xC917 => cell_load(c, 5),
        0x5ABE | 0xC80E => cell_sel(c, 0),
        0x5AC3 | 0xC813 => cell_sel(c, 1),
        0x5ACB | 0xC81B => cell_sel(c, 2),
        0x5AD3 | 0xC823 => cell_sel(c, 3),
        0x5ADB | 0xC82B => cell_sel(c, 4),
        0x5AE3 | 0xC833 => cell_sel(c, 5),
        _ => drop(c.host.svc(Call::Native(t))),
    }
}

/// `cs:0x5B16`/`0x5B1F`/`0x5B28` — `bay_pick 1/2/3`: the `[0x9184]+f`
/// bay's machine leaves the bay `+0x1F | 0x10` into the `0x5017`
/// held cells; empty takes the `0x69B1` refuse, a full cell row a
/// silent `ret` (file `0x35B16`–`0x35BA1`).
fn bay_pick(c: &mut Ctx, f: u16) {
    let rec = c.vm.r16(c.st, a!(REC_CUR));
    let b = c.vm.r8(c.st, rec.wrapping_add(f));
    if b == 0 {
        return refuse(c, 0x69B1, 0x17);
    }
    if c.vm.r16(c.st, CELLS + 0x14) != 0 {
        return;
    }
    c.vm.w8(c.st, rec.wrapping_add(f), 0);
    let m = a!(MACH_BASE).wrapping_add(u16::from(b).wrapping_sub(1).wrapping_mul(a!(MACH_STRIDE)));
    let fl = c.vm.r8(c.st, m + a!(M_FLAGS)) | 0x10;
    c.vm.w8(c.st, m + a!(M_FLAGS), fl);
    c.vm.w8(c.st, ANIM, 0xFF);
    cells(c);
    dfield::bays_status(c);
}

/// `cs:0x5BA2`+4k — `cell_load k`: held cell `k`'s unit re-docks
/// into `[0x9184]`'s first free bay (`+0x1F & 0xCF`); all bays full
/// takes the `0x7215` refuse (file `0x35BA2`–`0x35C1D`).
fn cell_load(c: &mut Ctx, k: u16) {
    let unit = c.vm.r16(c.st, CELLS.wrapping_add(k.wrapping_mul(4)));
    if unit == 0 {
        return;
    }
    let rec = c.vm.r16(c.st, a!(REC_CUR));
    let bay = [a!(F_LINK_A), a!(F_LINK_B), a!(F_LINK_C)]
        .into_iter()
        .find(|&f| c.vm.r8(c.st, rec.wrapping_add(f)) == 0);
    let Some(f) = bay else {
        return refuse(c, 0x7215, 0xB);
    };
    let idx = unit.wrapping_sub(a!(MACH_BASE)) / a!(MACH_STRIDE) + 1;
    c.vm.w8(c.st, rec.wrapping_add(f), u8::try_from(idx).unwrap_or(0));
    let fl = c.vm.r8(c.st, unit + a!(M_FLAGS)) & 0xCF;
    c.vm.w8(c.st, unit + a!(M_FLAGS), fl);
    cells(c);
    dfield::bays_status(c);
}

/// `cs:0x5ABE`+4k — `cell_sel k`: a kind `6`/`7` held unit with a
/// crew (`+0x14`) toggles `+0x1F` `0x20` and rebuilds; uncrewed takes
/// the `0x72EB` refuse, other kinds a silent `ret` (file `0x35ABE`–
/// `0x35B15`).
fn cell_sel(c: &mut Ctx, k: u16) {
    let u = c.vm.r16(c.st, CELLS.wrapping_add(k.wrapping_mul(4)));
    if u == 0 {
        return;
    }
    let ty = c.vm.r8(c.st, u + a!(M_TYPE));
    if ty != 6 && ty != 7 {
        return;
    }
    if c.vm.r16(c.st, u + M_CREW) == 0 {
        return refuse(c, 0x72EB, 0x11);
    }
    let v = c.vm.r8(c.st, u + a!(M_FLAGS)) ^ 0x20;
    c.vm.w8(c.st, u + a!(M_FLAGS), v);
    cells(c);
}

/// `cs:0x59EB` — rebuild `ds:0x5017`: the `0x9491` walk's held
/// (`+0x1F & 0x10`) in-use units hosted on `[0x9184]+0x28` pack into
/// the six cells with name + `0x15C`-base icon + `0x20`-flag
/// RUNNING/blank; the rest blank-pad (file `0x359EB`–`0x35ABD`).
pub(super) fn cells(c: &mut Ctx) {
    for i in 0..6u16 {
        c.vm.w16(c.st, CELLS + i * 4, 0);
        c.vm.w16(c.st, CELLS + i * 4 + 2, 0);
    }
    let rec = c.vm.r16(c.st, a!(REC_CUR));
    let serial = c.vm.r8(c.st, rec.wrapping_add(a!(F_SERIAL)));
    let (mut di, mut bp, mut bx) = (CELLS, 0u16, 0x13u16);
    for i in 0..MACH_COUNT {
        let m = a!(MACH_BASE).wrapping_add(i.wrapping_mul(a!(MACH_STRIDE)));
        if held(c, m, serial) {
            c.vm.w16(c.st, di, m);
            cell_row(c, m, &mut bx, 0x9B);
            di = di.wrapping_add(4);
            bp = bp.wrapping_add(1);
        }
    }
    pad(c, bp, bx, 0x9B);
}

/// The `0x35A1B` filters — in-use (`+0x1E`), serial match (`+0x21`),
/// held (`+0x1F & 0x10`).
fn held(c: &mut Ctx, m: u16, serial: u8) -> bool {
    c.vm.r8(c.st, m + a!(M_TYPE)) != 0
        && c.vm.r8(c.st, m + a!(M_HOST)) == serial
        && c.vm.r8(c.st, m + a!(M_FLAGS)) & 0x10 != 0
}

/// `0x35A2C` — one cell: `si` name, the `+0x1E − 1`-indexed
/// `0x15C`-base icon, then the `0x20`-flag ON/blank a row down at
/// `bx − 9`; `bx` resumes past it (file `0x35A2C`–`0x35A7B`).
fn cell_row(c: &mut Ctx, m: u16, bx: &mut u16, dx: u16) {
    let end = text::print_str(c, m, *bx, dx);
    let ty = c.vm.r8(c.st, m + a!(M_TYPE));
    c.host.svc(Call::Image(
        0x15Cu16.wrapping_add(u16::from(ty).wrapping_sub(1)),
    ));
    let s = if c.vm.r8(c.st, m + a!(M_FLAGS)) & 0x20 != 0 {
        S_RUN
    } else {
        S_BLANK
    };
    let end = text::print_str(c, s, end.wrapping_sub(9), dx + 8);
    *bx = end.wrapping_add(1);
}

/// `0x35A82` — the `6 − bp` blank + `0x163`-icon pad rows.
fn pad(c: &mut Ctx, bp: u16, mut bx: u16, dx: u16) {
    for _ in bp..6 {
        let end = text::print_str(c, S_BLANK, bx, dx);
        c.host.svc(Call::Image(0x163));
        let end = text::print_str(c, S_BLANK, end.wrapping_sub(9), dx + 8);
        bx = end.wrapping_add(1);
    }
}

/// `0x5B3C` — the shared message tail: the `0x4FE0` header erase at
/// `(4,0xB4)`, `si` at `(bx,0xB4)`, the `0x6F2D` wait, then the
/// `0x7BB8` header reprint (file `0x35B3C`–`0x35B6E`).
fn refuse(c: &mut Ctx, s: u16, bx: u16) {
    text::print_str(c, 0x4FE0, 4, 0xB4);
    text::print_str(c, s, bx, 0xB4);
    galmsg::wait(c);
    text::print_str(c, 0x7BB8, 4, 0xB4);
}
