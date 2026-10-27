//! The GAM-24 headless smoke mode — runs a [`crate::front::Game`] for a
//! fixed number of steps without a window or an audio device, feeding
//! scripted input from the `.scr` op format, and reduces the result to
//! a [`Digest`]: FNV-1a hashes of the VRAM, palette, rendered PCM and
//! reassembled `ds` image, plus the final `Rng` state.
//!
//! A CI job builds `supremacy-smoke` (default features — no `minifb`,
//! no `cpal`), runs it against the shipped `GAME/` assets and compares
//! the printed `key=hex` line against a pinned spec with `--expect`,
//! so a render/audio/state regression fails without a display.

mod digest;
mod drive;
mod hash;
mod input;
mod op;
mod script;
mod seg;
mod snap;

pub use digest::Digest;
pub use drive::{run, SAMPLES_PER_STEP};
pub use hash::{fnv1a, Fnv};
pub use op::{Op, Rec};
pub use script::{parse, Script};
pub use seg::Seg;
