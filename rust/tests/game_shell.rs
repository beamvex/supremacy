//! The UI shell port — `shell_enter` install order, `shell_step`
//! phase order, the `0x5D52` row click, the `0x6B4A` blink timer, the
//! `0x6B8F` ticker, the `0x8DC6` oval, and the status prints.

mod common;

use common::rig::Rig;
use supremacy::game::{self, Call, Tick};

#[test]
fn enter_installs_menu_and_sounds() {
    let mut r = Rig::new();
    r.w16(0xA0AA, 9);
    game::shell_enter(&mut r.ctx());
    let snd = r
        .host
        .0
        .iter()
        .filter(|c| matches!(c, Call::Sound(0xA)))
        .count();
    assert_eq!(snd, 2);
    assert_eq!(r.r16(game::HOT_LIST), 0xA0AC);
    assert_eq!((r.r16(game::HOT_COUNT), r.r8(game::UIMODE)), (0x31, 0));
    assert_eq!(r.r16(game::HOT_SEL), 9);
    assert!(r.host.0.contains(&Call::Image(0)));
    assert!(r.host.0.contains(&Call::Native(0xA36A)));
}

#[test]
fn enter_muted_skips_sound() {
    let mut r = Rig::new();
    r.w8(game::SND_ON, 0xFF);
    game::shell_enter(&mut r.ctx());
    assert!(!r.host.0.iter().any(|c| matches!(c, Call::Sound(_))));
    assert!(!r.host.0.iter().any(|c| matches!(c, Call::Chan(..))));
}

#[test]
fn step_runs_pipeline() {
    let mut r = Rig::new();
    r.w16(0x9164, 2);
    let t = game::shell_step(&mut r.ctx());
    assert!(matches!(t, Tick::Idle(_) | Tick::Machine(_)));
    assert_eq!(r.r8(game::FRAME), 1);
    assert_eq!(r.host.0[0], Call::Ui(0x2CA9));
    assert!(r.host.0.contains(&Call::Native(0xA369)));
    // Mark the selected record: index 2 → base + 2·0x3A, flag +5.
    let rec = r.r16(0x9158) + 2 * 0x3A;
    assert_eq!(r.r8(usize::from(rec) + 5), 1);
}

#[test]
fn row_click_selects() {
    let mut r = Rig::new();
    r.w16(game::BUTTONS, 1);
    r.w16(game::CUR_X, 0xAC);
    r.w16(game::CUR_Y, 0x11 + (2 << 3) + 1);
    r.w16(0x91C6, 7);
    r.w16(game::SEQ_DELAY, 5);
    game::row_click(&mut r.ctx());
    assert_eq!(r.r16(0x9164), 2);
    let rec = r.r16(0x9158) + 2 * 0x3A;
    assert_eq!((r.r16(0x9184), r.r16(0x917C)), (rec, rec));
    assert_eq!(r.r8(usize::from(rec) + 5), 1);
    assert_eq!(r.r16(game::SEQ_DELAY), 0);
    assert!(r.host.0.contains(&Call::PlanetPanel));
}

#[test]
fn row_click_ignores_outside_and_repeat() {
    let mut r = Rig::new();
    r.w16(0x9164, 6);
    r.w16(game::BUTTONS, 1);
    r.w16(game::CUR_X, 0x20); // left of the strip
    game::row_click(&mut r.ctx());
    assert_eq!(r.r16(0x9164), 6);
    r.w16(game::CUR_X, 0xAC);
    r.w16(game::CUR_Y, 0x11 + (6 << 3)); // row 6 = current selection
    game::row_click(&mut r.ctx());
    assert!(r.host.0.is_empty());
}

#[test]
fn blink_phases() {
    let mut r = Rig::new();
    r.w8(0x91D1, 2);
    r.w8(0x91F2, 1); // hold: dec + slot, no phase bump
    game::shell_step(&mut r.ctx());
    assert_eq!((r.r8(0x91D1), r.r8(0x91F3)), (1, 0));
    assert!(r.host.0.contains(&Call::Slot(0x1270, 0)));
}

#[test]
fn blink_flash_strings() {
    let mut r = Rig::new();
    r.w8(0x91D1, 1);
    r.w8(0x91F3, 0x11);
    r.w8(game::FRAME, 0xFD); // &7 = 5 → the 0x77E3 string
    r.w8(0x77E3, b'X');
    r.w8(0x77E4, 0xFF);
    game::blink_step(&mut r.ctx());
    assert!(r.host.0.contains(&Call::Glyph(b'X', 0x4D, 0x67)));
    assert_eq!(r.r8(0x91D1), 1); // flash arm doesn't decrement
}

fn ticker_rig() -> Rig {
    let mut r = Rig::new();
    r.w16(0x91C2, 3); // records left
    r.w16(0x915C, 0x928D); // record cursor
    r
}

#[test]
fn ticker_types_chars() {
    let mut r = ticker_rig();
    r.w16(0x928D, 0x9300); // record 0 msg cursor
    r.w8(0x9300, b'A');
    r.w8(0x9301, 0xFF);
    game::ticker_step(&mut r.ctx());
    assert_eq!(r.host.0[0], Call::Glyph(b'A', 0, 0xB8));
    assert_eq!(r.r16(0x91C0), 1);
    assert_eq!(r.r16(0x928D), 0x9301); // msg cursor committed
    game::ticker_step(&mut r.ctx()); // 0xFF end-of-record
    assert_eq!(r.r16(0x91C2), 2);
    assert_eq!(r.r16(0x915C), 0x9291); // advanced to next record
}

#[test]
fn ticker_ops() {
    let mut r = ticker_rig();
    r.w16(0x928D, 0x9300);
    r.w8(0x9300, 0xFC); // delay op
    r.w8(0x9301, 9);
    game::ticker_step(&mut r.ctx());
    assert_eq!(r.r8(0x91D2), 9);
    game::ticker_step(&mut r.ctx());
    assert_eq!(r.r8(0x91D2), 8); // delay decrements, no output
    assert!(r.host.0.is_empty());
    r.w8(0x91D2, 0); // expire the delay
    r.w8(0x9302, 0xFD); // committed cursor already sits at the op
    game::ticker_step(&mut r.ctx());
    assert_eq!((r.r8(0x91D1), r.r16(0x91C0)), (1, 0x29));
}
