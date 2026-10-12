//! GAM-15 — the `Ar` audit regressions: behaviours corrected against
//! the file-`0x2900` disasm that the earlier tests didn't cover (the
//! MPU `0xFE`-exact ACK and the per-step `mvol_apply` in the fader).

mod common;

use common::rec::Rec;
use common::{read, root};
use supremacy::audio::Ar;

fn img() -> Vec<u8> {
    read(&root().join("decompiled/GAME.unpacked.exe"))
}

/// The title song record (see `audio_ar.rs`).
fn song0(img: &[u8]) -> &[u8] {
    &img[0x4970..]
}

#[test]
fn roland_init_rejects_a_garbage_ack() {
    let mut a = Ar::new(Rec::new(), img().as_slice());
    // DSR clear, DRR clear, then a non-`0xFE` reply on `0x330` — the
    // asm `cmp`s for `0xFE` and stores `0` to `[0x1C02]` otherwise.
    a.ports.replies.extend([0x00, 0x00, 0x42]);
    assert_eq!(a.init(1), 0);
    assert!(a.ports.outs().contains(&(0x331, 0xFF))); // reset attempted
    a.ports.ops.clear();
    a.load_song(song0(&img()));
    for _ in 0..20 {
        a.service();
    }
    assert!(a.ports.ops.is_empty()); // `mode = 0xFF` — stayed absent
}

/// `F0` `SysEx` heads emitted on `0x330` over 10 services of `song0`,
/// with the fader armed at `rate` (`0` = unarmed baseline).
fn roland_f0s(rate: u8) -> usize {
    let mut a = Ar::new(Rec::new(), img().as_slice());
    a.ports.replies.extend([0x00, 0x00, 0xFE, 0x00, 0x00, 0xFE]);
    a.init(1);
    a.load_song(song0(&img()));
    a.select(0);
    if rate != 0 {
        a.fade_set(rate);
    }
    a.ports.ops.clear();
    for _ in 0..10 {
        a.service();
    }
    a.ports
        .outs()
        .iter()
        .filter(|&&(p, v)| p == 0x330 && v == 0xF0)
        .count()
}

#[test]
fn fade_applies_the_volume_each_step() {
    // Roland `frate = 0x10` — five `0x3B/0x2B/0x1B/0x0B/0` steps from
    // `mvol = 0x4B` (`[0x205E]` shipped `2`, steps on even services).
    // `0x1F40` SysEx's `volbuf` on *every* step (the `jnc` falls into
    // the apply) — five `F0`s over the unarmed baseline.
    assert_eq!(roland_f0s(0x10), roland_f0s(0) + 5);
}
