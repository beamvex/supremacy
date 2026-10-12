//! The headless smoke mode (GAM-24) — the digest is deterministic over
//! `Game::step` replays, `done` ends the run early, and the golden
//! values pin this build's behaviour so a render/audio/state drift
//! fails CI.
//!
//! Regenerate the goldens after a deliberate behaviour change, e.g.
//! `cargo run --bin supremacy-smoke -- ../GAME --ticks 90` from `rust/`.

mod common;

use common::{game_dir, read, root};
use supremacy::args::{Sound, Video};
use supremacy::assets::{heap_segment, load_set};
use supremacy::difftest as dt;
use supremacy::front::Game;
use supremacy::palette::{from_dac, DAC_LEN, DAC_OFS};
use supremacy::platform::MemFs;
use supremacy::smoke::{self, Digest};

/// The `supremacy-smoke GAME` bundle — MCG set, image palette, speaker,
/// K-mode off (no `K` letter), preset 3, seed `0x42`.
fn game() -> Game {
    let img = read(&root().join("decompiled/GAME.unpacked.exe"));
    let set = load_set(&game_dir(), "MCG", heap_segment()).unwrap();
    let pal = from_dac(&img[DAC_OFS..DAC_OFS + DAC_LEN]);
    Game::new(
        img,
        set,
        pal,
        Video::Mcga,
        Sound::PcSpeaker,
        false,
        3,
        0x42,
        Box::new(MemFs::new()),
    )
}

#[test]
fn digest_is_deterministic() {
    assert_eq!(
        smoke::run(&mut game(), &[], 90),
        smoke::run(&mut game(), &[], 90)
    );
}

#[test]
fn golden_shell_run() {
    let d = smoke::run(&mut game(), &[], 90);
    let want = Digest {
        steps: 90,
        video: 0x0CCF_FC77_4E67_A15B,
        pal: 0x9A74_F14E_5E88_E01F,
        audio: 0x006B_9955_B24D_FF65,
        state: 0x004A_1C57_990E_5C7D,
        rng: 0x0002_3B6B,
    };
    assert_eq!(d, want);
}

#[test]
fn golden_galaxy_run() {
    let mut g = game();
    g.enter_galaxy();
    let d = smoke::run(&mut g, &[], 60);
    let want = Digest {
        steps: 60,
        video: 0x8405_8F45_05A7_581B,
        pal: 0x2EFF_7DA3_552D_7E49,
        audio: 0x4CE9_06BA_48A6_AC1B,
        state: 0xFA68_F424_BC6E_69FE,
        rng: 0x0002_3B6B,
    };
    assert_eq!(d, want);
}

#[test]
fn script_poke_lands_and_done_stops_early() {
    let s = dt::parse("04 pokew ds F000 BEEF\n06 key 1C\n08 key 9C\n0A done\n");
    let mut g = game();
    let d = smoke::run(&mut g, &s.0, 90);
    assert_eq!(d.steps, 0x0A);
    assert_eq!(g.vm.r16(&g.st, 0xF000), 0xBEEF);
}

#[test]
fn expect_specs_match_partial_fields() {
    let d = smoke::run(&mut game(), &[], 10);
    let spec = format!("video={:X},state={:X} rng={:X}", d.video, d.state, d.rng);
    assert!(d.check(&spec).is_empty());
    assert!(d.check(&format!("steps={}", d.steps)).is_empty());
    assert_eq!(d.check("video=0").len(), 1);
    assert_eq!(d.check("bogus=1").len(), 1);
    assert_eq!(d.check("nothing here").len(), 2);
}
