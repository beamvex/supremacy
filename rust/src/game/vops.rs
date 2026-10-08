//! Event-VM op dispatch — the routines at `cs:0x7768..0x7C61` the
//! `ds:0x50E3` op table jumps to. Each returns [`Flow::Cont`]
//! (`jmp 0x7758`) or [`Flow::Ret`] (`ret`, suspend).
//!
//! Operand encoding in scripts: a bare `u16` immediate (`si += 2`) or a
//! 4-byte slot `{ptr, tag}` (`si += 4`; the shipped tag is `0x0FAA`).

use super::consts::{FLEET_BASE, FLEET_COUNT, FLEET_STRIDE, REC_COUNT, UIMODE};
use super::rec::F_KIND;
use super::rng::Rng;
use super::state::State;
use super::vhost::VmHost;
use super::vm::{Flow, Vm};
use super::{vdir, vimpl, vmac, vplan};

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

/// Borrow bundle handed to every op — the four contexts the asm
/// touches: VM memory, game state, the PRNG, host services.
pub struct Ctx<'a> {
    /// Low/high `ds` memory (scripts, tables, menu scratch).
    pub vm: &'a mut Vm,
    /// The saveable state block.
    pub st: &'a mut State,
    /// The `cs:0x55B2` PRNG.
    pub rng: &'a mut Rng,
    /// Frontend services (print/menu/sound/draw slots).
    pub host: &'a mut dyn VmHost,
}

/// Run event `idx` until a handler suspends — the `cs:0x773D` entry
/// (`si` from the `ds:0x5183` table) plus the `cs:0x7758` fetch loop.
/// Returns the op count executed.
pub fn run(c: &mut Ctx, idx: u16) -> u16 {
    let mut si = c.vm.r16(c.st, super::vm::TAB_BASE + (idx & 0xFF) * 4);
    let mut ran = 0u16;
    loop {
        let w = c.vm.r16(c.st, si);
        let target = c.vm.r16(c.st, super::vm::JT_BASE.wrapping_add(w));
        si = si.wrapping_add(2);
        ran = ran.wrapping_add(1);
        if op(c, target, &mut si) == Flow::Ret {
            return ran;
        }
    }
}

/// `jmp word [ds:0x50E3+w]` — dispatch on the `cs:` handler target.
pub fn op(c: &mut Ctx, t: u16, si: &mut u16) -> Flow {
    match t {
        0x7768 | 0x7AF0 => Flow::Ret,
        0x7769 => vimpl::wait_b(c, si, false),
        0x7774 => vimpl::print(c, si),
        0x7782 => vimpl::set_flag(c, si, 0xFF),
        0x778C => vimpl::set_flag(c, si, 0),
        0x7796 => vimpl::wait_eq_b(c, si),
        0x77A5 => vimpl::wait_eq_w(c, si, false),
        0x77B4 => vimpl::wait_eq_w(c, si, true),
        0x77C3 => vimpl::print_at(c, planet_rec(c) + 0xE),
        0x77E1 => vimpl::print_at(c, c.vm.r16(c.st, R_SEL) + 0xE),
        0x77F0 => vimpl::rand_planet(c),
        0x7811 => vimpl::ld_ind(c, si, R_A, 1),
        0x7825 => vimpl::ld_ind(c, si, R_B, 2),
        0x7839 => vimpl::ld_ind(c, si, R_C, 4),
        0x7853 => vimpl::cp(c, si, 1),
        0x7866 => vimpl::cp(c, si, 2),
        0x7879 => vimpl::cp(c, si, 4),
        _ => op2(c, t, si),
    }
}

fn op2(c: &mut Ctx, t: u16, si: &mut u16) -> Flow {
    match t {
        0x7893 => vimpl::add_imm(c, si, 1),
        0x78A2 => vimpl::add_imm(c, si, 2),
        0x78B1 => vimpl::add_imm(c, si, 4),
        0x78C7 => vimpl::wait_b(c, si, true),
        0x78D5 => vimpl::st_imm(c, si, 1),
        0x78E4 => vimpl::st_imm(c, si, 2),
        0x78F3 => vimpl::st_imm(c, si, 4),
        0x7908 => vimpl::native(c, si),
        0x790A => vplan::kill_sel(c),
        0x7980 => vplan::wait_kind(c, 0xA),
        0x798E => vplan::wait_kind(c, 7),
        _ => op3(c, t, si),
    }
}

fn op3(c: &mut Ctx, t: u16, si: &mut u16) -> Flow {
    match t {
        0x799C => vplan::wait_kind(c, 4),
        0x79AA => vimpl::delay(c, si),
        0x79BB => vimpl::menu(c),
        0x79C3 => vimpl::dialog(c),
        0x7A11 => vplan::levy(c),
        0x7A45 => vdir::disaster(c),
        0x7ADD => vmac::unhide(c),
        0x7AF1 => vmac::raze(c),
        0x7BAB => vmac::wait_derelict(c),
        0x7BE8 => vimpl::print_at(c, c.vm.r16(c.st, R_MACH)),
        0x7BF4 => vplan::depopulate(c),
        0x7C05 => vplan::charge(c),
        0x7C16 => vmac::reset_timers(c),
        0x7C30 => vmac::wait_ripe(c),
        _ => Flow::Ret,
    }
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
