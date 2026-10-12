//! The GAM-9 frontend — a concrete [`VmHost`][crate::game::VmHost] that
//! replays the game's service calls against a real framebuffer, palette
//! and asset set, plus the input pump that mirrors window events into
//! the `ds:` latch cells.
//!
//! - [`audio`]: the sound backend — `Call::Sound`/`Chan`/`Sfx` map onto
//!   the ported `int 0x80` PC/Tandy driver over a
//!   [`Ports`][crate::audio::Ports] synth bus (`audio::Bus`), rendered
//!   to `f32` by `audio::Audio`; `audio::Out`
//!   (feature `audio`) is the cpal stream the window loop feeds.
//! - `font`: the shipped 4×6 font — the `[0x1268]` slot's per-mode
//!   `ds:` pointer tables and packed records, read from the exe image.
//! - [`Pump`]: int-9/int-33h input state → `[0x9CD6..0x9CCD]` cells.
//! - [`Host`]: `VmHost` impl — `Image` draws, `Glyph` puts, the
//!   `0x125C` clear, `0x1260` DAC write, `int 33` calls; everything else
//!   counted in [`Host::dropped`].
//! - [`Game`]: owns the `Vm`/`State`/`Rng`/`Host` bundle and alternates
//!   the shell and galaxy frame loops.
//! - `window` (feature `minifb`): the `320×200` window + event loop.

pub mod audio;
mod font;
mod host;
mod load;
mod pump;
mod runner;
mod stars;
mod text;
#[cfg(feature = "minifb")]
pub mod window;

pub use host::{Host, InputSrc};
pub use pump::Pump;
pub use runner::{Game, Phase};
