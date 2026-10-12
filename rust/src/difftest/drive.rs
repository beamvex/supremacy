//! Port-side replay — applies a [`Script`]'s ops to a [`Game`] with the
//! probe's semantics (one tick = one `Game::step`) and writes the same
//! artefact names the TSR writes, into a parallel directory. Input ops
//! share [`input`][super::input] with the smoke digest's drive loop.

use std::fs;
use std::io;
use std::path::Path;

use crate::front::Game;

use super::dump::{dump_name, pal_name, stat_name, DONE_NAME};
use super::input::input;
use super::op::{Op, Rec};
use super::snap::snapshot;

/// Replay `ops` against `g`, writing port artefacts under `dir`.
/// Returns the tick reached.
///
/// `from_tick` skips ops scheduled at/before it — the boot-from-dump
/// path uses the tick of the boot dumps so inputs already folded into
/// the snapshot aren't replayed. The restored snapshot *is* the state
/// at `from_tick`, so the tick counter starts there: artefact ops at
/// `from_tick` re-emit without stepping, and later ops step
/// `tick − from_tick` frames to match the original's timeline.
///
/// # Errors
/// Propagates `dir` creation and artefact-write `io` failures.
pub fn run(g: &mut Game, ops: &[Rec], dir: &Path, from_tick: u16, settle: u16) -> io::Result<u16> {
    fs::create_dir_all(dir)?;
    let mut step = from_tick;
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
    if input(g, op, step) {
        return Ok(());
    }
    match op {
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
        Op::Stat { id } => fs::write(dir.join(stat_name(id)), ops_ptr(*step))?,
        Op::Done => fs::write(dir.join(DONE_NAME), b"K")?,
        _ => {}
    }
    Ok(())
}

fn ops_ptr(step: u16) -> [u8; 24] {
    let mut b = [0u8; 24];
    b[4..6].copy_from_slice(&step.to_le_bytes());
    b
}
