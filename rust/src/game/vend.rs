//! `0xFA` — the endgame monitor (`cs:0x82E5`, file `0x382E5`–`0x38338`).
//! Runs every sequencer cycle; normally returns after the planet-count
//! check, but fires one of two full-screen sequences when its branch
//! conditions hit.

use super::consts::REC_BASE;
use super::vhost::Call;
use super::vops::{faction_rec, kind_of, Ctx};

/// `ds:` flag set on the `0x8322` branch (file `0x38322`).
const END_A_FLAG: u16 = 0x91F1;
/// `ds:` flag set on the `0x8309` branch (file `0x38309`).
const END_B_FLAG: u16 = 0x91F0;
/// `ds:` selector written with `2`/`3` alongside the flags (`[0x9C9A]`,
/// file `0x3830E`/`0x38327`).
const END_CODE: u16 = 0x9C9A;

/// Which ending screen the monitor fired.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Ending {
    /// `cs:0x8322` — faction record kind `7`, or no kind-`7` planet left;
    /// sets `[0x91F1]`, code `2`, runs the `cs:0x8B17` sequence.
    A,
    /// `cs:0x8309` — record `0` kind `0xA`, or no kind-`0xA` planet left;
    /// sets `[0x91F0]`, code `3`, runs the `cs:0x8C65` sequence.
    B,
}

/// `cs:0x82E5` — one endgame check. The asm order: faction-record kind
/// `7` → A; record `0` kind `0xA` → B; then the `cs:0x37C7` planet
/// count — no kind-`7` → A, no kind-`0xA` → B, else the `ret` at
/// `cs:0x8308` (`None` here).
pub fn endgame(c: &mut Ctx) -> Option<Ending> {
    if kind_of(c, faction_rec(c)) == 7 {
        return Some(fire(c, Ending::A));
    }
    let rec0 = c.vm.r16(c.st, u16::try_from(REC_BASE).unwrap_or(0));
    if kind_of(c, rec0) == 0xA {
        return Some(fire(c, Ending::B));
    }
    match count_kinds(c) {
        (0, _) => Some(fire(c, Ending::A)),
        (_, 0) => Some(fire(c, Ending::B)),
        _ => None,
    }
}

/// `cs:0x37C7` — scan the planet records, returning `(kind7, kindA)`
/// counts (`ax`/`bp` in the asm).
fn count_kinds(c: &mut Ctx) -> (u16, u16) {
    let n =
        c.vm.r16(c.st, u16::try_from(super::consts::REC_COUNT).unwrap_or(0));
    let (mut a7, mut aa) = (0u16, 0u16);
    for i in 0..n {
        match kind_of(c, c.vm.rec_ofs(c.st, i)) {
            7 => a7 = a7.wrapping_add(1),
            0xA => aa = aa.wrapping_add(1),
            _ => {}
        }
    }
    (a7, aa)
}

/// The shared flag/code writes plus the host-side screen sequence
/// (`call 0xA3BC`/`0x2xxx`/`0xA36A`, `jmp 0x8B17`/`0x8C65`).
fn fire(c: &mut Ctx, e: Ending) -> Ending {
    let (flag, code, screen) = match e {
        Ending::A => (END_A_FLAG, 2, 0x8B17),
        Ending::B => (END_B_FLAG, 3, 0x8C65),
    };
    c.vm.w8(c.st, flag, 0xFF);
    c.vm.w8(c.st, END_CODE, code);
    c.host.svc(Call::Screen(screen));
    e
}
