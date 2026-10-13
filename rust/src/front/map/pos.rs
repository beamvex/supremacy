//! The `map_draw` per-record position math and the colony-blink kind
//! addend (file `0x2F2E2`–`0x2F380`).

use super::{cs_word, get, Mark};
use crate::game::{State, Vm};

/// `cs:` base of the map scale table.
const SCALE: u16 = 0x1284;
/// `ds:` frame counter — the colony-blink phase.
const FRAME: u16 = 0x91D4;

/// The map position + save — `(cell, blink quadrant)` (file
/// `0x2F2E2`–`0x2F31C`; the quadrant is pre-`3→1` remap).
pub fn pos(
    img: &[u8],
    vm: &Vm,
    st: &State,
    si: u16,
    scroll: u8,
    m: &mut Mark,
    buf: &[u8],
) -> (u16, u8) {
    let ang = vm.r16(st, si + 6);
    let ph = u8::try_from(ang & 0xFF).unwrap_or(0).wrapping_add(scroll);
    let sin = cs_word(img, ((ang & 0xFF00) | u16::from(ph)).wrapping_add(SCALE)).cast_signed();
    let prod = i64::from(sin) * i64::from(vm.r16(st, si + 8).cast_signed());
    let x = u16::try_from(((prod << 2) >> 16) & 0xFFFF).unwrap_or(0);
    let y = vm.r16(st, si + 0xA).wrapping_add(0x40).wrapping_mul(80);
    m.cell = y.wrapping_add(x).wrapping_add(0x50);
    m.under = get(buf, m.cell);
    (m.cell, ph >> 6)
}

/// The marker byte's kind addend — `kind ^ 0xD` while the blink window
/// is open on an occupied colony, else `kind` (file
/// `0x2F34E`–`0x2F380`).
pub fn blink(vm: &Vm, st: &State, si: u16) -> u8 {
    let kind = vm.r16(st, si + 0xC);
    let k = u8::try_from(kind & 0xFF).unwrap_or(0);
    let live = kind != 1
        && kind != 4
        && vm.r16(st, si + 0x26) != 0
        && vm.r16(st, si + 0x22) != 0
        && (vm.r8(st, FRAME) & 0xF) >= 8;
    if live {
        k ^ 0xD
    } else {
        k
    }
}
