//! The oval hover region — `cs:0x8DC6` (file `0x38DC6`–`0x38E0F`),
//! run once per shell frame. The cursor is tested against a
//! symmetric region centred on `x = 0x50` spanning `y ∈ [0x40,0x9B)`:
//! each y quantises to a row base (`0x3F` below `0x50`, `0x90` above,
//! `bx = y` under `0x40`), and `ds:[0xA8 + base − y]` gives the row's
//! half-width (runtime-loaded table; the shipped image has zeros).
//! Inside → the frame's generic handler is skipped; outside →
//! `jmp [0x1278]`.

use super::cells::OVAL_TAB;
use super::consts::{CUR_X, CUR_Y};
use super::vhost::Call;
use super::vops::Ctx;

/// `u16` casts for the `ds:` slot constants.
macro_rules! a {
    ($k:expr) => {
        u16::try_from($k).unwrap_or(0)
    };
}

/// Top of the oval region (file `0x38DD2`).
const Y_HI: u16 = 0x9B;
/// First quantised band (file `0x38DD7`).
const Y_MID: u16 = 0x40;
/// Centre line of the oval (file `0x38DE0`/`0x38DE5`).
const Y_CEN: u16 = 0x50;

/// `cs:0x8DC6` — swallow inside the oval, pass through to `[0x1278]`
/// outside (file `0x38DC6`–`0x38E0F`).
pub fn hover_check(c: &mut Ctx) {
    if !inside(c) {
        c.host.svc(Call::Slot(0x1278, 0));
    }
}

/// The region test (file `0x38DC6`–`0x38E0B`): index into the
/// half-width table is `bx + 0xA8` where `bx` is `y` (under `0x40`),
/// `0x3F − y` (`0x40..=0x50`) or `0x90 − y` (`0x51..0x9B`).
fn inside(c: &mut Ctx) -> bool {
    let y = c.vm.r16(c.st, a!(CUR_Y));
    if y >= Y_HI {
        return false;
    }
    let bx = if y >= Y_MID {
        let q: u16 = if y > Y_CEN { 0x90 } else { 0x3F };
        q.wrapping_sub(y)
    } else {
        y
    };
    let w = u16::from(c.vm.r8(c.st, bx.wrapping_add(a!(OVAL_TAB))));
    // `cl`/`dl` are 8-bit wrap: `0x50 ∓ w`, `ch`/`dh` cleared.
    let (lo, hi) = (Y_CEN.wrapping_sub(w) & 0xFF, (Y_CEN + w) & 0xFF);
    let x = c.vm.r16(c.st, a!(CUR_X));
    x <= hi && x.wrapping_add(8) >= lo
}
