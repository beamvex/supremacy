//! The headless run — `ticks` × [`Game::step`], script input ops
//! applied at their tick via [`difftest::input`], and a fixed chunk of
//! PCM rendered per step so the `audio` channel sees what a windowed
//! run's stream would have been fed.
//!
//! Determinism is the contract: the same exe image, asset set, args,
//! seed and script must always produce the same [`Digest`]. Steps an
//! op's wait loop runs internally count toward `steps` but render no
//! audio — same input, same render points.

use crate::difftest::{self, Op, Rec};
use crate::front::audio::BOOT_RATE;
use crate::front::Game;

use super::digest::Digest;
use super::hash::{fnv1a, Fnv};

/// PCM samples rendered per step — the window loop's `rate/30` stride
/// at the boot rate.
#[allow(clippy::cast_lossless)] // const context — `usize::try_from` isn't const
pub const SAMPLES_PER_STEP: usize = BOOT_RATE as usize / 30;

/// Run `g` for `ticks` steps (or until a `done` op), hashing as it
/// goes; returns the final [`Digest`]. `ops` are the
/// [`difftest`][crate::difftest] script records — `key`/`mouse`/
/// `pokew`/`waitcell` apply, `done` ends the run, and the artefact ops
/// (`dump`/`pal`/`stat`) are skipped: snapshots here are the hash
/// channels, not files.
#[must_use]
pub fn run(g: &mut Game, ops: &[Rec], ticks: u16) -> Digest {
    let mut sorted = ops.to_vec();
    sorted.sort_by_key(|r| r.tick);
    let mut it = sorted.iter().peekable();
    let mut audio = Fnv::new();
    let mut pcm = vec![0.0f32; SAMPLES_PER_STEP];
    let mut step = 0u16;
    let mut done = false;
    while step < ticks && !done {
        while let Some(r) = it.next_if(|r| r.tick <= step) {
            done |= apply(g, r.op, &mut step);
        }
        if done {
            break;
        }
        tick(g, &mut audio, &mut pcm);
        step = step.saturating_add(1);
    }
    finish(g, step, audio.sum())
}

/// One frame plus its audio chunk folded into `audio`.
fn tick(g: &mut Game, audio: &mut Fnv, pcm: &mut [f32]) {
    g.step();
    g.host.snd.render(pcm);
    for &s in pcm.iter() {
        audio.write_u32(s.to_bits());
    }
}

/// `true` when the op ends the run — `input` handles the four input
/// ops, the artefact ops fall through as no-ops.
fn apply(g: &mut Game, op: Op, step: &mut u16) -> bool {
    if difftest::input(g, op, step) {
        return false;
    }
    matches!(op, Op::Done)
}

/// Hash the quiescent end state — VRAM, palette, `ds` image, LCG.
fn finish(g: &Game, steps: u16, audio: u64) -> Digest {
    let mut pal = Fnv::new();
    for i in 0..256 {
        pal.write(&g.host.pal.rgb(i));
    }
    Digest {
        steps,
        video: fnv1a(&g.host.scr.buf),
        pal: pal.sum(),
        audio,
        state: fnv1a(&difftest::ds_image(g)),
        rng: g.rng.state(),
    }
}
