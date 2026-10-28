//! The orders-screen controls — `cs:0x32DB`/`0x32FB`/`0x331B`
//! `order_mode 0/1/2` with the `0x3357` machine-list tail,
//! `cs:0x71C4` `order_mode_redraw`, the `cs:0x36BD`/`0x36C4`
//! tax-rate arrows (`page_prev`/`page_next`), `cs:0x374B`
//! `order_collect` over the `cs:0x37C7` census, and `cs:0x37EA`
//! `order_rename` (files `0x332DB`–`0x33437`, `0x371C4`–`0x371E2`,
//! `0x336BD`–`0x338C1`).

use super::super::cells::{IN_DONE, LINE_MODE, OVERLAY_F};
use super::super::consts::{
    ARMED, DIRTY0, FRAME, MACH_BASE, MACH_COUNT, MACH_STRIDE, NAME_BUF, ORD_MODE, REC_BASE,
    REC_COUNT, REC_STRIDE, REP_AT, REP_DIR, REP_DL, REP_IN, SEL_REC,
};
use super::super::mach::{M_FLAGS, M_HOST, M_TYPE};
use super::super::rec::{F_CREDITS, F_KIND, F_NAME, F_SERIAL, F_TAX};
use super::super::vhost::Call;
use super::super::vops::{faction_rec, kind_of, set_bits, Ctx};
use super::super::{dialog, linein, namedit, text};
use super::{galmsg, ordlist};

macro_rules! a {
    ($k:expr) => {
        u16::try_from($k).unwrap_or(0)
    };
}
macro_rules! f {
    ($k:expr) => {
        u16::try_from($k).unwrap_or(0)
    };
}

/// List origin of the machine block and its wrap (file `0x3337F`).
const LIST_X: u16 = 0x30;
/// Column-x wrap — `bx == 0x4E` starts the next `dx` band (file
/// `0x333EE`).
const LIST_WRAP: u16 = 0x4E;
/// The `0x68B1` blank-row print for unused slots.
const S_BLANK: u16 = 0x68B1;

/// `cs:0x32DB`/`0x32FB`/`0x331B` — `order_mode 0/1/2`: the sound
/// block then the mode inner (file `0x332DB`–`0x3333A`).
pub fn order_mode(c: &mut Ctx, mode: u8) {
    dialog::snd_block(c);
    mode_inner(c, mode);
}

/// `cs:0x71C4` — re-run the `[0x91CB]` mode inner (file
/// `0x371C4`–`0x371E2`).
pub fn mode_redraw(c: &mut Ctx) {
    let m = c.vm.r8(c.st, a!(ORD_MODE));
    if m <= 2 {
        mode_inner(c, m);
    }
}

/// The `0x333B`/`0x3345`/`0x334F` inners — `[0x91CB] = mode` + the
/// label string, then the `0x3357` machine-list tail.
pub fn mode_inner(c: &mut Ctx, mode: u8) {
    let s = match mode {
        2 => 0x75A5,
        1 => 0x758F,
        _ => 0x7579,
    };
    c.vm.w8(c.st, a!(ORD_MODE), mode);
    machine_list(c, s);
}

/// `0x3357` — the mode label + selected name at `(0x30,0x75)`, then
/// the `0x9491` machine walk filtered by `[0x91CB]` (mode `0` wants
/// `+0x1F & 0x10`, mode `1` wants `& 4`, mode `2` wants neither) on
/// the selected planet's serial; 30-entry columns, 6 shown max, the
/// rest blank-padded (file `0x33357`–`0x33437`).
fn machine_list(c: &mut Ctx, s: u16) {
    let sel = c.vm.r16(c.st, a!(SEL_REC));
    if sel != 0 {
        let bx = text::print_str(c, s, 0x30, 0x75);
        text::print_str(c, sel.wrapping_add(f!(F_NAME)), bx, 0x75);
    }
    let serial = c.vm.r8(c.st, sel.wrapping_add(f!(F_SERIAL)));
    let mode = c.vm.r8(c.st, a!(ORD_MODE));
    let (mut bx, mut dx, mut bp) = (LIST_X, 0x9Bu16, 0u16);
    for i in 0..MACH_COUNT {
        let m = a!(MACH_BASE).wrapping_add(i.wrapping_mul(a!(MACH_STRIDE)));
        if shown(c, m, serial, mode) {
            machine_row(c, m, &mut bx, dx);
            bp = bp.wrapping_add(1);
            if bump(&mut bx, &mut dx) && bp == 6 {
                return;
            }
        }
    }
    pad(c, bp, bx, dx);
}

/// The `0x33388` filters — in-use (`+0x1E`), host serial match
/// (`+0x21`), then the mode flag test.
fn shown(c: &mut Ctx, m: u16, serial: u8, mode: u8) -> bool {
    if c.vm.r8(c.st, m.wrapping_add(f!(M_TYPE))) == 0 {
        return false;
    }
    if c.vm.r8(c.st, m.wrapping_add(f!(M_HOST))) != serial {
        return false;
    }
    let fl = c.vm.r8(c.st, m.wrapping_add(f!(M_FLAGS)));
    match mode {
        2 => fl & 0x14 == 0,
        1 => fl & 4 != 0,
        _ => fl & 0x10 != 0,
    }
}

/// `0x333BD` — one listed machine: the `si` name print at `(bx,dx)`
/// then the `+0x1E − 1`-indexed `[0x128C]` icon (`0x15C`-based; the
/// descriptor patch is folded into the `Call::Image`).
fn machine_row(c: &mut Ctx, m: u16, bx: &mut u16, dx: u16) {
    text::print_str(c, m, *bx, dx);
    let ty = c.vm.r8(c.st, m.wrapping_add(f!(M_TYPE)));
    let icon = 0x15Cu16.wrapping_add(u16::from(ty).wrapping_sub(1));
    c.host.svc(Call::Image(icon));
    *bx = bx.wrapping_add(1);
}

/// `0x333ED` — the per-entry advance: `bp` counted by the caller,
/// `bx` wraps at `0x4E` to the next `dx + 0x23` band; `true` on wrap.
fn bump(bx: &mut u16, dx: &mut u16) -> bool {
    if *bx != LIST_WRAP {
        return false;
    }
    *bx = LIST_X;
    *dx = dx.wrapping_add(0x23);
    true
}

/// `0x33404` — fill the unused slots with the `0x68B1` blank + icon
/// `0x163`: `cx = 5 − bp` do-while, so `6 − bp` rows and always one
/// when `bp ≥ 6` (file `0x33404`–`0x33436`).
fn pad(c: &mut Ctx, bp: u16, mut bx: u16, mut dx: u16) {
    for _ in 0..6u16.saturating_sub(bp).max(1) {
        text::print_str(c, S_BLANK, bx, dx);
        c.host.svc(Call::Image(0x163));
        bx = bx.wrapping_add(1);
        bump(&mut bx, &mut dx);
    }
}

/// `cs:0x36BD`/`0x36C4` — the tax-rate arrows: `[0x91F8] = dir`
/// (`0xFF` down / `0` up), then the `[0x91F5]`/`[0x91F6]`/`[0x91F7]`
/// repeat accelerator, stepping `[si + 0x1E]` clamped `0..=0x64` and
/// arming the `0x100` reprint (file `0x336BD`–`0x3373B`).
pub fn order_page(c: &mut Ctx, dir: u8) {
    c.vm.w8(c.st, a!(REP_DIR), dir);
    let rec = c.vm.r16(c.st, a!(SEL_REC));
    if rec == 0 {
        return;
    }
    c.vm.w8(c.st, a!(ARMED), 0);
    if !accel(c) {
        return;
    }
    let v = c.vm.r8(c.st, rec.wrapping_add(f!(F_TAX)));
    if dir != 0 {
        if v == 0 {
            return;
        }
        c.vm.w8(c.st, rec.wrapping_add(f!(F_TAX)), v - 1);
    } else {
        if v == 0x64 {
            return;
        }
        c.vm.w8(c.st, rec.wrapping_add(f!(F_TAX)), v + 1);
    }
    set_bits(c, a!(DIRTY0), 0x100);
}

/// `0x336D6`–`0x3370E` — the accelerator: consecutive-frame presses
/// keep the `[0x91F6]` ramp; otherwise it reseeds `0xC`. A live
/// `[0x91F7]` delay postpones the step (`false`). The detail fuel
/// slider's `0x34D28`–`0x34D63` is the same cell dance.
pub(super) fn accel(c: &mut Ctx) -> bool {
    let last = c.vm.r8(c.st, a!(REP_AT)).wrapping_add(1);
    let now = c.vm.r8(c.st, a!(FRAME));
    c.vm.w8(c.st, a!(REP_AT), now);
    if last != now {
        c.vm.w8(c.st, a!(REP_IN), 0xC);
        c.vm.w8(c.st, a!(REP_DL), 0);
    }
    let dl = c.vm.r8(c.st, a!(REP_DL));
    if dl != 0 {
        c.vm.w8(c.st, a!(REP_DL), dl - 1);
        return false;
    }
    let iv = c.vm.r8(c.st, a!(REP_IN));
    let iv = if iv != 0 { iv - 1 } else { iv };
    c.vm.w8(c.st, a!(REP_IN), iv);
    c.vm.w8(c.st, a!(REP_DL), iv);
    true
}

/// `cs:0x374B` — `order_collect`: `< 2` colonies → `0x7C13` at
/// `(0xB,0x61)`; else `0x73EB` + every kind-`0xA` record's
/// `+0x36`/`+0x38` into the faction record and the `0x4` credits
/// arm. Both paths blip `al=0xC`/`cl=0x16`, `0x6F2D` wait, `0x38C2`
/// (file `0x3374B`–`0x337C6`).
pub fn order_collect(c: &mut Ctx) {
    if count_kinds(c).0 < 2 {
        text::print_str(c, 0x7C13, 0xB, 0x61);
    } else {
        text::print_str(c, 0x73EB, 0xD, 0x61);
        collect(c);
        set_bits(c, a!(DIRTY0), 4);
    }
    super::snd(c, 0xC, 0x16);
    galmsg::wait(c);
    ordlist::hint(c);
}

/// `0x33761` — lump `+0x36/0x38` of each kind-`0xA` record into the
/// faction record (`0x5E3A`), zeroing the colonies (file
/// `0x3376D`–`0x3379E`).
fn collect(c: &mut Ctx) {
    let di = faction_rec(c);
    let n = c.vm.r16(c.st, a!(REC_COUNT));
    let base = c.vm.r16(c.st, a!(REC_BASE));
    for i in 0..n {
        let si = base.wrapping_add(i.wrapping_mul(a!(REC_STRIDE)));
        if c.vm.r16(c.st, si.wrapping_add(f!(F_KIND))) != 0xA {
            continue;
        }
        let lo = c.vm.r16(c.st, si.wrapping_add(f!(F_CREDITS)));
        let hi = c.vm.r16(c.st, si.wrapping_add(f!(F_CREDITS) + 2));
        let sum =
            c.vm.r16(c.st, di.wrapping_add(f!(F_CREDITS)))
                .overflowing_add(lo);
        c.vm.w16(c.st, di.wrapping_add(f!(F_CREDITS)), sum.0);
        let h = c.vm.r16(c.st, di.wrapping_add(f!(F_CREDITS) + 2));
        c.vm.w16(
            c.st,
            di.wrapping_add(f!(F_CREDITS) + 2),
            h.wrapping_add(hi).wrapping_add(u16::from(sum.1)),
        );
        c.vm.w16(c.st, si.wrapping_add(f!(F_CREDITS)), 0);
        c.vm.w16(c.st, si.wrapping_add(f!(F_CREDITS) + 2), 0);
    }
}

/// `cs:0x37C7` — the census: `bp` = kind-`0xA` count, `ax` = kind-7
/// count over `[0x91B8]`+1 records (file `0x337C7`–`0x337E9`).
fn count_kinds(c: &mut Ctx) -> (u16, u16) {
    let n = c.vm.r16(c.st, a!(REC_COUNT));
    let base = c.vm.r16(c.st, a!(REC_BASE));
    let (mut bp, mut ax) = (0u16, 0u16);
    for i in 0..=n {
        let k = c.vm.r16(
            c.st,
            base.wrapping_add(i.wrapping_mul(a!(REC_STRIDE))) + f!(F_KIND),
        );
        if k == 0xA {
            bp = bp.wrapping_add(1);
        }
        if k == 7 {
            ax = ax.wrapping_add(1);
        }
    }
    (bp, ax)
}

/// `cs:0x37EA` — `order_rename`: guards the selection (`0x8594`
/// blip on refuse), prints `0x7513`, stages `+0xE` at `0x838E`, runs
/// the `0xE652` inline edit bracketed in `[0x9CC5]`; Enter
/// (`[0x4FCF] == 0`) arms the name repaint + `0x6F53`, Esc restores
/// the name + `0x6B24` + `0x6F2D`; the `0x38C2` tail (file
/// `0x337EA`–`0x338C1`).
pub fn order_rename(c: &mut Ctx) {
    let rec = c.vm.r16(c.st, a!(SEL_REC));
    if rec == 0 || kind_of(c, rec) != 0xA {
        return dialog::snd_block(c);
    }
    dialog::snd_block(c);
    text::print_str(c, 0x7513, 0xA, 0x61);
    let name = rec.wrapping_add(f!(F_NAME));
    namedit::copy9(c, name, a!(NAME_BUF));
    c.vm.w8(c.st, a!(LINE_MODE), 1);
    linein::unnamed_edit(c, name, 0x1A, 0x61, 8);
    c.vm.w8(c.st, a!(LINE_MODE), 0);
    rename_done(c, name);
}

/// `0x33864` — the accept/cancel tail of `order_rename`.
fn rename_done(c: &mut Ctx, name: u16) {
    if c.vm.r8(c.st, a!(IN_DONE)) == 0 {
        set_bits(c, a!(DIRTY0), 0x200);
        ordlist::planet_list_paint(c);
    } else {
        namedit::copy9(c, a!(NAME_BUF), name);
        c.vm.w8(c.st, a!(OVERLAY_F), 0xFF);
        text::print_str(c, 0x6B24, 0x1A, 0x61);
        c.vm.w8(c.st, a!(OVERLAY_F), 0);
        galmsg::wait(c);
    }
    ordlist::hint(c);
}
