//! `.BIN`/`.GPH` asset-set loading.
//!
//! Port of the loader pair in `GAME.unpacked.exe`: `.GPH` index read +
//! in-place paragraph fixup (`0x3208E`-`0x320E9`) and the chunked `.BIN`
//! heap load (`0x320F2`-`0x3215A`). Filename buffers at `ds:0x1242`/`
//! 0x124D` are `???.???` templates the video-mode select patches with
//! `MCG`/`EGA`/`CGA`/`TGA` + `.BIN`/`.GPH`; here the stem is a parameter.

mod heap;
mod load_bin;
mod load_index;
mod load_set;
mod set;
mod stream;

pub use heap::heap_segment;
pub use load_bin::load_bin;
pub use load_index::load_index;
pub use load_set::load_set;
pub use set::AssetSet;
