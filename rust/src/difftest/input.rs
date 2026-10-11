//! Input-op application — the port-side semantics of `key`, `mouse`,
//! `pokew` and `waitcell`, shared by the artefact-driving replay
//! ([`drive::run`][super::run]) and the headless smoke digest
//! ([`crate::smoke`]). `step` counts `Game::step`s, including the ones
//! a latch wait or `waitcell` runs internally.

use crate::front::Game;
use crate::game::{VmHost, KEY_PEND};

use super::op::Op;
use super::seg::Seg;

/// `drv:` base in the exe image — `snap.rs`'s constant, needed to
/// mirror `POKEW` into `Host::img`.
const DRV_OFS: usize = 0x292B0;
/// Max `step()`s a `WaitCell`/latch wait may take before the replay
/// gives up on it (keeps a stuck scenario from hanging forever).
const WAIT_CAP: u16 = 5000;

/// Apply an input op to `g` — `true` when `op` was one of
/// `Key`/`Mouse`/`Poke`/`WaitCell` (the artefact ops return `false` so
/// callers can dispatch on them).
pub fn input(g: &mut Game, op: Op, step: &mut u16) -> bool {
    match op {
        Op::Key { scan } => key(g, scan, step),
        Op::Mouse { x, y, buttons, .. } => {
            g.host.pump.move_to(x, y);
            g.host.pump.set_buttons(buttons & 1 != 0, buttons & 2 != 0);
        }
        Op::Poke { seg, ofs, val } => poke(g, seg, ofs, val),
        Op::WaitCell { ofs, val } => wait(g, ofs, val, step),
        _ => return false,
    }
    true
}

/// The probe's latch-write: hold until `[0x9CCB]` frees, then queue the
/// code and mirror it into `ds:` (like the next `pump_input` does).
fn key(g: &mut Game, scan: u8, step: &mut u16) {
    for _ in 0..WAIT_CAP {
        if g.vm.r8(&g.st, u16::try_from(KEY_PEND).unwrap_or(0)) == 0 {
            break;
        }
        g.step();
        *step += 1;
    }
    g.host.pump.key(scan);
    g.host.pump_input(&mut g.vm, &mut g.st);
}

fn poke(g: &mut Game, seg: Seg, ofs: u16, val: u16) {
    match seg {
        Seg::Ds => g.vm.w16(&mut g.st, ofs, val),
        Seg::Drv => {
            let b = DRV_OFS + usize::from(ofs);
            g.host.img[b] = u8::try_from(val & 0xFF).unwrap_or(0);
            g.host.img[b + 1] = u8::try_from(val >> 8).unwrap_or(0);
        }
        Seg::Abs(_) => {}
    }
}

fn wait(g: &mut Game, ofs: u16, val: u8, step: &mut u16) {
    for _ in 0..WAIT_CAP {
        if g.vm.r8(&g.st, ofs) == val {
            return;
        }
        g.step();
        *step += 1;
    }
}
