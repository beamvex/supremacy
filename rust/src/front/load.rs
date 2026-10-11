//! `Call::Load` — the int-21 asset loaders (`0x20F2`/`0x208E`/
//! `0x21BF`/`0x215B`/`0x2228`/`0x228C`/`0x22F5`/`0x2359`).
//!
//! Each opens the filename cell at `ds:arg` — one of the `ds:0x1208`+
//! `???.???` templates the mode select patches with the set stem —
//! then find-firsts, reads and closes into the load heap, retrying
//! through the `0x8AC0` insert-disk prompt on failure. The port maps
//! the cell straight to `{stem}{sfx}.{BIN,GPH}`: the `.BIN` read
//! stashes the payload, the `.GPH` read rebuilds the index and swaps
//! the active [`AssetSet`] (the asm loads BIN-then-GPH at every call
//! site). A missing file or pair counts as `dropped` — the `0x8AC0`
//! retry prompt has no frame-loop equivalent.

use std::path::PathBuf;

use crate::assets::{heap_segment, load_bin, load_index, AssetSet};

/// `ds:` cell → (`stem` suffix, `.GPH`?). Order follows the mode
/// templates' name table: `*.BIN`, `*.GPH`, `*IN.*`, `*DTH.*`, `*WIN.*`.
fn cell_name(cell: u16) -> Option<(&'static str, bool)> {
    Some(match cell {
        0x1208 => ("", false),
        0x1210 => ("", true),
        0x1218 => ("IN", false),
        0x1222 => ("IN", true),
        0x122C => ("DTH", false),
        0x1237 => ("DTH", true),
        0x1242 => ("WIN", false),
        0x124D => ("WIN", true),
        _ => return None,
    })
}

/// The asset directory + set stem `Call::Load` resolves against, and
/// the stashed `.BIN` payload waiting for its `.GPH` half.
pub struct Loader {
    dir: PathBuf,
    stem: String,
    bin: Option<Vec<u8>>,
}

impl Loader {
    /// A loader rooted at `dir` for stem `stem` (`MCG` etc.).
    #[must_use]
    pub fn new(dir: PathBuf, stem: &str) -> Self {
        Self {
            dir,
            stem: stem.to_owned(),
            bin: None,
        }
    }

    /// One loader call — `false` (counted `dropped`) on an unknown
    /// cell, unreadable file or a `.GPH` with no `.BIN` staged.
    pub fn load(&mut self, set: &mut AssetSet, cell: u16) -> bool {
        let Some((sfx, gph)) = cell_name(cell) else {
            return false;
        };
        let base = self.dir.join(format!("{}{sfx}", self.stem));
        if !gph {
            self.bin = load_bin(&base.with_extension("BIN")).ok();
            return self.bin.is_some();
        }
        let (Ok(records), Some(bin)) = (
            load_index(&base.with_extension("GPH"), heap_segment()),
            self.bin.take(),
        ) else {
            return false;
        };
        *set = AssetSet {
            records,
            bin,
            heap_seg: heap_segment(),
        };
        true
    }
}
