//! Which bytes the port side boots — the diff is only honest when both
//! halves run the same program, so the image follows the original
//! `GAME.EXE` DOSBox-X is about to boot rather than the committed
//! blobs alone (GAM-57).
//!
//! Precedence: a `decompiled/GAME.unpacked.exe` on disk still wins —
//! difftest debugging may swap in a patched image — then
//! `GAME/GAME.EXE` unpacked through [`crate::img::from_exe`] (the same
//! file the DOS pass runs), then the `data/*.bin` blobs as the
//! `--port-only` fallback on a clone with no exe at all.

use std::fmt;
use std::path::Path;

/// Where [`port_img`] sourced the port-side bytes — the driver prints
/// it so a run's transcript always names the binary under test.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ImgSrc {
    /// Dev override — `decompiled/GAME.unpacked.exe` on disk.
    Unpacked,
    /// The shipped `GAME/GAME.EXE`, unpacked at run time.
    GameExe,
    /// The committed `data/*.bin` blobs — no exe on disk.
    Blobs,
}

impl fmt::Display for ImgSrc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Unpacked => "decompiled/GAME.unpacked.exe",
            Self::GameExe => "GAME/GAME.EXE (unpacked)",
            Self::Blobs => "embedded data/*.bin",
        };
        f.write_str(s)
    }
}

/// The port-side image for a run rooted at `root`, in the module-docs
/// precedence order, plus where it came from.
#[must_use]
pub fn port_img(root: &Path) -> (Vec<u8>, ImgSrc) {
    if let Ok(b) = std::fs::read(root.join("decompiled/GAME.unpacked.exe")) {
        return (b, ImgSrc::Unpacked);
    }
    let packed = std::fs::read(root.join("GAME/GAME.EXE")).ok();
    if let Some(img) = packed.and_then(|f| crate::img::from_exe(&f)) {
        return (img, ImgSrc::GameExe);
    }
    (crate::img::image(), ImgSrc::Blobs)
}
