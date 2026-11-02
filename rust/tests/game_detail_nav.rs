//! The detail screen's cross-screen row — records 3–7 of the `0x9EDE`
//! list (build/status/orders/galaxy/fleet). The hit-test is driven
//! through `menu::service` from inside the screen's own frame loop, so
//! these cover the dispatch *and* the frame unwind the galaxy entry
//! needs (it has no loop of its own — the runner's `Phase::Galaxy`
//! drives it off `uimode 5`).

mod common;

use common::rig::Rig;
use supremacy::game::{self, Call};

/// `ds:` base of the live detail list and its record stride.
const LIST: usize = 0x9EDE;
/// Record stride (`0x14`) — `menu::service`'s `HOT_STRIDE`.
const STRIDE: usize = 0x14;

/// Copy the shipped `0x9EDE` records 3–7 out of the exe image into the
/// rig's zeroed `hi` region — the geometry *and* the `+0xC` action
/// words come from the data the game runs on, not from the test.
fn seed_nav(r: &mut Rig) {
    let img = supremacy::img::image();
    for n in 3..8 {
        let s = LIST + n * STRIDE;
        for b in 0..STRIDE {
            r.w8(s + b, img[game::DS_BASE + s + b]);
        }
    }
}

/// Record 0 colonised and selected, the nav records seeded, and the
/// cursor parked on the centre of nav record `n` with both buttons
/// latched: the left press fires the hit-test on the screen's first
/// frame and the `[0x9CDA] & 2` edge exits every loop the hit opens,
/// so the entry call returns instead of spinning.
fn armed(n: usize) -> Rig {
    let mut r = Rig::new();
    let rec = r.r16(game::REC_BASE);
    r.w16(game::REC_CUR, rec);
    r.w16(usize::from(rec) + 0xC, 0xA);
    seed_nav(&mut r);
    let s = LIST + n * STRIDE;
    let (x1, x2) = (r.r16(s), r.r16(s + 4));
    let (y1, y2) = (r.r16(s + 2), r.r16(s + 6));
    r.w16(game::CUR_X, x1 + (x2 - x1) / 2);
    r.w16(game::CUR_Y, y1 + (y2 - y1) / 2);
    r.w16(game::BUTTONS, 3);
    r
}

/// [`armed`] + the `0x9179` planet entry.
fn click(n: usize) -> Rig {
    let mut r = armed(n);
    game::dispatch_action(&mut r.ctx(), 0x9179);
    r
}

/// [`armed`] + a docked machine selected and the `0x918D`
/// machine-detail entry — the `0x8E10` variant body over the same
/// shared frame loop.
fn mach_click(n: usize) -> Rig {
    let mut r = armed(n);
    let m = u16::try_from(game::MACH_BASE).unwrap_or(0);
    r.w8(usize::from(m) + game::M_TYPE, 3);
    r.w8(usize::from(m) + 0x21, 0); // `+0x21` host index → record 0
    r.w16(game::SEL_MACH, m);
    game::dispatch_action(&mut r.ctx(), 0x918D);
    r
}

/// Backdrop image index each destination screen draws on entry.
const BACKDROP: [(usize, u16, &str); 4] = [
    (3, 0x2A, "build"),
    (4, 0x02, "status"),
    (5, 0x40, "orders"),
    (7, 0x27, "fleet"),
];

#[test]
fn nav_records_open_their_screens() {
    for (n, img, name) in BACKDROP {
        let r = click(n);
        assert!(r.host.0.contains(&Call::Present(2)), "{name}: detail");
        assert!(r.host.0.contains(&Call::Image(img)), "{name}: backdrop");
    }
}

#[test]
fn nav_galaxy_record_hands_the_loop_to_the_runner() {
    let mut r = click(6);
    assert_eq!(r.r8(game::UIMODE), 5); // the runner's `Phase::Galaxy`
    assert_eq!(r.r16(game::HOT_LIST), 0xA81C); // the sidebar list
    assert_eq!(r.r16(game::HOT_COUNT), 3);
    // The frame was abandoned like the asm's `pop ax` — no `0x2CC5`
    // shell re-entry stamped `uimode 0` over the hand-off.
    assert!(!r.host.0.contains(&Call::Image(0)));
}

#[test]
fn mach_entry_nav_records_open_their_screens() {
    for (n, img, name) in BACKDROP {
        let r = mach_click(n);
        assert!(r.host.0.contains(&Call::Present(3)), "{name}: 0x2458");
        assert!(r.host.0.contains(&Call::Image(img)), "{name}: backdrop");
    }
}

#[test]
fn mach_entry_galaxy_record_hands_the_loop_over() {
    let mut r = mach_click(6);
    assert!(r.host.0.contains(&Call::Present(3))); // the 0x2458 head
    assert_eq!(r.r8(game::UIMODE), 5);
    assert_eq!(r.r16(game::HOT_LIST), 0xA81C);
    assert_eq!(r.r16(game::HOT_COUNT), 3);
}

#[test]
fn right_click_still_exits_to_the_shell() {
    let mut r = Rig::new();
    let rec = r.r16(game::REC_BASE);
    r.w16(game::REC_CUR, rec);
    r.w16(usize::from(rec) + 0xC, 0xA);
    seed_nav(&mut r);
    r.w16(game::CUR_X, 0x10); // over the build icon
    r.w16(game::CUR_Y, 0xB4);
    r.w16(game::BUTTONS, 2); // right button only — no hit-test
    game::dispatch_action(&mut r.ctx(), 0x9179);
    assert!(!r.host.0.contains(&Call::Image(0x2A)));
    assert_eq!(r.r8(game::UIMODE), 0); // back in the shell
    assert_eq!(r.r16(game::HOT_LIST), 0xA0AC);
}
