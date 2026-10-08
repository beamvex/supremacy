//! The decimal printers — `cs:0x686C` (`u16`, space-padded to 5),
//! `cs:0x698F` (`u16`, no padding), `cs:0x68E0` (`bp:ax` `u32`, padded
//! to 8). All emit `0x68D6` char puts at `(bx·4px, dx)`, `bx += 1` per
//! char (files `0x3686C`–`0x36A7A`).
//!
//! Structure per printer: a pad zone of the leading decades (`0x20`
//! spaces while the value is still below the decade — skipped entirely
//! in the `0x698F` variant), then the forced zone — the divisor chain
//! `0x68BD`/`0x69C6` (`u16` tail, units through the landed decade for
//! `0x686C`; the `u32` printer's chain always covers `1000..1`).

use super::text::put;
use super::vops::Ctx;

/// The `u16` decades — `0x686C` pads the first four and lands
/// unconditionally on the units divisor.
const DEC16: [u32; 5] = [10_000, 1_000, 100, 10, 1];
/// The `u32` decades — `0x68E0` pads the first four; the `0x6A56`
/// `10_000` position and below always print.
const DEC32: [u32; 8] = [10_000_000, 1_000_000, 100_000, 10_000, 1_000, 100, 10, 1];

/// `cs:0x686C` — `u16` padded to a 5-column field (file `0x3686C`–
/// `0x368BA`). Returns `bx` advanced past the field.
pub fn num_pad(c: &mut Ctx, v: u16, mut bx: u16, dx: u16) -> u16 {
    field(c, u32::from(v), &DEC16, true, 1, &mut bx, dx);
    bx
}

/// `cs:0x698F` — `u16` with no padding: emits from the first live
/// decade only, `v = 0` still prints `'0'` (file `0x3698F`–`0x369A4`).
pub fn num(c: &mut Ctx, v: u16, mut bx: u16, dx: u16) -> u16 {
    field(c, u32::from(v), &DEC16, false, 1, &mut bx, dx);
    bx
}

/// `cs:0x68E0` — `bp:ax` `u32` padded to an 8-column field; the last
/// four decades print unconditionally (file `0x368E0`–`0x36A7A`).
pub fn num32(c: &mut Ctx, v: u32, mut bx: u16, dx: u16) -> u16 {
    field(c, v, &DEC32, true, 4, &mut bx, dx);
    bx
}

/// The decade walk — the asm's per-decade `sub`/`sbb` loops are
/// `v / d` here; once a digit emits every later position emits too.
fn field(c: &mut Ctx, v: u32, dec: &[u32], pad: bool, forced: usize, bx: &mut u16, dx: u16) {
    let cut = dec.len() - forced;
    let mut live = false;
    for (i, &d) in dec.iter().enumerate() {
        if !live && i < cut && v < d {
            if pad {
                put(c, 0x20, *bx, dx);
                *bx = bx.wrapping_add(1);
            }
            continue;
        }
        live = true;
        let digit = u8::try_from(v / d % 10).unwrap_or(0);
        put(c, 0x30 + digit, *bx, dx);
        *bx = bx.wrapping_add(1);
    }
}
