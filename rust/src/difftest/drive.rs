//! Port-side replay — applies a [`Script`]'s ops to a [`Game`] with the
//! probe's semantics (one tick = one `Game::step`) and writes the same
//! artefact names the TSR writes, into a parallel directory.

use std::fs;
use std::io;
use std::path::Path;

use crate::front::Game;
use crate::game::{VmHost, KEY_PEND};

use super::dump::{dump_name, pal_name, stat_name, DONE_NAME};
use super::op::{Op, Rec};
use super::seg::Seg;
use super::snap::snapshot;

/// `drv:` base in the exe image — `snap.rs`'s constant, needed to
/// mirror `POKEW` into `Host::img`.
const DRV_OFS: usize = 0x292B0;
/// Max `step()`s a `WaitCell`/latch wait may take before the replay
/// gives up on it (keeps a stuck scenario from hanging forever).
const WAIT_CAP: u16 = 5000;

/// Replay `ops` against `g`, writing port artefacts under `dir`.
/// Returns the number of `step()`s run.
///
/// `from_tick` skips ops scheduled at/before it — the boot-from-dump
/// path uses the tick of the boot dumps so inputs already folded into
/// the snapshot aren't replayed.
///
/// # Errors
/// Propagates `dir` creation and artefact-write `io` failures.
pub fn run(g: &mut Game, ops: &[Rec], dir: &Path, from_tick: u16, settle: u16) -> io::Result<u16> {
    fs::create_dir_all(dir)?;
    let mut step = 0u16;
    for r in ops {
        if r.tick <= from_tick && !artefact(r.op) {
            continue; // inputs/pokes already folded into the boot dump
        }
        while step < r.tick {
            g.step();
            step += 1;
        }
        apply(g, r.op, dir, &mut step)?;
    }
    for _ in 0..settle {
        g.step();
        step += 1;
    }
    Ok(step)
}

/// Does this op only emit an artefact? Such ops still run at/before
/// `from_tick` — the boot snapshot re-emits them so the report has a
/// port side for the boot dumps too.
fn artefact(op: Op) -> bool {
    matches!(op, Op::Dump { .. } | Op::Pal { .. } | Op::Stat { .. })
}

fn apply(g: &mut Game, op: Op, dir: &Path, step: &mut u16) -> io::Result<()> {
    match op {
        Op::Key { scan } => key(g, scan, step),
        Op::Mouse { x, y, buttons, .. } => {
            g.host.pump.move_to(x, y);
            g.host.pump.set_buttons(buttons & 1 != 0, buttons & 2 != 0);
        }
        Op::Dump { id, .. } | Op::Pal { id } => {
            let name = if matches!(op, Op::Pal { .. }) {
                pal_name(id)
            } else {
                dump_name(id)
            };
            if let Some(b) = snapshot(g, &op) {
                fs::write(dir.join(name), b)?;
            }
        }
        Op::Poke { seg, ofs, val } => poke(g, seg, ofs, val),
        Op::WaitCell { ofs, val } => wait(g, ofs, val, step),
        Op::Stat { id } => fs::write(dir.join(stat_name(id)), ops_ptr(*step))?,
        Op::Done => fs::write(dir.join(DONE_NAME), b"K")?,
    }
    Ok(())
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

fn ops_ptr(step: u16) -> [u8; 24] {
    let mut b = [0u8; 24];
    b[4..6].copy_from_slice(&step.to_le_bytes());
    b
}
