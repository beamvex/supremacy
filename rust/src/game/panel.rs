//! The machine-type info panel — `cs:0x3AE9` (file `0x33AE9`–`0x33C6C`),
//! run when `[0x91EF]` is set. Reprints every stat field of the
//! selected type record `[0x9154]`; several coordinates and which rows
//! print at all depend on `[0x91E4]` (difficulty).

use super::consts::{DIFFICULTY, MACH_BASE, MACH_COUNT, MACH_STRIDE, PANEL_REQ, TYPE_REC};
use super::mach::M_TYPE;
use super::num;
use super::text;
use super::vhost::Call;
use super::vops::Ctx;

/// `u16` casts for the `ds:` slot constants.
macro_rules! a16 {
    ($k:expr) => {
        u16::try_from($k).unwrap_or(0)
    };
}

/// The `cs:0x3AE9` body — clear the request and repaint the panel.
pub fn info_panel(c: &mut Ctx) {
    c.vm.w8(c.st, a16!(PANEL_REQ), 0);
    let t = c.vm.r16(c.st, a16!(TYPE_REC));
    let n = c.vm.r16(c.st, t + 0x14);
    text::print_str(c, n, 0x19, 0x98);
    let d = c.vm.r16(c.st, t + 0x2C);
    text::print_str(c, d, 0x12, 0x92);
    for f in [0x3CADu16, 0x3CDB, 0x3CF6] {
        c.host.svc(Call::Native(f));
    }
    rows_a(c, t);
    rows_b(c, t);
    rows_c(c, t);
}

/// Field `+6`, the mode-2 `+0x10`, the mode≠0 `+0xE` and the mode-2
/// `+0xA` row (file `0x33B17`–`0x33B9A`).
fn rows_a(c: &mut Ctx, t: u16) {
    let diff = c.vm.r8(c.st, a16!(DIFFICULTY));
    let y = if diff == 2 {
        0xB0
    } else if diff == 0 {
        0xB7
    } else {
        0xB3
    };
    num::num_pad(c, c.vm.r16(c.st, t + 6), 0x18, y);
    if diff == 2 {
        num::num_pad(c, c.vm.r16(c.st, t + 0x10), 0x18, 0xB7);
    }
    if diff != 0 {
        num::num_pad(
            c,
            c.vm.r16(c.st, t + 0xE),
            0x18,
            if diff == 2 { 0xBE } else { 0xBC },
        );
    }
    mid_row(c, t, diff);
}

/// The `+0xA` capacity row — `0xFFFE` prints the `ds:0x7325` "none"
/// string instead of the number (file `0x33B73`–`0x33B9A`).
fn mid_row(c: &mut Ctx, t: u16, diff: u8) {
    if diff != 2 {
        return;
    }
    let cap = c.vm.r16(c.st, t + 0xA);
    if cap == 0xFFFE {
        text::print_str(c, 0x7325, 0x2A, 0xB0);
    } else {
        num::num_pad(c, cap, 0x2C, 0xB0);
    }
}

/// The `+4` cost row and the mode≠0 `+0xC` stock row — `0xFFFF` prints
/// the `ds:0x731C` "none" string (file `0x33B9B`–`0x33BED`).
fn rows_b(c: &mut Ctx, t: u16) {
    let diff = c.vm.r8(c.st, a16!(DIFFICULTY));
    let y = if diff == 1 { 0xB3 } else { 0xB7 };
    num::num_pad(c, c.vm.r16(c.st, t + 4), 0x29, y);
    if diff == 0 {
        return;
    }
    let dx = if diff == 1 { 0xBC } else { 0xBE };
    let stock = c.vm.r16(c.st, t + 0xC);
    if stock == 0xFFFF {
        text::print_str(c, 0x731C, 0x29, dx);
    } else {
        num::num_pad(c, stock, 0x29, dx);
    }
}

/// The owned-count, mode-2 unit-cost and mode≠0 `+0x1E` rows (file
/// `0x33BEE`–`0x33C6B`).
fn rows_c(c: &mut Ctx, trec: u16) {
    let diff = c.vm.r8(c.st, a16!(DIFFICULTY));
    let kind = c.vm.r8(c.st, trec + 0x12);
    let row_y = if diff == 2 {
        0xB0
    } else if diff == 0 {
        0xB7
    } else {
        0xB3
    };
    let count = u16::from(count_type(c, kind));
    num::num_pad(c, count, 0x3D, row_y);
    if diff == 2 {
        total_cost(c, trec);
    }
    if diff != 0 {
        let dx = if diff == 1 { 0xBC } else { 0xBE };
        num::num_pad(c, c.vm.r16(c.st, trec + 0x1E), 0x3B, dx);
    }
}

/// Mode-2 row at `(0x3A,0xB7)` — `+0xC == 0xFFFF` prints `ds:0x7509`,
/// else `+0x13 · +0xC` truncated to 16 bits (file `0x33C19`–`0x33C4A`).
fn total_cost(c: &mut Ctx, t: u16) {
    let stock = c.vm.r16(c.st, t + 0xC);
    if stock == 0xFFFF {
        text::print_str(c, 0x7509, 0x3A, 0xB7);
        return;
    }
    let each = u16::from(c.vm.r8(c.st, t + 0x13));
    num::num_pad(c, stock.wrapping_mul(each), 0x3A, 0xB7);
}

/// `cs:0x6632` (file `0x36632`) — count machine records whose `M_TYPE`
/// equals `ty`; the asm scans all `0x20` entries, `ah` = count.
fn count_type(c: &mut Ctx, ty: u8) -> u8 {
    let mut n = 0u8;
    for i in 0..MACH_COUNT {
        let m = a16!(MACH_BASE) + i * a16!(MACH_STRIDE);
        if c.vm.r8(c.st, m + a16!(M_TYPE)) == ty {
            n = n.wrapping_add(1);
        }
    }
    n
}
