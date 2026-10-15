//! The embedded `GAME.unpacked.exe` image (GAM-55) — the port reads the
//! unpacked 16-bit file by raw offset, but that file is git-ignored, so
//! the three regions it actually reads ship as committed blobs in
//! `rust/data/` and [`image`] rebuilds the full byte layout at runtime:
//!
//! | blob | file offsets | contents |
//! |---|---|---|
//! | `low.bin` | `0x0C00..0x106A0` | PC/Tandy/AdLib driver modules + songs |
//! | `dgroup.bin` | `0x106A0..0x292B0` | the 64 KiB `ds:` image plus the
//!   `ss:` space above it — the `menu_text` stream (`STREAM = 0x202F0`)
//!   reads up into it — up to the `cs:` base |
//! | `cs.bin` | `0x292B0..0x38E90` | the `cs:` space (`TPL_OFS`-relative) |
//!
//! Together the blobs are contiguous — `image()` output equals the
//! load image `exepack::unpack` produces (`u.image`), with only the
//! `0x0000..0x0C00` MZ header zeroed (nothing reads it). Regenerate the
//! blobs from the shipped `GAME.EXE` with `cargo run --bin extract-img`.

mod blobs;
mod from_exe;
mod image;

pub use blobs::{Region, REGIONS};
pub use from_exe::from_exe;
pub use image::image;

/// Byte length of the file layout [`image`] reconstructs — the verified
/// unpacked reference is 233,104 bytes.
pub const IMAGE_LEN: usize = 0x38E90;

/// MZ header size in the unpacked reference — every file offset the
/// code cites is `MZ_HDR +` the load-image offset [`crate::exepack`]'s
/// `unpack` produces.
pub const MZ_HDR: usize = 0xC00;
