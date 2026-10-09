//! Port-side snapshots — the byte content a `DUMP`/`PAL` op produces
//! when replayed against [`Game`] instead of DOS.

use crate::front::Game;

use super::op::Op;
use super::seg::Seg;

/// The driver/template segment base in the unpacked exe — runtime
/// `drv:`/`cs:` offsets map to file `0x292B0 + ofs` (`host.rs`'s
/// `TPL_OFS`): the RNG state `cs:0x55A2` = file `0x2E852`.
const DRV_OFS: usize = 0x292B0;
/// `cs:` offset of the 32-bit LCG state (file `0x2E852`/`0x2E854`).
const RNG_OFS: usize = 0x55A2;

/// The full 64 KiB `ds` space reassembled from the port's three-region
/// model (`low` + save block + `hi`), matching a `dump ds 0 FFFF`.
#[must_use]
pub fn ds_image(g: &Game) -> Vec<u8> {
    let mut v = Vec::with_capacity(0x10000);
    v.extend_from_slice(&g.vm.low);
    v.extend_from_slice(&g.st.block[..]);
    v.extend_from_slice(&g.vm.hi);
    v
}

/// `drv:` bytes — the exe image at `DRV_OFS + ofs`, with the LCG state
/// patched in so `drv:0x55A2` dumps compare like-for-like.
fn drv_image(g: &Game, ofs: u16, len: u16) -> Vec<u8> {
    let base = DRV_OFS + usize::from(ofs);
    let end = base.saturating_add(usize::from(len));
    let mut v = g.host.img.get(base..end).unwrap_or(&[]).to_vec();
    for i in 0..4 {
        if usize::from(ofs) + i >= RNG_OFS && usize::from(ofs) + i < RNG_OFS + 4 {
            let b = g.rng.state().to_le_bytes();
            v[i] = b[usize::from(ofs) + i - RNG_OFS];
        }
    }
    v
}

/// The port bytes a `Pal`/`Stat`/`Dump` op would emit — `None` for ops
/// that produce no artefact.
#[must_use]
pub fn snapshot(g: &Game, op: &Op) -> Option<Vec<u8>> {
    match *op {
        Op::Dump { seg, ofs, len, .. } => Some(match seg {
            Seg::Ds => window(&ds_image(g), ofs, len),
            Seg::Drv => drv_image(g, ofs, len),
            Seg::Abs(0xA000) => window(&g.host.scr.buf, ofs, len),
            Seg::Abs(_) => Vec::new(),
        }),
        Op::Pal { .. } => Some(
            (0..256)
                .flat_map(|i| g.host.pal.rgb(i).map(|c| c >> 2))
                .collect(),
        ),
        _ => None,
    }
}

fn window(b: &[u8], ofs: u16, len: u16) -> Vec<u8> {
    let o = usize::from(ofs);
    let n = usize::from(len);
    b.get(o..o.saturating_add(n)).unwrap_or(&[]).to_vec()
}
