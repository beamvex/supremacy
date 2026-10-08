//! `unpack` must reproduce the verified unpacked images byte-for-byte:
//! load image, relocation table, and real entry/stack.

mod common;

use common::{load_image, mz_relocs, read, root};
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
    check(
        &read(&root().join("GAME/GAME.EXE")),
        &read(&root().join("decompiled/GAME.unpacked.exe")),
    );
}

#[test]
fn supcht_exe_unpacks_identically() {
    check(
        &read(&root().join("rust/tests/fixtures/SUPCHT.EXE")),
        &read(&root().join("decompiled/SUPCHT.unpacked.exe")),
    );
}

#[test]
fn repackaged_exe_roundtrips_image_and_relocs() {
    let u = unpack(&read(&root().join("GAME/GAME.EXE"))).unwrap();
    let exe = u.to_exe();
    assert_eq!(load_image(&exe), u.image.as_slice());
    let mut got = u.relocs.clone();
    got.sort_unstable();
    assert_eq!(mz_relocs(&exe), got);
}
