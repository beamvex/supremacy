use super::{load_bin, load_index, AssetSet};
use std::io;
use std::path::Path;

/// Load one asset set — `dir/STEM.GPH` + `dir/STEM.BIN` — the way the game
/// does for e.g. `MCG`/`EGAIN`/`CGAWIN`.
///
/// # Errors
/// Propagates the first `fs::read` failure.
pub fn load_set(dir: &Path, stem: &str, heap_seg: u16) -> io::Result<AssetSet> {
    let records = load_index(&dir.join(format!("{stem}.GPH")), heap_seg)?;
    let bin = load_bin(&dir.join(format!("{stem}.BIN")))?;
    Ok(AssetSet {
        records,
        bin,
        heap_seg,
    })
}
