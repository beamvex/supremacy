//! The uimode-4 build screen (list `ds:0xA482`, GAM-81) — the entry's
//! current-planet name + machine-list paints, and the `0xA482`
//! records' actions through `screens::dispatch` (`bmach_sel`,
//! `build_xfer`, `build_orbit`, the `0x540C` fly modal's cancel
//! tail).

mod common;

use common::rig::Rig;
use supremacy::game::{self, Call};

/// `ds:` slot of the `+0x21` host-index field.
const M_HOST: usize = 0x21;
/// `ds:` slot of a planet record's serial index.
const F_SERIAL: usize = 0x28;

/// Colonised record, machine 0 in-use on it, bay A docked, right-exit
/// armed for the frame loop.
fn rig() -> Rig {
    let mut r = Rig::new();
    let rec = r.r16(game::REC_BASE);
    r.w16(game::REC_CUR, rec);
    r.w16(game::SEL_REC, rec);
    r.w16(usize::from(rec) + 0xC, 0xA);
    for (i, &ch) in b"HOME....".iter().enumerate() {
        r.w8(usize::from(rec) + 0xE + i, ch);
    }
    r.w8(usize::from(rec) + 0x16, 0xFF);
    let m = u16::try_from(game::MACH_BASE).unwrap_or(0);
    for (i, &ch) in b"MINER...".iter().enumerate() {
        r.w8(usize::from(m) + i, ch);
    }
    r.w8(usize::from(m) + 8, 0xFF);
    r.w8(usize::from(m) + game::M_TYPE, 6);
    let serial = r.r8(usize::from(rec) + F_SERIAL);
    r.w8(usize::from(m) + M_HOST, serial);
    r.w8(usize::from(rec) + 0x18, 1);
    r.w8(usize::from(rec) + 0x1C, 0);
    r.w8(usize::from(rec) + 0x20, 0);
    r.w16(game::BUTTONS, 2);
    r
}

/// `true` when `want` prints left-to-right on row `dx` from `bx`.
fn row_has(r: &Rig, want: &[u8], bx: u16, dx: u16) -> bool {
    want.iter().enumerate().all(|(i, &ch)| {
        let b = bx.wrapping_add(u16::try_from(i).unwrap_or(0));
        r.host.0.contains(&Call::Glyph(ch, b, dx))
    })
}

#[test]
fn build_entry_paints_planet_and_machine_list() {
    let mut r = rig();
    game::dispatch_action(&mut r.ctx(), 0xB8F7);
    assert!(row_has(&r, b"HOME", 3, 0xE), "current planet name");
    assert!(row_has(&r, b"MINER...", 8, 0x83), "machine list cell");
    assert!(!r.host.0.contains(&Call::Native(0x4EB6)));
    assert!(!r.host.0.contains(&Call::Native(0x6FBF)));
    assert!(!r.host.0.contains(&Call::Native(0x4C8B)));
}

#[test]
fn bmach_sel_picks_docked_unit() {
    let mut r = rig();
    let m = u16::try_from(game::MACH_BASE).unwrap_or(0);
    game::dispatch_action(&mut r.ctx(), 0xBEE7);
    assert_eq!(r.r16(game::SEL_MACH), m);
    assert_ne!(r.r16(game::DIRTY0) & 0x3000, 0);
}

#[test]
fn build_xfer_docks_orbiting_unit() {
    let mut r = rig();
    let rec = r.r16(game::SEL_REC);
    let m = u16::try_from(game::MACH_BASE).unwrap_or(0);
    r.w16(usize::from(rec) + 0x24, 2); // owner
    r.w16(game::SEL_MACH, m);
    r.w8(usize::from(m) + game::M_FLAGS, 4); // in orbit
    r.w8(usize::from(rec) + 0x18, 0); // all bays free
    game::dispatch_action(&mut r.ctx(), 0xBF45);
    assert_eq!(r.r8(usize::from(rec) + 0x18), 1, "unit index in bay A");
    assert_eq!(r.r8(usize::from(m) + game::M_FLAGS) & 4, 0);
}

#[test]
fn build_orbit_releases_docked_unit() {
    let mut r = rig();
    let rec = r.r16(game::SEL_REC);
    let m = u16::try_from(game::MACH_BASE).unwrap_or(0);
    r.w16(game::SEL_MACH, m);
    // Type record for kind 6 (0x9B12 + 5·0x30): remote + nuclear —
    // skips the fuel/crew guards.
    r.w16(0x9C02 + 0xA, 0xFFFE);
    r.w16(0x9C02 + 0xC, 0xFFFF);
    game::dispatch_action(&mut r.ctx(), 0xC019);
    assert_eq!(r.r8(usize::from(rec) + 0x18), 0, "bay A released");
    assert_ne!(r.r8(usize::from(m) + game::M_FLAGS) & 4, 0, "in orbit");
}

#[test]
fn build_fly_cancel_restores_selection() {
    let mut r = rig();
    let rec = r.r16(game::SEL_REC);
    let m = u16::try_from(game::MACH_BASE).unwrap_or(0);
    r.w16(game::SEL_MACH, m);
    r.w8(usize::from(m) + game::M_FLAGS, 4); // in orbit
                                             // BUTTONS = 2 from rig() → the modal pick's right-click abandon.
    game::dispatch_action(&mut r.ctx(), 0xC15C);
    assert_eq!(r.r16(game::SEL_REC), rec, "selection restored");
    assert_eq!(r.r8(game::LAUNCH_F), 0, "pick disarmed");
    assert_eq!(r.r8(game::UIMODE), 0, "abandoned to the shell");
}
