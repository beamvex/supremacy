//! Machine/station ops for the event VM — `cs:0x7ADD`, `0x7AF1`,
//! `0x7BAB`, `0x7C16`, `0x7C30`.

use super::consts::{
    DIRTY0, FLEET_BASE, FLEET_COUNT, FLEET_STRIDE, MACH_BASE, MACH_COUNT, MACH_STRIDE, SEL_MACH,
    SEL_REC,
};
use super::mach::{M_ACC, M_FLAGS, M_HOST, M_TIMER, M_TYPE};
use super::vm::Flow;
use super::vops::{set_bits, uimode, unlink_ships, Ctx, R_MACH, R_SEL};

fn mach(i: u16) -> u16 {
    u16::try_from(MACH_BASE).unwrap_or(0) + i * u16::try_from(MACH_STRIDE).unwrap_or(0)
}

/// `cs:0x7ADD` — clear flag `0x20` on all `0x20` machines.
pub fn unhide(c: &mut Ctx) -> Flow {
    for i in 0..MACH_COUNT {
        let a = mach(i) + u16::try_from(M_FLAGS).unwrap_or(0);
        c.vm.w8(c.st, a, c.vm.r8(c.st, a) & !0x20);
    }
    Flow::Cont
}

/// `cs:0x7AF1` — raze every machine on the selected planet: unlink its
/// ships, clear its type, and clear the matching post-fleet machine-
/// link bytes. Suspends when the selection is the current record or a
/// UI-mode-1/selected-machine abort hits.
pub fn raze(c: &mut Ctx) -> Flow {
    let sel = c.vm.r16(c.st, R_SEL);
    if sel == c.vm.r16(c.st, u16::try_from(SEL_REC).unwrap_or(0)) {
        return Flow::Ret;
    }
    let serial = c.vm.r8(c.st, sel + 0x28);
    for i in 0..MACH_COUNT {
        if !raze_one(c, mach(i), serial, i + 1) {
            return Flow::Ret;
        }
    }
    Flow::Cont
}

/// One machine of the `0x7AF1` pass — `false` on the suspend paths.
fn raze_one(c: &mut Ctx, m: u16, serial: u8, idx: u16) -> bool {
    let flags = c.vm.r8(c.st, m + u16::try_from(M_FLAGS).unwrap_or(0));
    let host = c.vm.r8(c.st, m + u16::try_from(M_HOST).unwrap_or(0));
    if host != serial || ((flags & 0x10) == 0 && (flags & 0x4) != 0) {
        return true;
    }
    let m_sel = u16::try_from(SEL_MACH).unwrap_or(0);
    if uimode(c) != 0 && uimode(c) != 5 && m == c.vm.r16(c.st, m_sel) {
        return false;
    }
    if !unlink_ships(c, m) {
        return false;
    }
    c.vm.w8(c.st, m + u16::try_from(M_TYPE).unwrap_or(0), 0);
    clear_tail(c, idx);
    true
}

/// The post-loop compare at `0x37B77`: `bp` has run past the fleet
/// array, so it tests the byte `idx` (1-based machine index) against
/// three cells inside the tail — machine-link slots laid out like the
/// record's `+0x18`/`+0x1C`/`+0x20`.
fn clear_tail(c: &mut Ctx, idx: u16) {
    let tail = u16::try_from(FLEET_BASE + usize::from(FLEET_COUNT) * FLEET_STRIDE).unwrap_or(0);
    for ofs in [0x18, 0x1C, 0x20] {
        let a = tail + ofs;
        if c.vm.r8(c.st, a) == u8::try_from(idx).unwrap_or(0) {
            c.vm.w8(c.st, a, 0);
        }
    }
}

/// `cs:0x7BAB` — find a derelict (type 7, flags `0x30`) machine off the
/// current planet; stores it at `[0x81F2]` and clears its type.
/// Suspends while none qualify.
pub fn wait_derelict(c: &mut Ctx) -> Flow {
    let cur = c.vm.r16(c.st, u16::try_from(SEL_REC).unwrap_or(0));
    let serial = c.vm.r8(c.st, cur + 0x28);
    for i in 0..MACH_COUNT {
        let m = mach(i);
        if derelict(c, m, serial) {
            c.vm.w16(c.st, R_MACH, m);
            c.vm.w8(c.st, m + u16::try_from(M_TYPE).unwrap_or(0), 0);
            return Flow::Cont;
        }
    }
    Flow::Ret
}

/// The `0x7BAB` per-machine test.
fn derelict(c: &mut Ctx, m: u16, serial: u8) -> bool {
    let ty = m + u16::try_from(M_TYPE).unwrap_or(0);
    let fl = c.vm.r8(c.st, m + u16::try_from(M_FLAGS).unwrap_or(0));
    c.vm.r8(c.st, ty) == 7
        && (fl & 0x30) == 0x30
        && c.vm.r8(c.st, m + u16::try_from(M_HOST).unwrap_or(0)) != serial
        && m != c.vm.r16(c.st, u16::try_from(SEL_MACH).unwrap_or(0))
}

/// `cs:0x7C16` — zero the build timer `+0x0C` on all machines, dirty
/// `0x800`.
pub fn reset_timers(c: &mut Ctx) -> Flow {
    for i in 0..MACH_COUNT {
        c.vm.w16(c.st, mach(i) + u16::try_from(M_TIMER).unwrap_or(0), 0);
    }
    set_bits(c, u16::try_from(DIRTY0).unwrap_or(0), 0x800);
    Flow::Cont
}

/// `cs:0x7C30` — find a ripe farm (type 5, flag `0x4`, `+0x0A >=
/// 0x4B0`); harvest it into `[0x81F2]` and clear `+0x0A`, dirty
/// `0x4000`. Suspends while none qualify.
pub fn wait_ripe(c: &mut Ctx) -> Flow {
    for i in 0..MACH_COUNT {
        let m = mach(i);
        if ripe(c, m) {
            c.vm.w16(c.st, R_MACH, m);
            c.vm.w16(c.st, m + u16::try_from(M_ACC).unwrap_or(0), 0);
            set_bits(c, u16::try_from(DIRTY0).unwrap_or(0), 0x4000);
            return Flow::Cont;
        }
    }
    Flow::Ret
}

/// The `0x7C30` per-machine test (`type 5`, flag `0x4`, `+0x0A` ripe).
fn ripe(c: &mut Ctx, m: u16) -> bool {
    let ty = m + u16::try_from(M_TYPE).unwrap_or(0);
    let fl = c.vm.r8(c.st, m + u16::try_from(M_FLAGS).unwrap_or(0));
    let acc = c.vm.r16(c.st, m + u16::try_from(M_ACC).unwrap_or(0));
    c.vm.r8(c.st, ty) == 5 && (fl & 0x4) != 0 && acc >= 0x4B0
}
