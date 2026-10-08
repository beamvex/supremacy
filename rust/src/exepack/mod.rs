//! Microsoft EXEPACK unpacker — port of the stub at `GAME.EXE:0x23A90`.
//!
//! The stub copies its own block (header + stub + packed fixups) to the top
//! of the `dest_len` region, then decompresses *backwards* (`std` at
//! `0x23AA9`): each command block is `[fill?][len:2][cmd]`, `cmd & 0xFE` =
//! `0xB0` fill / `0xB2` copy, `cmd & 1` = last block. The load image's head
//! is stored uncompressed, so seeding `dest` with the image and expanding
//! the tail reproduces the in-place algorithm byte-for-byte. Afterwards the
//! `0x23B22` loop applies `add [es:di],bx` fixups from the packed relocation
//! stream (16 batches of `count` + `count`×`u16` offsets, one per `0x1000`
//! paragraphs) and the stub far-jumps to the real `cs:ip`.

mod apply_relocs;
mod cmd;
mod cmd_last;
mod commands;
mod consts;
mod copy;
mod error;
mod error_display;
mod fill;
mod header;
mod header_parse;
mod mz;
mod mz_parse;
mod mz_write;
mod read_cmd;
mod reloc_list;
mod reloc_ofs;
mod sat;
mod scan;
mod to_exe;
mod unpack;
mod unpacked;
mod word;

pub use apply_relocs::apply_relocs;
pub use error::Error;
pub use header::Header;
pub use reloc_list::reloc_list;
pub use unpack::unpack;
pub use unpacked::Unpacked;
