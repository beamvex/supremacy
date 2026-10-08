//! The GAM-9 frontend — a concrete [`VmHost`][crate::game::VmHost] that
//! replays the game's service calls against a real framebuffer, palette
//! and asset set, plus the input pump that mirrors window events into
//! the `ds:` latch cells.
//!
//! - `font`: a debug 4×6 font for the `[0x1268]` glyph slot (the real
//!   table is runtime-loaded by the undecoded `0x1278` op).
//! - [`Pump`]: int-9/int-33h input state → `[0x9CD6..0x9CCD]` cells.
//! - [`Host`]: `VmHost` impl — `Image` draws, `Glyph` puts, the
//!   `0x125C` clear, `0x1260` DAC write, `int 33` calls; everything else
//!   counted in [`Host::dropped`].
//! - [`Game`]: owns the `Vm`/`State`/`Rng`/`Host` bundle and alternates
//!   the shell and galaxy frame loops.
//! - `window` (feature `minifb`): the `320×200` window + event loop.

mod font;
mod host;
mod pump;
mod runner;
#[cfg(feature = "minifb")]
pub mod window;

pub use host::Host;
pub use pump::Pump;
pub use runner::{Game, Phase};
