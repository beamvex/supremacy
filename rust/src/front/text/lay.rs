//! The four `menu_text` variants' `si`-space parameters — one
//! [`Lay`] per video mode (file `0x2F629`/`0x30520`/`0x30FFE`/
//! `0x31CAA`). The stream bytecodes are identical; only the `si`
//! addressing and the tile painters differ per mode.
//!
//! `si` spaces: MCG is linear (`row·0x140 + byte`), EGA is within a
//! plane (`line·0x28 + byte`, all four planes written at the same
//! `di` through the map mask), CGA/TGA are bank-interleaved
//! (`bank·0x2000 + line·pitch + byte`, CGA two banks `0x50`-wide,
//! TGA four `0xA0`-wide). The xoff and `0xF0` arg bytes shift right
//! `xsh` bits before applying so the same stream lands the same
//! pixels in every mode.

use crate::args::Video;

/// `0x18` — the 16×24 tile row span every variant's painters share.
pub const TILE_H: usize = 0x18;
/// Screen height in visual rows — the `brows` cap.
pub const ROWS: usize = 0xC8;

/// A mode's `menu_text` geometry.
pub struct Lay {
    /// `si` at each block's line start.
    pub org: u16,
    /// Right-shift on the xoff and `0xF0` arg bytes.
    pub xsh: u8,
    /// `0xF0` row step after `si -= arg >> xsh`.
    pub row_add: u16,
    /// CGA's `si ^= 0x2000` bank toggle on a new line (`0` elsewhere).
    pub bank_xor: u16,
    /// `si` advance per tile op.
    pub tile: u16,
}

/// The mode's `Lay`.
pub fn lay(mode: Video) -> Lay {
    match mode {
        Video::Mcga => Lay {
            org: 0x3840,
            xsh: 0,
            row_add: 0x1F40,
            bank_xor: 0,
            tile: 0x10,
        },
        Video::Ega => Lay {
            org: 0x708,
            xsh: 3,
            row_add: 0x3E8,
            bank_xor: 0,
            tile: 2,
        },
        Video::Cga => Lay {
            org: 0x26E0,
            xsh: 2,
            row_add: 0x3C0,
            bank_xor: 0x2000,
            tile: 4,
        },
        Video::Tga => Lay {
            org: 0x460,
            xsh: 1,
            row_add: 0x3C0,
            bank_xor: 0,
            tile: 8,
        },
    }
}

/// Final walk `si` → the visual rows the staged block covers
/// (`si_max/pitch + TILE_H` linear; the bank decode for CGA/TGA —
/// out-of-screen `si` values just overflow into the [`ROWS`] cap).
pub fn vrows(si: u16, mode: Video) -> usize {
    let s = usize::from(si);
    let n = match mode {
        Video::Mcga => s / 0x140,
        Video::Ega => s / 0x28,
        Video::Cga => (s & 0x1FFF) / 0x50 * 2 + (s >> 13),
        Video::Tga => (s & 0x1FFF) / 0xA0 * 4 + (s >> 13),
    };
    n + TILE_H
}

/// Byte offset of visual row `y` inside a banked `buf` —
/// `(y % banks)·0x2000 + (y / banks)·pitch`.
pub fn rofs(y: usize, banks: usize, pitch: usize) -> usize {
    (y % banks) * 0x2000 + (y / banks) * pitch
}
