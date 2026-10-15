//! The port side's exe image (GAM-57): `difftest::port_img` must boot
//! the original `GAME.EXE`'s bytes — a dev's `unpacked.exe` override
//! first, then the shipped packed exe unpacked at run time, then the
//! embedded blobs — so both halves of a diff trace to the same binary.

mod common;

use common::{root, try_read};
use supremacy::difftest as dt;
use supremacy::difftest::ImgSrc;

#[test]
fn shipped_exe_unpacks_to_the_embedded_image() {
    let Some(packed) = try_read(&root().join("GAME/GAME.EXE")) else {
        return;
    };
    let img = supremacy::img::from_exe(&packed).unwrap();
    assert_eq!(img, supremacy::img::image());
}

#[test]
fn port_img_uses_the_shipped_exe_when_present() {
    let Some(packed) = try_read(&root().join("GAME/GAME.EXE")) else {
        return;
    };
    let dir = std::env::temp_dir().join("supremacy-difftest-img-exe");
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(dir.join("GAME")).unwrap();
    std::fs::write(dir.join("GAME/GAME.EXE"), &packed).unwrap();
    let (img, src) = dt::port_img(&dir);
    assert_eq!(src, ImgSrc::GameExe);
    assert_eq!(img, supremacy::img::image());
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn port_img_falls_back_to_blobs_without_an_exe() {
    let dir = std::env::temp_dir().join("supremacy-difftest-img-none");
    let _ = std::fs::remove_dir_all(&dir);
    let (img, src) = dt::port_img(&dir);
    assert_eq!(src, ImgSrc::Blobs);
    assert_eq!(img, supremacy::img::image());
    // A GAME.EXE that isn't EXEPACK still falls through to the blobs.
    std::fs::create_dir_all(dir.join("GAME")).unwrap();
    std::fs::write(dir.join("GAME/GAME.EXE"), b"not an exe").unwrap();
    let (_, src) = dt::port_img(&dir);
    assert_eq!(src, ImgSrc::Blobs);
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn port_img_prefers_a_dev_unpacked_exe() {
    let dir = std::env::temp_dir().join("supremacy-difftest-img-ovr");
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(dir.join("decompiled")).unwrap();
    std::fs::write(dir.join("decompiled/GAME.unpacked.exe"), b"patched").unwrap();
    let (img, src) = dt::port_img(&dir);
    assert_eq!(src, ImgSrc::Unpacked);
    assert_eq!(img, b"patched");
    let _ = std::fs::remove_dir_all(&dir);
}
