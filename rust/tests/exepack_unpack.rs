//! `unpack` must reproduce the verified unpacked images byte-for-byte:
//! load image, relocation table, and real entry/stack. The packed exe
//! and its unpacked references are git-ignored, so each test skips when
//! they aren't on disk (fresh clone) — `extract-img --check` covers the
//! committed blobs' provenance.

mod common;

use common::{load_image, mz_relocs, root, try_read};
use supremacy::exepack::unpack;

fn check(packed: &[u8], want: &[u8]) {
    let u = unpack(packed).unwrap();
    assert_eq!(u.image, load_image(want), "unpacked image differs");
    let mut got = u.relocs.clone();
    got.sort_unstable();
    assert_eq!(got, mz_relocs(want), "relocation tables differ");
    let w = |o: usize| u16::from_le_bytes([want[o], want[o + 1]]);
    assert_eq!((u.ip, u.cs), (w(0x14), w(0x16)));
    assert_eq!((u.sp, u.ss), (w(0x10), w(0x0E)));
}

#[test]
fn game_exe_unpacks_identically() {
    let (Some(packed), Some(want)) = (
        try_read(&root().join("GAME/GAME.EXE")),
        try_read(&root().join("decompiled/GAME.unpacked.exe")),
    ) else {
        return;
    };
    check(&packed, &want);
}

#[test]
fn supcht_exe_unpacks_identically() {
    let (Some(packed), Some(want)) = (
        try_read(&root().join("rust/tests/fixtures/SUPCHT.EXE")),
        try_read(&root().join("decompiled/SUPCHT.unpacked.exe")),
    ) else {
        return;
    };
    check(&packed, &want);
}

#[test]
fn repackaged_exe_roundtrips_image_and_relocs() {
    let Some(packed) = try_read(&root().join("GAME/GAME.EXE")) else {
        return;
    };
    let u = unpack(&packed).unwrap();
    let exe = u.to_exe();
    assert_eq!(load_image(&exe), u.image.as_slice());
    let mut got = u.relocs.clone();
    got.sort_unstable();
    assert_eq!(mz_relocs(&exe), got);
}

/// The embedded blobs rebuild the load image `unpack` produces — the
/// regions are contiguous, so `img[MZ_HDR..]` equals `u.image` exactly
/// (the MZ header stays zeroed; nothing reads it). Needs `GAME.EXE`;
/// skips without it.
#[test]
fn embedded_blobs_match_unpacked_image() {
    let Some(packed) = try_read(&root().join("GAME/GAME.EXE")) else {
        return;
    };
    let u = unpack(&packed).unwrap();
    let img = supremacy::img::image();
    assert_eq!(
        &img[supremacy::img::MZ_HDR..],
        u.image.as_slice(),
        "embedded image differs"
    );
}
