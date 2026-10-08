//! The `0xA10F` menu service — hit-test dispatch, debounce, the `K`-mode
//! key branch, nav links and the cursor-snap ease.

mod common;

use common::rig::Rig;
use supremacy::game::{self, Call};

/// Two stacked hotspot records at `0x3000`, cursor inside record 1.
fn hotspots(r: &mut Rig) {
    r.w16(game::HOT_LIST, 0x3000);
    r.w16(game::HOT_COUNT, 2);
    r.w16(game::CUR_X, 50);
    r.w16(game::CUR_Y, 60);
    for i in 0..2usize {
        let s = 0x3000 + i * 0x14;
        r.w16(s, u16::try_from(10 + i * 40).unwrap_or(0)); // x1
        r.w16(s + 2, 10); // y1
        r.w16(s + 4, u16::try_from(40 + i * 40).unwrap_or(0)); // x2
        r.w16(s + 6, 90); // y2
        r.w16(s + 8, u16::try_from(0x10 + i).unwrap_or(0)); // pressed image
        r.w16(s + 0xC, u16::try_from(0x7770 + i).unwrap_or(0)); // action
    }
}

#[test]
fn hit_dispatches_action() {
    let mut r = Rig::new();
    hotspots(&mut r);
    r.w16(game::BUTTONS, 1);
    r.w16(game::HOT_SEL, 3);
    game::menu_service(&mut r.ctx());
    assert_eq!(r.host.0, [Call::Image(0x11), Call::Native(0x7771)]);
    assert_eq!(r.r8(game::ARMED), 1);
    assert_eq!(r.r16(0x3000 - 2), 3); // selection stashed at list-2
    assert_eq!(r.r16(game::HOT_IMG), 0x11);
}

#[test]
fn debounce_and_release() {
    let mut r = Rig::new();
    hotspots(&mut r);
    r.w16(game::BUTTONS, 1);
    game::menu_service(&mut r.ctx());
    let n = r.host.0.len();
    game::menu_service(&mut r.ctx()); // still held → armed blocks re-fire
    assert_eq!(r.host.0.len(), n);
    r.w16(game::BUTTONS, 0); // release edge clears the latch
    game::menu_service(&mut r.ctx());
    assert_eq!(r.r8(game::ARMED), 0);
    r.w16(game::BUTTONS, 1);
    game::menu_service(&mut r.ctx()); // fresh press fires again
    assert_eq!(r.host.0.len(), n + 2);
}

#[test]
fn k_mode_enter_presses_left() {
    let mut r = Rig::new();
    r.w8(game::K_MODE, 1);
    r.w8(game::KEY_PEND, 1);
    r.w8(game::KEY_CODE, 0x1C); // Enter
    game::menu_service(&mut r.ctx());
    assert_eq!(r.r16(game::BUTTONS), 1);
    assert_eq!(r.r8(game::KEY_PEND), 0);
}

#[test]
fn nav_follows_link_and_ff_stays() {
    let mut r = Rig::new();
    hotspots(&mut r);
    r.w8(game::K_MODE, 1);
    r.w16(game::HOT_SEL, 0);
    r.w8(0x3000 + 0x11, 1); // record 0 down-link → record 1
    r.w8(0x3000 + 0x10, 0xFF); // up-link stays
    r.w8(game::KEY_PEND, 1);
    r.w8(game::KEY_CODE, 0x50); // down
    game::menu_service(&mut r.ctx());
    assert_eq!(r.r16(game::HOT_SEL), 1);
    r.w8(game::KEY_PEND, 1);
    r.w8(game::KEY_CODE, 0x48); // up — record 1's link is 0 (real)
    game::menu_service(&mut r.ctx());
    assert_eq!(r.r16(game::HOT_SEL), 0);
}

#[test]
fn snap_eases_cursor_to_centre() {
    let mut r = Rig::new();
    hotspots(&mut r);
    r.w8(game::K_MODE, 1);
    r.w16(game::HOT_SEL, 0); // centre of rec 0 = (25, 50)
    r.w16(game::CUR_X, 0);
    r.w16(game::CUR_Y, 0);
    game::menu_service(&mut r.ctx()); // no pending key → snap
                                      // x eases 0 → 0 + ((25 >> 3) | 1) = 3; y eases 0 → (50 >> 3) | 1 = 7
    assert_eq!(r.r16(game::CUR_X), 3);
    assert_eq!(r.r16(game::CUR_Y), 7);
    assert_eq!(r.host.0.last(), Some(&Call::Native(0xA497)));
}
