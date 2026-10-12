//! The shell "launch" action — `cs:0x71E3` (records `0xDF33`, file
//! `0x371E3`–`0x373D8`): scans the `0x9491` unit table for a kind-`4`
//! slot into `[0x9150]` — none reports `0x7130`/`0x7123`; an in-transit
//! unit (`+0x1F & 1`) reports via the `0x7380` branch; otherwise the
//! `[0x9184]` kind gates (`1` → `0x7783`, `!= 4` → `0x70F2`) and the
//! kind-`4` bombardment flow runs the troop/distance counts, the
//! `0xE77D` rename editor and the commit (`0x72F0`–`0x3737F`).

use super::super::cells::IN_DONE;
use super::super::consts::{MACH_BASE, QUEUED_REC, REC_CUR, REINF_REC};
use super::super::vhost::Call;
use super::super::vops::{kind_of, Ctx};
use super::super::{dialog, namedit, status, text};
use super::machd::mach_host;
use super::snd;

/// `u16` casts for the `ds:` slot constants.
macro_rules! a {
    ($k:expr) => {
        u16::try_from($k).unwrap_or(0)
    };
}

/// Unit-record stride (`0x371FF`).
const MACH_STRIDE: u16 = 0x28;
/// Unit-table slot count (`cx = 0x1F` + the `jns` iteration).
const MACH_SLOTS: u8 = 0x20;
/// `0x8314` — the troop-count number cell.
const N_TROOPS: u16 = 0x8314;
/// `0x8318` — the distance number cell.
const N_DIST: u16 = 0x8318;
/// `[0x8206]` — the "launch queued" flag byte.
const LAUNCH_F: u16 = 0x8206;
/// Troops per `+0x1A` divisor (`bx = 0xFA`).
const TROOP_DIV: u16 = 0xFA;
/// The plural word cells `0x67C7`/`0x67CC`.
const S_ONE: u16 = 0x67C7;
/// `!= 1` plural word.
const S_MANY: u16 = 0x67CC;

/// `cs:0x71E3` — the launch button (file `0x371E3`–`0x3737F`).
pub fn fleet_launch(c: &mut Ctx) {
    snd(c, 3, 0xA);
    let Some(m) = find_kind4(c) else {
        text::enqueue(c, 0x7130);
        text::enqueue(c, 0x7123);
        return;
    };
    c.vm.w16(c.st, a!(REINF_REC), m);
    if c.vm.r8(c.st, m + 0x1F) & 1 != 0 {
        return transit(c, m);
    }
    let rec = c.vm.r16(c.st, a!(REC_CUR));
    match kind_of(c, rec) {
        1 => text::enqueue(c, 0x7783),
        4 => bombard(c, m, rec),
        _ => text::enqueue(c, 0x70F2),
    }
}

/// `0x371FF` — first unit slot with `+0x1E == 4` (the kind-`4` scan).
fn find_kind4(c: &mut Ctx) -> Option<u16> {
    (0..MACH_SLOTS)
        .map(|i| a!(MACH_BASE).wrapping_add(u16::from(i).wrapping_mul(MACH_STRIDE)))
        .find(|&m| c.vm.r8(c.st, m + 0x1E) == 4)
}

/// `0x7380` — the in-transit report: `+0x18 == 0` → the
/// `0x683A`/`0x8313`/plural/`0x6850`/`0x687C` chain, else the
/// `0x683A`/`0x8317`/plural/`0x6894`/`0x687C` one.
fn transit(c: &mut Ctx, m: u16) {
    let seq = if c.vm.r16(c.st, m + 0x18) == 0 {
        [0x683A, 0x8313, plur(c, N_TROOPS), 0x6850, 0x687C]
    } else {
        [0x683A, 0x8317, plur(c, N_DIST), 0x6894, 0x687C]
    };
    for s in seq {
        text::enqueue(c, s);
    }
}

/// `0x67C7`/`0x67CC` — the `1`/plural word for number cell `cell`.
fn plur(c: &mut Ctx, cell: u16) -> u16 {
    if c.vm.r16(c.st, cell) == 1 {
        S_ONE
    } else {
        S_MANY
    }
}

/// `0x7246` — troops `+0x1A / 0xFA` → `0x8314`, `|+0x28 − +0x21|` →
/// `0x8318`, the report enqueues, `0x2FE7` pump, `0xE77D` rename; a
/// cancel writes `"LIFELESS!"` and reports `0x7123`.
fn bombard(c: &mut Ctx, m: u16, rec: u16) {
    let troops = c.vm.r16(c.st, rec + 0x1A) / TROOP_DIV;
    c.vm.w16(c.st, N_TROOPS, troops);
    let d =
        c.vm.r8(c.st, rec + 0x28)
            .wrapping_sub(c.vm.r8(c.st, m + 0x21))
            .cast_signed();
    let d = if d < 0 { d.wrapping_neg() } else { d };
    c.vm.w16(c.st, N_DIST, u16::from_ne_bytes(i16::from(d).to_ne_bytes()));
    for s in [
        0x67A3,
        0x8317,
        plur(c, N_DIST),
        0x6812,
        0x8313,
        plur(c, N_TROOPS),
        0x67D2,
    ] {
        text::enqueue(c, s);
    }
    dialog::pump(c);
    namedit::prompt_edit(c, rec + 0xE, 0x2A, 0xB8, 8);
    if c.vm.r8(c.st, a!(IN_DONE)) == 0 {
        launch(c, m, rec);
    } else {
        lifeless(c, rec);
    }
}

/// `0x72C9` — overwrite the record name field with `"LIFELESS!"` and
/// enqueue `0x7123`.
fn lifeless(c: &mut Ctx, rec: u16) {
    let s = rec + 0xE;
    for (o, w) in [(0u16, 0x494Cu16), (2, 0x4546), (4, 0x454C), (6, 0x5353)] {
        c.vm.w16(c.st, s + o, w);
    }
    c.vm.w8(c.st, s + 8, 0x21);
    text::enqueue(c, 0x7123);
}

/// `0x72F0` — commit: unit `+0x1A`/`+0x22`/`+0x1F`/`+0x18` fields,
/// `[0x8206] = 0xFF`, the `0x5085` host's matching bay byte cleared,
/// `0x6865` queued, the record queued/named, `+0xC` kind byte → `1`,
/// `0x6AD2` + `0x5C97` panel refresh.
fn launch(c: &mut Ctx, m: u16, rec: u16) {
    let cur = c.vm.r16(c.st, a!(REC_CUR));
    c.vm.w16(c.st, m + 0x1A, cur);
    let (t, d) = (c.vm.r16(c.st, N_TROOPS), c.vm.r16(c.st, N_DIST));
    c.vm.w16(c.st, m + 0x22, t);
    c.vm.w8(c.st, m + 0x1F, 0xD);
    c.vm.w16(c.st, m + 0x18, d);
    c.vm.w8(c.st, LAUNCH_F, 0xFF);
    bay_clear(c, m);
    text::enqueue(c, 0x6865);
    c.vm.w16(c.st, a!(QUEUED_REC), rec);
    namedit::copy9(c, rec + 0xE, 0x8332);
    c.vm.w8(c.st, rec + 0xC, 1);
    status::sel_name(c);
    c.host.svc(Call::PlanetPanel);
}

/// `0x730F`–`0x37348` — clear the byte in the host's `+0x18/+0x1C`/
/// `+0x20` bay triple matching the unit's `index + 1` (unmatched falls
/// through to `+0x20`).
fn bay_clear(c: &mut Ctx, m: u16) {
    let bay = mach_host(c, m) + 0x18;
    let idx = u8::try_from(m.wrapping_sub(a!(MACH_BASE)) / MACH_STRIDE)
        .unwrap_or(0)
        .wrapping_add(1);
    let slot = if c.vm.r8(c.st, bay) == idx {
        bay
    } else if c.vm.r8(c.st, bay + 4) == idx {
        bay + 4
    } else {
        bay + 8
    };
    c.vm.w8(c.st, slot, 0);
}
