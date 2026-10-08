//! The dialog layer — `dlg_enter`/`dlg_step`/`dlg_close`, the
//! save/load actions, the `ds:0` filename sanitiser, the flag
//! actions, and the `0x8DC6` oval pass-through.

mod common;

use common::rig::Rig;
use supremacy::game::{self, Call, Loaded};

#[test]
fn dlg_enter_installs_and_draws() {
    let mut r = Rig::new();
    r.w16(0xB05A, 4);
    game::dlg_enter(&mut r.ctx());
    assert_eq!(r.r16(game::HOT_LIST), 0xB05C);
    assert_eq!((r.r16(game::HOT_COUNT), r.r16(game::HOT_SEL)), (3, 4));
    assert_eq!(r.r8(0x91F2), 0); // pump released the blink hold
    assert!(r.host.0.contains(&Call::Rect(0x14, 0x5A, 8, 0x66)));
    assert!(r.host.0.contains(&Call::Image(0x12B)));
    assert!(r.host.0.contains(&Call::Slot(0x1274, 0)));
}

#[test]
fn dlg_step_exits_on_right_click() {
    let mut r = Rig::new();
    assert!(!game::dlg_step(&mut r.ctx()));
    r.w16(game::BUTTONS, 2);
    assert!(game::dlg_step(&mut r.ctx()));
    game::dlg_close(&mut r.ctx());
    assert_eq!(r.r16(game::HOT_LIST), 0xA0AC); // main list restored
}

#[test]
fn save_game_ok_prints_saved() {
    let mut r = Rig::new();
    r.w8(0x7C2E, b'O');
    r.w8(0x7C2F, 0xFF);
    r.w8(0x7929, b'Z'); // error string stays silent on success
    r.w8(0x792A, 0xFF);
    r.keys(&[0x14, 0x12, 0x1F, 0x14, 0x1C]); // "TEST" + Enter
    assert!(game::save_game(&mut r.ctx()));
    let saved = r.files.files.get("TEST").expect("TEST save written");
    assert_eq!(&saved[..], &r.st.block[..]);
    assert!(r.host.0.contains(&Call::Glyph(b'O', 0x2A, 0xB1)));
    assert!(!r.host.0.contains(&Call::Glyph(b'Z', 0x2A, 0xB1)));
}

#[test]
fn save_game_err_prints_error() {
    let mut r = Rig::new();
    r.w8(0x7929, b'Z');
    r.w8(0x792A, 0xFF);
    r.keys(&[0x1C]); // Enter on an empty name → ""
    r.files.fail = true; // DOS carry-flag path
    assert!(!game::save_game(&mut r.ctx()));
    assert!(r.host.0.contains(&Call::Glyph(b'Z', 0x2A, 0xB1)));
}

#[test]
fn load_game_switches_on_91e5() {
    let mut r = Rig::new();
    // A saved block whose `[0x91E5]` post-load switch = 2 → Loaded::C.
    let mut blk = vec![0u8; 0x1F8B];
    blk[usize::from(0x91E5u16) - 0x7D39..][..2].copy_from_slice(&2u16.to_le_bytes());
    r.files.files.insert("S".to_owned(), blk);
    r.keys(&[0x1F, 0x1C]); // "S" + Enter
    assert_eq!(game::load_game(&mut r.ctx()), Some(Loaded::C));
    assert_eq!(r.r8(0x9CCA), 0); // slot-mode brackets cleared
    assert_eq!(r.r16(0x91E5), 2); // block loaded into state
    assert!(r.host.0.contains(&Call::PlanetPanel));
}

#[test]
fn input_name_sanitises() {
    let mut r = Rig::new();
    // 'N' (0x31), '\' key → '[' (0x2B), ' ' (0x39), 'Z' (0x2C), Enter.
    r.keys(&[0x31, 0x2B, 0x39, 0x2C, 0x1C]);
    game::input_name(&mut r.ctx());
    assert_eq!(r.r8(0), b'N');
    assert_eq!(r.r8(1), b'\\'); // '[' remapped
    assert_eq!(r.r8(2), 0); // space terminated
    assert_eq!((r.r8(0x4FCB), r.r8(0x4FCC)), (4, 9));
    assert!(r.host.0.contains(&Call::Mouse(0x14, 0x1E, 0)));
    assert!(r.host.0.contains(&Call::Mouse(0x14, 0x1F, 0)));
}

#[test]
fn confirm_exits_via_action_dispatch() {
    let mut r = Rig::new();
    // confirm list record 0 covers the cursor; `+0xC` = `cs:0x2F01`
    // (act_yes). menu::service dispatches it through actions.rs.
    let rec = 0xB09A;
    r.w16(rec, 0);
    r.w16(rec + 2, 0);
    r.w16(rec + 4, 0x140);
    r.w16(rec + 6, 0xC8);
    r.w16(rec + 0xC, 0x2F01);
    r.w16(game::CUR_X, 0x10);
    r.w16(game::CUR_Y, 0x10);
    r.w16(game::BUTTONS, 1); // left held
    r.w8(0x78B8, b'T');
    r.w8(0x78B9, 0xFF);
    game::confirm(&mut r.ctx());
    assert_eq!(r.r8(0x91DA), 1); // act_yes ran
    assert_eq!(r.r16(game::HOT_LIST), 0xB05C); // dlg list restored
    let titles = r
        .host
        .0
        .iter()
        .filter(|c| matches!(c, Call::Glyph(b'T', 0x2A, 0xB1)))
        .count();
    assert!(titles >= 2); // title + post-answer reprint
}

#[test]
fn hover_passes_outside_oval() {
    let mut r = Rig::new();
    r.w16(game::CUR_Y, 0x9B); // below the region
    game::hover_check(&mut r.ctx());
    assert_eq!(r.host.0, [Call::Slot(0x1278, 0)]);
}

#[test]
fn hover_swallows_inside_oval() {
    let mut r = Rig::new();
    r.w16(game::CUR_Y, 0x60);
    r.w8(0xA8 + (0x90 - 0x60), 0x10); // half-width at this row
    r.w16(game::CUR_X, 0x50);
    game::hover_check(&mut r.ctx());
    assert!(r.host.0.is_empty());
}
