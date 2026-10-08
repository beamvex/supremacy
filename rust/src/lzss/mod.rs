//! Okumura LZSS decoder (`N=4096`, `F=18`, `THRESHOLD=2`).
//!
//! Port of the four near-identical decoder routines in `GAME.unpacked.exe`:
//!
//! | file offset | stride | store | trailing byte |
//! |---|---|---|---|
//! | `0x2EBEF` (MCG/TGA/CGA) | `0x140` | `stosb` | not consumed |
//! | `0x2EC8C` (MCG/TGA/CGA) | `0x140` | `xor [es:di],al` | not consumed |
//! | `0x2FC5D` (EGA) | `0x28` | `stosb` | consumed (`lodsb` at `0x2FCF7`) |
//! | `0x2FCF9` (EGA) | `0x28` | `xor [es:di],al` | consumed (`lodsb` at `0x2FD9A`) |
//!
//! All variants share the format: a 4 KB ring buffer (zero-initialised,
//! write cursor `r = 0xFEE`), flag bits consumed LSB-first with a `0x100`
//! sentinel reload, and a terminator of match flag + `00 00` token followed
//! by a `00` byte.

mod consts;
mod copy_match;
mod decode;
mod decode_at;
mod decode_planes;
mod decoder;
mod decoder_default;
mod decoder_new;
mod draw;
mod emit;
mod frame;
mod frame_emit;
mod frame_new;
mod kind;
mod literal;
mod next_flag;
mod push;
mod run;
mod step;
mod token;
mod vec_emit;

pub use decode::decode;
pub use decode_at::decode_at;
pub use decode_planes::decode_planes;
pub use decoder::Decoder;
pub use draw::{draw, Blit};
pub use emit::Emit;
pub use frame::Frame;
pub use kind::Kind;
