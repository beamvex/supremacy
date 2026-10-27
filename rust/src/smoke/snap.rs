//! The port-side `ds` snapshot the [`Digest`][super::Digest]'s `state`
//! channel hashes.

use crate::front::Game;

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
