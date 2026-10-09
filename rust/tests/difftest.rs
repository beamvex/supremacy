//! The `difftest` harness — script codec, segment selectors, artefact
//! naming, the diff report, port-side replay/dump generation and
//! checkpoint boot, plus a gated DOSBox-X end-to-end (skipped unless
//! `DIFFTEST_E2E=1`, a `dosbox-x` binary and `difftest/probe.com` are
//! all present).

mod common;

use common::{game_dir, read, root};
use supremacy::args::{Sound, Video};
use supremacy::assets::{heap_segment, load_set};
use supremacy::difftest as dt;
use supremacy::front::Game;
use supremacy::palette::{from_dac, DAC_OFS};
use supremacy::platform::MemFs;

fn game() -> Game {
    let img = read(&root().join("decompiled/GAME.unpacked.exe"));
    let set = load_set(&game_dir(), "MCG", heap_segment()).unwrap();
    let pal = from_dac(&img[DAC_OFS..DAC_OFS + 768]);
    Game::new(
        img,
        set,
        pal,
        Video::Mcga,
        Sound::PcSpeaker,
        true,
        3,
        0x42,
        Box::new(MemFs::new()),
    )
}

#[test]
fn script_parse_and_record_layout() {
    let s = dt::parse(
        "# comment\n0A key 1C\n0E key 9C\n14 mouse 1E 160 96 1\n\
         1E dump ds 7D39 1F8B 01\n1E dump drv 55A2 4 03\n\
         1E dump A000 0 FA00 04\n1E pal 05\n1E stat 06\n\
         28 waitcell 91CD 5\n32 pokew drv 55A2 1234\n3C done\n",
    );
    assert_eq!(s.0.len(), 11);
    assert_eq!(s.0[0].op, dt::Op::Key { scan: 0x1C });
    let r = s.0[3].encode();
    assert_eq!(&r[..4], &[0x1E, 0x00, 3, 1]); // tick, DUMP, id
    assert_eq!(&r[4..10], &[0x39, 0x7D, 0x8B, 0x1F, 0xFF, 0xFF]);
    let m = s.0[2].encode();
    assert_eq!(m[2], 2);
    assert_eq!(u16::from_le_bytes([m[4], m[5]]), 0x160);
    let w = s.0[8].encode();
    assert_eq!(w[2], 8);
    assert_eq!(&w[4..8], &[0xCD, 0x91, 5, 0]);
    assert_eq!(s.to_bin().len(), s.0.len() * 12);
    let rec = |w: u16| dt::Rec {
        tick: w,
        op: dt::Op::Done,
    };
    assert_eq!(rec(9).encode()[2], 5);
}

#[test]
fn segment_selectors_round_trip() {
    assert_eq!(dt::Seg::Ds.word(), 0xFFFF);
    assert_eq!(dt::Seg::Drv.word(), 0xFFFE);
    assert_eq!(dt::Seg::from_word(0xA000), dt::Seg::Abs(0xA000));
    assert_eq!(dt::Seg::parse("ds"), Some(dt::Seg::Ds));
    assert_eq!(dt::Seg::parse("A000"), Some(dt::Seg::Abs(0xA000)));
}

#[test]
fn artefact_names_and_dbg_parse() {
    assert_eq!(dt::dump_name(1), "D01.BIN");
    assert_eq!(dt::pal_name(5), "P05.BIN");
    assert_eq!(dt::stat_name(0), "S00.BIN");
    let d = dt::dbg_parse(&[
        0x23, 0x1A, 0x14, 0x08, 2, 0, 12, 0, 84, 0, 1, 2, 3, 4, 0, 0, 0xBB, 0xAA, 0x99, 0x88, 0, 0,
        0, 0,
    ]);
    assert_eq!((d.game_ds, d.drv_seg, d.tick), (0x1A23, 0x0814, 2));
}

#[test]
fn report_counts_first_diffs() {
    let d = dt::compare("D01.BIN", &[1, 2, 3, 4], &[1, 9, 3, 8, 0]);
    assert_eq!(d.diffs, 3); // two mismatches + one byte of length skew
    assert_eq!(d.first, vec![(1, 2, 9), (3, 4, 8)]);
    let mut r = dt::Report::default();
    r.diffs.push(d);
    r.diffs.push(dt::compare("P05.BIN", &[7], &[7]));
    assert_eq!(r.clean(), 1);
    assert!(r.render().contains("D01.BIN"));
}

#[test]
fn port_replay_writes_matching_artefacts() {
    let dir = std::env::temp_dir().join("supremacy-difftest-port");
    let s = dt::parse(
        "05 key 1C\n06 key 9C\n\
         08 dump ds 7D39 1F8B 01\n08 dump drv 55A2 4 03\n\
         08 dump A000 0 FA00 04\n08 pal 05\n08 stat 06\n0A done\n",
    );
    let mut g = game();
    let steps = dt::run(&mut g, &s.0, &dir, 0, 0).unwrap();
    assert_eq!(steps, 0x0A);
    for (n, len) in [
        ("D01.BIN", 0x1F8B),
        ("D03.BIN", 4),
        ("D04.BIN", 0xFA00),
        ("P05.BIN", 768),
        ("S06.BIN", 24),
        ("DONE.BIN", 1),
    ] {
        let b = read(&dir.join(n));
        assert_eq!(b.len(), len, "{n}");
    }
    // The rng dword dump carries the live LCG state, not the exe image.
    assert_eq!(
        &read(&dir.join("D03.BIN"))[..],
        &g.rng.state().to_le_bytes()
    );
}

#[test]
fn boot_from_ds_dump_restores_state() {
    let mut g = game();
    for _ in 0..40 {
        g.step();
    }
    let ds = dt::port_ds(&g);
    let img = read(&root().join("decompiled/GAME.unpacked.exe"));
    let set = load_set(&game_dir(), "MCG", heap_segment()).unwrap();
    let pal = from_dac(&img[DAC_OFS..DAC_OFS + 768]);
    let mut g2 = dt::from_dumps(
        img,
        set,
        pal,
        Video::Mcga,
        Sound::PcSpeaker,
        true,
        &ds,
        g.rng.state(),
        Box::new(MemFs::new()),
    );
    assert_eq!(dt::port_ds(&g2), ds);
    g2.step(); // a restored game must still drive the loop
}

#[test]
fn dosbox_e2e_is_gated_on_tools() {
    let e2e = std::env::var_os("DIFFTEST_E2E").is_some();
    let probe = root().join("difftest/probe.com");
    let bin = dt::find_dosbox();
    if !e2e || bin.is_none() || !probe.is_file() {
        eprintln!("skipped (set DIFFTEST_E2E=1; needs dosbox-x + probe.com)");
        return;
    }
    let s = dt::parse("10 dump ds 0 8000 01\n20 dump ds 8000 8000 02\n30 done\n");
    let out = root().join("difftest/out");
    std::fs::write(root().join("difftest/SCRIPT.BIN"), s.to_bin()).unwrap();
    let conf = out.join("e2e.conf");
    std::fs::write(&conf, dt::conf(&root(), "M K P")).unwrap();
    dt::run_dos(
        &bin.unwrap(),
        &conf,
        &out,
        std::time::Duration::from_secs(240),
    )
    .unwrap();
    for n in ["D01.BIN", "D02.BIN", dt::DONE_NAME] {
        assert!(out.join(n).is_file(), "{n}");
    }
}
