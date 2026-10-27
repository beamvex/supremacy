use super::{IMAGE_LEN, REGIONS};

/// Rebuild the unpacked-exe byte layout from the committed blobs — a
/// zeroed `IMAGE_LEN` vector with each [`super::Region`] copied to its
/// file offset. `Host::img`, `Vm::from_image`, `State::from_image`, the
/// palette/font/audio readers and smoke snapshots all index by the
/// file offsets the docs cite, so the result is interchangeable with a
/// `std::fs::read` of `GAME.unpacked.exe` for every byte the port reads.
#[must_use]
pub fn image() -> Vec<u8> {
    let mut img = vec![0; IMAGE_LEN];
    for r in &REGIONS {
        img[r.ofs..r.ofs + r.bytes.len()].copy_from_slice(r.bytes);
    }
    img
}
