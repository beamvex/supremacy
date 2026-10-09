//! Checkpoint boot — start the port from a DOS `ds` dump so the run
//! doesn't depend on the original's timer-seeded RNG: the script dumps
//! `ds` and the `drv:0x55A2` LCG state early, and the replay restores
//! them here instead of relying on `Game::new` init matching.

use crate::args::{Sound, Video};
use crate::assets::AssetSet;
use crate::front::{Game, Phase};
use crate::game::UIMODE;
use crate::palette::Palette;
use crate::platform::DosFiles;

use super::snap::ds_image;

/// A [`Game`] whose `ds` state and RNG come straight from a DOS
/// checkpoint: `ds` is the 64 KiB region the probe's `dump ds` wrote
/// (two `0x8000` halves concatenated), `rng_state` the `drv:0x55A2`
/// dword dump.
#[must_use]
#[allow(clippy::too_many_arguments)]
pub fn from_dumps(
    img: Vec<u8>,
    set: AssetSet,
    pal: Palette,
    video: Video,
    sound: Sound,
    k_mode: bool,
    ds: &[u8],
    rng_state: u32,
    files: Box<dyn DosFiles>,
) -> Game {
    let mut g = Game::new(img, set, pal, video, sound, k_mode, 0, rng_state, files);
    restore(&mut g, ds);
    g.rng.set_state(rng_state);
    g.phase = if g.st.byte(UIMODE) == 5 {
        Phase::Galaxy
    } else {
        Phase::Shell
    };
    g
}

/// Overlay a `ds` dump onto the port's three-region memory model.
fn restore(g: &mut Game, ds: &[u8]) {
    let lo = g.vm.low.len().min(ds.len());
    g.vm.low[..lo].copy_from_slice(&ds[..lo]);
    let blk = 0x7D39..0x9CC4;
    if ds.len() >= blk.end {
        g.st.block.copy_from_slice(&ds[blk]);
    }
    if ds.len() > 0x9CC4 {
        let n = g.vm.hi.len().min(ds.len() - 0x9CC4);
        g.vm.hi[..n].copy_from_slice(&ds[0x9CC4..0x9CC4 + n]);
    }
}

/// The tick a bootable script's checkpoint dumps sit at — the maximum
/// tick among `dump ds` ops whose id is `1`/`2` (the canonical full-ds
/// pair) or `3` (the RNG dword). Replay skips ops at/before it.
#[must_use]
pub fn boot_tick(ops: &[super::Rec]) -> u16 {
    use super::op::Op;
    ops.iter()
        .filter(|r| matches!(r.op, Op::Dump { id: 1..=3, .. }))
        .map(|r| r.tick)
        .max()
        .unwrap_or(0)
}

/// Whether a `ds` image pair + rng dump are all present in `dir`.
#[must_use]
pub fn have_boot(dir: &std::path::Path) -> bool {
    use super::dump::dump_name;
    [dump_name(1), dump_name(2), dump_name(3)]
        .iter()
        .all(|n| dir.join(n).is_file())
}

/// Read the boot state out of a dump directory — the two `0x8000`-byte
/// `ds` halves plus the `drv:0x55A2` dword.
#[must_use]
pub fn read_boot(dir: &std::path::Path) -> Option<(Vec<u8>, u32)> {
    use super::dump::dump_name;
    let mut ds = std::fs::read(dir.join(dump_name(1))).ok()?;
    ds.extend(std::fs::read(dir.join(dump_name(2))).ok()?);
    let r = std::fs::read(dir.join(dump_name(3))).ok()?;
    let seed = u32::from_le_bytes(r.get(0..4)?.try_into().ok()?);
    Some((ds, seed))
}

/// The `ds` bytes the port would dump at this instant — for tests that
/// round-trip [`from_dumps`] without a DOS run.
#[must_use]
pub fn port_ds(g: &Game) -> Vec<u8> {
    ds_image(g)
}
