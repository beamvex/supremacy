//! Game-state: the save block round-trip, the `cs:0x55B2` LCG against an
//! independent 16-bit-half model, and the new-game record init.

mod common;

use supremacy::game::{self, Rng, State};
use supremacy::platform::HostFs;

/// Reference model of file `0x2E863`–`0x2E93B` — the scramble done on four
/// 16-bit halves exactly as the asm sequences the `add`/`adc`/`xchg`s.
/// Returns `(new_s, folded)`.
fn model_step(s: u32) -> (u32, u16) {
    let (mut al, mut ah) = (
        u16::try_from(s & 0xFFFF).unwrap_or(0),
        u16::try_from(s >> 16).unwrap_or(0),
    );
    let (mut pl, mut ph) = (al, ah);
    let (n, c) = pl.overflowing_add(7);
    pl = n;
    ph = ph.wrapping_add(u16::from(c));
    for _ in 0..2 {
        let (n, c) = al.overflowing_add(al);
        al = n;
        ah = ah.wrapping_add(ah).wrapping_add(u16::from(c));
    }
    let (ml, mh) = (al, ah);
    let (n, c) = al.overflowing_add(al);
    al = n;
    ah = ah.wrapping_add(ah).wrapping_add(u16::from(c));
    let (n, c) = al.overflowing_add(pl);
    al = n;
    ah = ah.wrapping_add(ph).wrapping_add(u16::from(c));
    let (n, c) = al.overflowing_add(ml);
    al = n;
    ah = ah.wrapping_add(mh).wrapping_add(u16::from(c));
    (u32::from(al) | (u32::from(ah) << 16), al ^ ah)
}

fn model_range(s: &mut u32, b: u16) -> u16 {
    let (ns, r) = model_step(*s);
    *s = ns;
    u16::try_from(((u32::from(b) + 1) * u32::from(r)) >> 16).unwrap_or(0)
}

#[test]
fn rng_matches_the_16bit_model() {
    for seed in [0u32, 1, 0x1234_5678, 0xFFFF_FFFF, 0x8000_0001] {
        let mut port = Rng::new(seed);
        let mut s = seed;
        for b in [0x4E20u16, 0x3E8, 0x1F4, 0xFFFF, 0] {
            assert_eq!(
                port.range(b),
                model_range(&mut s, b),
                "seed {seed:#x} b {b:#x}"
            );
        }
    }
}

#[test]
fn rng_folded_word() {
    let mut port = Rng::new(0xDEAD_BEEF);
    let mut s = 0xDEAD_BEEF;
    for _ in 0..8 {
        let (ns, r) = model_step(s);
        s = ns;
        assert_eq!(port.next16(), r);
    }
}

#[test]
fn save_is_exactly_the_state_block() {
    let dir = std::env::temp_dir().join(format!("supremacy-test-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let mut fs = HostFs::new(dir.clone());
    let mut st = State::new();
    st.set_dword(game::REC_BASE + 0x36, 0x1879A);
    assert!(st.save(&mut fs, "TEST.SAV"));
    let on_disk = std::fs::read(dir.join("TEST.SAV")).unwrap();
    assert_eq!(on_disk.len(), game::STATE_LEN);
    assert_eq!(on_disk, st.block[..]);
    let mut back = State::new();
    assert!(back.load(&mut fs, "TEST.SAV"));
    assert_eq!(back.block, st.block);
    std::fs::remove_file(dir.join("TEST.SAV")).unwrap();
}

#[test]
fn state_mirrors_shipped_image() {
    let img = common::read(&common::root().join("decompiled/GAME.unpacked.exe"));
    let st = State::from_image(&img);
    // ds:0x7D39 = file 0x189D9 — the string workspace starts with
    // "  \xFDFORMAT COMPLETE.\xFD" (0xFD-separated messages).
    assert_eq!(&st.block[3..0x13], b"FORMAT COMPLETE.");
    assert_eq!(st.byte(0x7D39), b' ');
}

#[test]
fn preset_and_new_game() {
    let mut st = State::new();
    game::select_galaxy(&mut st, 2); // SMINE: 0x10 records, diff 1
    assert_eq!(st.word(game::REC_BASE), 0x8BC6);
    assert_eq!(st.word(game::REC_TOTAL), 0x10);
    assert_eq!(st.planets(), 0x0F);
    assert_eq!(st.byte(game::DIFFICULTY), 1);
    let mut rng = Rng::new(0x1111_2222);
    let mut vm = game::Vm::new();
    // The asm flow snapshots configured state (`0x844B`) before the
    // `0x8432` restore inside `new_game` — mirror it so the preset
    // survives.
    vm.stage_save(&st);
    game::new_game(&mut vm, &mut st, &mut rng);
    let base = usize::from(st.word(game::REC_BASE));
    // record 0 — fixed block
    assert_eq!(st.dword(base + game::F_CREDITS), 0x1879A);
    assert_eq!(st.byte(base + game::F_OWNER), 2);
    // planets 1..n-1 are LIFELESS!
    for i in 1..st.planets() {
        let r = usize::from(st.rec_ofs(i));
        assert_eq!(
            &st.block[r + game::F_NAME - game::STATE_OFS..][..9],
            b"LIFELESS!"
        );
        assert_eq!(st.byte(r + game::F_SERIAL), u8::try_from(i).unwrap());
    }
    // faction record — rolled resources
    let f = usize::from(st.rec_ofs(st.planets()));
    let credits = st.dword(f + game::F_CREDITS);
    assert!((0xC350..=0xC350 + 0x4E20).contains(&credits));
    assert_eq!(st.byte(f + game::F_OWNER), 2);
    let r1 = st.word(f + game::F_FOOD);
    let r2 = st.word(f + game::F_MINERALS);
    let r3 = st.word(f + game::F_FUEL);
    let r4 = st.word(f + game::F_ENERGY);
    assert!(r2 - r1 == r3 - r2 && r3 - r2 == r4 - r3); // arithmetic series
}
