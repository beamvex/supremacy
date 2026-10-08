//! Command-line parser — `GAME <C/T/E/M> <K> <P/T/A/R>`.
//!
//! Port of the routine at `0x31FF8`-`0x3207D`, which reads single-letter
//! arguments from fixed cells in the PSP command tail: `es:0x82` = video,
//! `es:0x84` = keyboard flag, `es:0x86` = sound. `SUPRE.BAT`'s `GAME M M A`
//! is MCGA video + real mouse + `AdLib`; bare `GAME` falls through to the
//! MCG/PC-speaker defaults exactly as the asm does.

mod config;
mod parse;
mod sound;
mod sound_from;
mod video;
mod video_from;
mod video_stem;
mod video_template;

pub use config::Config;
pub use parse::parse;
pub use sound::Sound;
pub use video::Video;
