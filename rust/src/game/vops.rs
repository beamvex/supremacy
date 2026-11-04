//! Event-VM op bodies — the routines at `cs:0x7768..0x7C61`, now
//! called directly by the compiled `script::evNN` functions (GAM-80:
//! the `cs:0x7758` fetch-dispatch loop is transpiled away). Each
//! returns [`Flow::Cont`] (`jmp 0x7758`) or [`Flow::Ret`] (`ret`,
//! suspend).

use super::consts::{FLEET_BASE, FLEET_COUNT, FLEET_STRIDE, REC_COUNT, UIMODE};
use super::rec::F_KIND;
use super::rng::Rng;
use super::state::State;
use super::vhost::{Call, VmHost};
use super::vm::Vm;
use crate::platform::DosFiles;

/// The VM's `ds` scratch registers (inside the save block).
pub const R_SEL: u16 = 0x81EE;
/// Found-machine pointer (`cs:0x7BD6`/`0x7C49` stores here).
pub const R_MACH: u16 = 0x81F2;
/// Current planet index.
pub const R_PLANET: u16 = 0x81F6;
/// Scratch byte.
pub const R_A: u16 = 0x81F7;
/// Scratch word.
pub const R_B: u16 = 0x81F8;
/// Scratch dword (lo/hi at `+0`/`+2`).
pub const R_C: u16 = 0x81FA;

/// Borrow bundle handed to every op — the five contexts the asm
/// touches: VM memory, game state, the PRNG, host services, files.
pub struct Ctx<'a> {
    /// Low/high `ds` memory (scripts, tables, menu scratch).
    pub vm: &'a mut Vm,
    /// The saveable state block.
    pub st: &'a mut State,
    /// The `cs:0x55B2` PRNG.
    pub rng: &'a mut Rng,
    /// Frontend services (print/menu/sound/draw slots).
    pub host: &'a mut dyn VmHost,
    /// The `int 21h` file layer the save/load routines use.
    pub files: &'a mut dyn DosFiles,
}

/// One `0x2CA9` frame boundary inside a loop: the vsync/UI service plus
/// re-mirroring host input into the `ds:` cells (the asm's int handlers
/// feed them asynchronously mid-loop).
pub fn vsync(c: &mut Ctx) {
    c.host.svc(Call::Ui(0x2CA9));
    c.host.pump_input(c.vm, c.st);
}

/// `ds:` offset of the current planet's record (`[0x81F6]` × `0x3A` +
/// `[0x9158]`).
#[must_use]
pub fn planet_rec(c: &Ctx) -> u16 {
    c.vm.rec_ofs(c.st, c.vm.r16(c.st, R_PLANET) & 0xFF)
}

/// `ds:` offset of the player-faction record (`[0x91B8]` × `0x3A` +
/// `[0x9158]` — the `cs:0x5E3A` accessor).
#[must_use]
pub fn faction_rec(c: &Ctx) -> u16 {
    c.vm.rec_ofs(c.st, c.vm.r16(c.st, u16::try_from(REC_COUNT).unwrap_or(0)))
}

/// `ds:` byte of the UI-mode cell `[0x91CD]`.
#[must_use]
pub fn uimode(c: &Ctx) -> u8 {
    c.vm.r8(c.st, u16::try_from(UIMODE).unwrap_or(0))
}

/// Word `rec + F_KIND` — the kind/status test the wait ops use.
#[must_use]
pub fn kind_of(c: &Ctx, rec: u16) -> u16 {
    c.vm.r16(c.st, rec + u16::try_from(F_KIND).unwrap_or(0))
}

/// `or word [a],v` — the dirty-flag idiom every op shares.
pub fn set_bits(c: &mut Ctx, a: u16, v: u16) {
    c.vm.w16(c.st, a, c.vm.r16(c.st, a) | v);
}

/// Shared ship-unlink: zero the 8-byte head of every fleet record whose
/// `+2` link points at `rec` (file `0x37916`/`0x37B3B`). Returns `false`
/// for the `[0x914C]` early-out in UI mode 1 (the `0x797F`/`0x7BAA`
/// abort path).
pub fn unlink_ships(c: &mut Ctx, rec: u16) -> bool {
    for k in 0..FLEET_COUNT {
        let s =
            u16::try_from(FLEET_BASE).unwrap_or(0) + k * u16::try_from(FLEET_STRIDE).unwrap_or(0);
        if uimode(c) == 1 && s == c.vm.r16(c.st, 0x914C) {
            return false;
        }
        if c.vm.r16(c.st, s + 2) == rec {
            for f in 0..8 {
                c.vm.w8(c.st, s + f, 0);
            }
        }
    }
    true
}
