//! `0xFA` — the endgame monitor (`cs:0x82E5`, file `0x382E5`–`0x38338`)
//! and the two full-screen sequences it fires.
//!
//! Branch order (file `0x382E5`–`0x38308`): the player-faction record —
//! index `[0x91B8]` via `cs:0x5E3A`, **not** record `0` — at kind `7`
//! (fallen/conquered) → **A**; record `0` (the home/starbase record)
//! at kind `0xA` (inhabited) → **B**; then `cs:0x37C7` counts the
//! planet records — no kind `7` left → A, no kind `0xA` left → B, else
//! `ret` (`None` here).
//!
//! The arms (`0x8322`/`0x8309`) set a flag + `[0x9C9A]` code and load
//! the ending's asset pair through the `ds:0x1208`+ name cells —
//! A is **defeat** (`*DTH.BIN`/`*DTH.GPH`, `cs:0x8B17`), B is
//! **victory** (`*WIN.BIN`/`*WIN.GPH`, `cs:0x8C65`). The sequences
//! themselves run frame-stepped in [`seq`]; the `0x1D35` restart tail
//! is [`tail`].

mod fx;
mod seq;
mod tail;

use super::consts::{KEY_PEND, REC_BASE, REC_COUNT};
use super::vhost::Call;
use super::vm::HI_OFS;
use super::vops::{faction_rec, kind_of, Ctx};

pub use seq::{end_step, End};

/// `u16` casts for the `ds:` slot constants.
macro_rules! a {
    ($k:expr) => {
        u16::try_from($k).unwrap_or(0)
    };
}

/// `ds:` flag set on the `0x8322` branch (file `0x38322`).
const END_A_FLAG: u16 = 0x91F1;
/// `ds:` flag set on the `0x8309` branch (file `0x38309`).
const END_B_FLAG: u16 = 0x91F0;
/// `ds:` selector written with `2`/`3` alongside the flags (`[0x9C9A]`,
/// file `0x3830E`/`0x38327`; the `0x8AC0` disk prompt reads it back).
const END_CODE: u16 = 0x9C9A;

/// `ds:` filename cells — the mode select patches each `???.???`
/// template with the set stem (`MCG` → `MCG.BIN` … `MCGWIN.GPH`).
const CELL_BIN: u16 = 0x1208;
const CELL_GPH: u16 = 0x1210;
const CELL_IN_BIN: u16 = 0x1218;
const CELL_IN_GPH: u16 = 0x1222;
const CELL_DTH_BIN: u16 = 0x122C;
const CELL_DTH_GPH: u16 = 0x1237;
const CELL_WIN_BIN: u16 = 0x1242;
const CELL_WIN_GPH: u16 = 0x124D;

/// Which ending sequence the monitor fired.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Ending {
    /// `cs:0x8322` — faction record kind `7`, or no kind-`7` planet
    /// left — **defeat**: flag `0x91F1`, code `2`, `*DTH` assets
    /// (`ds:0x122C`/`0x1237`), `cs:0x8B17`.
    A,
    /// `cs:0x8309` — record `0` kind `0xA`, or no kind-`0xA` planet
    /// left — **victory**: flag `0x91F0`, code `3`, `*WIN` assets
    /// (`ds:0x1242`/`0x124D`), `cs:0x8C65`.
    B,
}

/// `cs:0x82E5` — one endgame check; see the module docs for the branch
/// order. Firing runs the arm's flag/code/loads and the sequence head
/// synchronously, then parks the rest of the body in `Vm::end` for
/// [`end_step`] to pace.
pub fn endgame(c: &mut Ctx) -> Option<Ending> {
    if kind_of(c, faction_rec(c)) == 7 {
        return Some(fire(c, Ending::A));
    }
    let rec0 = c.vm.r16(c.st, a!(REC_BASE));
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
    let n = c.vm.r16(c.st, a!(REC_COUNT));
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

/// The arm block plus the sequence head (file `0x38309`–`0x38B55`);
/// the frame-paced body suspends into `Vm::end`.
fn fire(c: &mut Ctx, e: Ending) -> Ending {
    arm(c, e);
    seq::head(c, e);
    c.vm.end = Some(End::new(e, c.vm.r16(c.st, seq::SCENARIO)));
    e
}

/// `0x8322`/`0x8309`: `[0x91F1]`/`[0x91F0]` + `[0x9C9A]`, the `0xA3BC`
/// buttons-only hook, the ending's `.BIN`/`.GPH` pair (`*DTH` for A,
/// `*WIN` for B), `0xA36A` input reinstall (file `0x38309`–`0x38338`).
fn arm(c: &mut Ctx, e: Ending) {
    let (flag, code, bin, gph) = match e {
        Ending::A => (END_A_FLAG, 2, CELL_DTH_BIN, CELL_DTH_GPH),
        Ending::B => (END_B_FLAG, 3, CELL_WIN_BIN, CELL_WIN_GPH),
    };
    c.vm.w8(c.st, flag, 0xFF);
    c.vm.w8(c.st, END_CODE, code);
    c.host.svc(Call::Mouse(0x14, 0x1E, 0));
    load_bin(c, bin);
    load_gph(c, gph);
    c.host.svc(Call::Mouse(0x14, 0x1F, 0));
}

/// `0x20F2`/`0x21BF`/`0x228C`/`0x2359` — the `.BIN` payload loader:
/// `[0x9CC4]`/`[0x9CCB]` cleared, then find+read+close on the `ds:`
/// name cell, retrying through the `0x8AC0` disk prompt on failure.
fn load_bin(c: &mut Ctx, cell: u16) {
    c.vm.w8(c.st, a!(HI_OFS), 0);
    c.vm.w8(c.st, a!(KEY_PEND), 0);
    c.host.svc(Call::Load(cell));
}

/// `0x208E`/`0x215B`/`0x2228`/`0x22F5` — the `.GPH` index loader on a
/// `ds:` name cell (the same find/read/close without the cell clears).
fn load_gph(c: &mut Ctx, cell: u16) {
    c.host.svc(Call::Load(cell));
}
