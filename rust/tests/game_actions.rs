//! Menu-action dispatch — the `0x286B`/game-window aliases, the
//! screen entries, the standalone actions, and the `uimode` phase
//! contract the runner keys on.

mod common;

use common::rig::Rig;
use supremacy::game::{self, Call};

/// `ds:` offset of the ticker queue's first record slot.
const Q0: usize = 0x928D;

/// A single hotspot at `0x3000` covering the cursor, action `act`.
fn hotspot(r: &mut Rig, act: u16) {
    r.w16(game::HOT_LIST, 0x3000);
    r.w16(game::HOT_COUNT, 1);
    r.w16(game::CUR_X, 20);
    r.w16(game::CUR_Y, 20);
    for (o, v) in [
        (0usize, 10u16),
        (2, 10),
        (4, 40),
        (6, 90),
        (8, 0x10),
        (0xC, act),
    ] {
        r.w16(0x3000 + o, v);
    }
}

#[test]
fn stub_clears_armed_in_both_windows() {
    for t in [0x8544u16, 0xF294] {
        let mut r = Rig::new();
        r.w8(game::ARMED, 1);
        game::dispatch_action(&mut r.ctx(), t);
        assert_eq!(r.r8(game::ARMED), 0);
        assert!(r.host.0.is_empty());
    }
}

#[test]
fn confirm_aliases_set_the_flag() {
    for t in [0x2F01u16, 0x9C51] {
        let mut r = Rig::new();
        game::dispatch_action(&mut r.ctx(), t);
        assert_eq!(r.r8(game::DLG_FLAG), 1);
    }
    for t in [0x2F08u16, 0x9C58] {
        let mut r = Rig::new();
        game::dispatch_action(&mut r.ctx(), t);
        assert_eq!(r.r8(game::DLG_FLAG), 2);
    }
    for t in [0x2F95u16, 0x9CE5] {
        let mut r = Rig::new();
        game::dispatch_action(&mut r.ctx(), t);
        assert_eq!(r.r8(game::DLG_FLAG), 0xFF);
    }
}

#[test]
fn galaxy_entry_installs_uimode5() {
    let mut r = Rig::new();
    game::dispatch_action(&mut r.ctx(), 0xA656);
    assert_eq!(r.r8(game::UIMODE), 5);
    assert_eq!(r.r16(game::HOT_LIST), 0xA81C);
    assert_eq!(r.r16(game::HOT_COUNT), 3);
    let faction = r
        .r16(game::REC_BASE)
        .wrapping_add(r.r16(game::REC_COUNT).wrapping_mul(0x3A));
    assert_eq!(r.r16(game::SEL_REC), faction);
    assert_eq!(r.r8(game::REDRAW), 0xFF);
    assert_eq!(r.r8(game::PANEL_REQ), 0xFF);
    assert_eq!(r.r8(game::PANEL_ON), 0);
    assert_eq!(r.r8(game::ARMED), 0);
}

#[test]
fn galaxy_window_variant_opens_the_window() {
    let mut r = Rig::new();
    game::dispatch_action(&mut r.ctx(), 0xA659);
    assert_eq!(r.host.0.first(), Some(&Call::Slot(0x1274, 0)));
    assert_eq!(r.r8(game::UIMODE), 5);
}

#[test]
fn hotspot_click_enters_galaxy() {
    let mut r = Rig::new();
    hotspot(&mut r, 0xA656); // shipped-record window
    r.w16(game::BUTTONS, 1);
    game::menu_service(&mut r.ctx());
    assert_eq!(r.r8(game::UIMODE), 5);
    assert_eq!(r.r16(game::HOT_LIST), 0xA81C);
}

#[test]
fn detail_refusal_enqueues_and_reenters_shell() {
    let mut r = Rig::new();
    let rec = r.r16(game::REC_BASE);
    r.w16(game::REC_CUR, rec);
    r.w16(usize::from(rec) + 0xC, 7); // not colonised
    r.w16(game::TICK_PUT, u16::try_from(Q0).unwrap_or(0));
    game::dispatch_action(&mut r.ctx(), 0x9179);
    assert_eq!(r.r16(Q0), 0x7402); // refusal message queued
    assert_eq!(r.r8(game::UIMODE), 0); // shell re-entry installed
    assert_eq!(r.r16(game::HOT_LIST), 0xA0AC);
}

#[test]
fn detail_entry_runs_one_frame_then_leaves() {
    let mut r = Rig::new();
    let rec = r.r16(game::REC_BASE);
    r.w16(game::REC_CUR, rec);
    r.w16(usize::from(rec) + 0xC, 0xA); // colonised
    r.w16(game::BUTTONS, 2); // right button — exit edge
    game::dispatch_action(&mut r.ctx(), 0x2429);
    assert!(r.host.0.contains(&Call::Image(0x29)));
    assert!(r.host.0.contains(&Call::Present(2)));
    assert_eq!(r.r8(game::UIMODE), 0); // leave → shell_enter
    assert_eq!(r.r16(game::HOT_LIST), 0xA0AC);
}

#[test]
fn snd_toggle_flips_and_reports() {
    let mut r = Rig::new();
    r.w16(game::TICK_PUT, u16::try_from(Q0).unwrap_or(0));
    r.w8(game::SND_ON, 0);
    game::dispatch_action(&mut r.ctx(), 0xF29A);
    assert_eq!(r.r8(game::SND_ON), 0xFF);
    assert_eq!(r.r16(Q0), 0x7C94); // "off" message
    game::dispatch_action(&mut r.ctx(), 0x854A);
    assert_eq!(r.r8(game::SND_ON), 0);
    assert_eq!(r.r16(Q0 + 4), 0x7CA9); // "on" message at the next slot
}

#[test]
fn sel_prev_wraps_to_list_max() {
    let mut r = Rig::new();
    r.w16(game::SEL_IDX, 0);
    r.w8(game::LIST_MAX, 5);
    game::dispatch_action(&mut r.ctx(), 0xCAF1);
    assert_eq!(r.r16(game::SEL_IDX), 5);
}

#[test]
fn sync_sel_recomputes_the_index() {
    let mut r = Rig::new();
    let rec = r.r16(game::REC_BASE).wrapping_add(3 * 0x3A);
    r.w16(game::SEL_REC, rec);
    game::sync_sel(&mut r.ctx());
    assert_eq!(r.r16(game::SEL_IDX), 3);
    assert_eq!(r.r16(game::REC_CUR), rec);
    assert_eq!(r.r8(usize::from(rec) + 5), 1); // re-marked
}

#[test]
fn unknown_action_stays_native() {
    let mut r = Rig::new();
    game::dispatch_action(&mut r.ctx(), 0x7771);
    assert_eq!(r.host.0, [Call::Native(0x7771)]);
}

#[test]
fn cell_pick_arms_the_key_pair() {
    for t in [0x8539u16, 0xF289] {
        let mut r = Rig::new();
        game::dispatch_action(&mut r.ctx(), t);
        assert_eq!(r.r8(game::KEY_CODE), 0x1C);
        assert_eq!(r.r8(game::KEY_PEND), 1);
        assert!(r.host.0.is_empty());
    }
}

#[test]
fn mode_pick_writes_its_index() {
    let mut r = Rig::new();
    game::dispatch_action(&mut r.ctx(), 0xEE34);
    assert_eq!(r.r8(0x91DC), 4);
    game::dispatch_action(&mut r.ctx(), 0x80EA);
    assert_eq!(r.r8(0x91DC), 5);
}

#[test]
fn sel_faction_points_at_the_faction_record() {
    let mut r = Rig::new();
    let faction = r
        .r16(game::REC_BASE)
        .wrapping_add(r.r16(game::REC_COUNT).wrapping_mul(0x3A));
    game::dispatch_action(&mut r.ctx(), 0xCBAA);
    assert_eq!(r.r16(game::SEL_REC), faction);
    assert_eq!(r.r16(game::REC_CUR), faction);
    assert_eq!(r.r16(game::SEL_IDX), r.r16(game::REC_COUNT) & 0xFF);
    assert!(r.host.0.contains(&Call::Rect(0x18, 0x13, 0x3, 0x33)));
}

#[test]
fn detail_mach_falls_back_without_a_machine() {
    let mut r = Rig::new();
    let rec = r.r16(game::REC_BASE);
    r.w16(game::REC_CUR, rec);
    r.w16(usize::from(rec) + 0xC, 0xA);
    r.w16(game::BUTTONS, 2); // exit edge so the loop returns
    r.w16(game::SEL_MACH, 0);
    game::dispatch_action(&mut r.ctx(), 0x918D);
    assert!(r.host.0.contains(&Call::Present(2))); // planet path
    assert!(!r.host.0.contains(&Call::Native(0x29D9)));
}

#[test]
fn detail_mach_enters_the_machine_screen() {
    let mut r = Rig::new();
    let rec = r.r16(game::REC_BASE);
    r.w16(usize::from(rec) + 0xC, 0xA); // host record, colonised
    let m = u16::try_from(game::MACH_BASE).unwrap_or(0);
    r.w16(game::SEL_MACH, m);
    r.w8(usize::from(m) + 0x21, 0); // +0x21 → host index 0
    r.w16(game::BUTTONS, 2);
    game::dispatch_action(&mut r.ctx(), 0x243D);
    assert!(r.host.0.contains(&Call::Present(3)));
    assert!(r.host.0.contains(&Call::Native(0x29D9)));
    assert_eq!(r.r8(game::UIMODE), 0); // frame loop exited → shell
}

#[test]
fn surface_refuses_a_bare_kind7() {
    let mut r = Rig::new();
    let rec = r.r16(game::REC_BASE);
    r.w16(game::REC_CUR, rec);
    r.w16(usize::from(rec) + 0xC, 7); // outpost, nothing aboard
    r.w16(game::TICK_PUT, u16::try_from(Q0).unwrap_or(0));
    game::dispatch_action(&mut r.ctx(), 0xAD86);
    assert_eq!(r.r16(Q0), 0x7402);
    assert_eq!(r.r8(game::UIMODE), 0); // shell re-entry
}

#[test]
fn surface_entry_installs_uimode7() {
    let mut r = Rig::new();
    let rec = r.r16(game::REC_BASE);
    r.w16(game::REC_CUR, rec);
    r.w16(usize::from(rec) + 0xC, 0xA); // colonised
    r.w16(game::BUTTONS, 2);
    game::dispatch_action(&mut r.ctx(), 0x4036);
    assert_eq!(r.host.0.first(), Some(&Call::Slot(0x1274, 0)));
    assert!(r.host.0.contains(&Call::Image(0x57)));
    assert!(r.host.0.contains(&Call::Present(7)));
    assert!(r.host.0.contains(&Call::Rect(0x1C, 6, 3, 0x41)));
    assert!(r.host.0.contains(&Call::Native(0x4190))); // table build
    assert_eq!(r.r16(game::HOT_LIST), 0xA0AC); // leave → shell list
    assert_eq!(r.r8(game::UIMODE), 0); // frame loop exited → shell
}

#[test]
fn surface_messages_a_kind4() {
    let mut r = Rig::new();
    let rec = r.r16(game::REC_BASE);
    r.w16(game::REC_CUR, rec);
    r.w16(usize::from(rec) + 0xC, 4);
    r.w16(game::TICK_PUT, u16::try_from(Q0).unwrap_or(0));
    game::dispatch_action(&mut r.ctx(), 0xAD86);
    assert_eq!(r.r16(Q0), 0x6AD6);
    assert!(!r.host.0.contains(&Call::Image(0x57)));
}

#[test]
fn view_steps_cycle_and_skip_one() {
    let mut r = Rig::new();
    game::dispatch_action(&mut r.ctx(), 0xD398); // prev: 0 → 6
    assert_eq!(r.r8(game::TYPE_SEL), 6);
    game::dispatch_action(&mut r.ctx(), 0x6648); // label window: 6 → 5
    assert_eq!(r.r8(game::TYPE_SEL), 5);
    game::dispatch_action(&mut r.ctx(), 0xD398); // 5 → 4
    assert_eq!(r.r8(game::TYPE_SEL), 4);
    game::dispatch_action(&mut r.ctx(), 0xD3E3); // next: 4 → 5
    assert_eq!(r.r8(game::TYPE_SEL), 5);
    r.w8(game::TYPE_SEL, 0);
    game::dispatch_action(&mut r.ctx(), 0x6693); // 0 → skips 1 → 2
    assert_eq!(r.r8(game::TYPE_SEL), 2);
    assert_eq!(r.r8(game::REDRAW), 0xFF);
    assert_eq!(r.r8(game::PANEL_REQ), 0xFF);
}

#[test]
fn launch_without_carrier_reports() {
    let mut r = Rig::new();
    r.w16(game::TICK_PUT, u16::try_from(Q0).unwrap_or(0));
    game::dispatch_action(&mut r.ctx(), 0xDF33);
    assert_eq!(r.r16(Q0), 0x7130);
    assert_eq!(r.r16(Q0 + 4), 0x7123);
    assert!(!r.host.0.contains(&Call::Native(0xDF33)));
}

#[test]
fn launch_transit_reports_the_unit() {
    let mut r = Rig::new();
    let m = u16::try_from(game::MACH_BASE).unwrap_or(0);
    r.w8(usize::from(m) + game::M_TYPE, 4);
    r.w8(usize::from(m) + game::M_FLAGS, 1); // in transit
    r.w16(usize::from(m) + 0x18, 0); // empty
    r.w16(game::TICK_PUT, u16::try_from(Q0).unwrap_or(0));
    game::dispatch_action(&mut r.ctx(), 0x71E3);
    assert_eq!(r.r16(Q0), 0x683A);
    assert_eq!(r.r16(game::REINF_REC), m);
}

#[test]
fn spy_menu_kind4_reports_and_tails() {
    let mut r = Rig::new();
    let rec = r.r16(game::REC_BASE);
    r.w16(game::REC_CUR, rec);
    r.w16(usize::from(rec) + 0xC, 4);
    r.w16(game::TICK_PUT, u16::try_from(Q0).unwrap_or(0));
    r.w16(game::TICK_PTR, u16::try_from(Q0).unwrap_or(0));
    r.w8(0x762C, 0xFF); // record terminators so the pump drains
    r.w8(0x712C, 0xFF);
    game::dispatch_action(&mut r.ctx(), 0xEE99);
    assert_eq!(r.r16(Q0), 0x762C);
    assert_eq!(r.r16(Q0 + 4), 0x712C);
    assert_eq!(r.r8(0x81FE), 0xFF); // report flag armed by the tail
    assert!(!r.host.0.contains(&Call::Native(0xEE99)));
}

#[test]
fn order_open_enters_surface_for_the_sel() {
    let mut r = Rig::new();
    let rec = r.r16(game::REC_BASE);
    r.w16(usize::from(rec) + 0xC, 0xA); // colonised
    r.w16(game::SEL_REC, rec);
    r.w16(game::BUTTONS, 2);
    game::dispatch_action(&mut r.ctx(), 0xAC91);
    assert!(r.host.0.contains(&Call::Present(7)));
    assert_eq!(r.r8(game::UIMODE), 0); // leave → shell
    assert_eq!(r.r16(game::HOT_LIST), 0xA0AC);
}

#[test]
fn order_sel_follows_the_row_link() {
    let mut r = Rig::new();
    let rec = r.r16(game::REC_BASE).wrapping_add(2 * 0x3A);
    r.w16(usize::from(rec) + 0xC, 0xA);
    r.w16(game::LINK_REC, 0x3000);
    r.w16(0x3002, rec); // row record at link +2
    r.w16(game::BUTTONS, 2);
    game::dispatch_action(&mut r.ctx(), 0x3F18);
    assert!(r.host.0.contains(&Call::Present(7)));
}

#[test]
fn mach_open_enters_the_host_surface() {
    let mut r = Rig::new();
    let rec = r.r16(game::REC_BASE);
    r.w16(usize::from(rec) + 0xC, 0xA); // host colonised
    let m = u16::try_from(game::MACH_BASE).unwrap_or(0);
    r.w8(usize::from(m) + game::M_TYPE, 2); // not DAC
    r.w8(usize::from(m) + game::M_HOST, 0); // host = record 0
    r.w16(game::SEL_MACH, m);
    r.w16(game::BUTTONS, 2);
    game::dispatch_action(&mut r.ctx(), 0xACE1);
    assert_eq!(r.r16(game::SEL_REC), rec);
    assert!(r.host.0.contains(&Call::Present(7)));
}

#[test]
fn order_build_copies_sel_into_cur() {
    let mut r = Rig::new();
    let rec = r.r16(game::REC_BASE).wrapping_add(0x3A);
    r.w16(game::SEL_REC, rec);
    r.w16(game::BUTTONS, 2);
    game::dispatch_action(&mut r.ctx(), 0xB8F1);
    assert_eq!(r.r16(game::REC_CUR), rec);
    assert_eq!(r.r8(game::UIMODE), 0); // leave → shell
}

#[test]
fn gal_build_commits_the_new_unit() {
    let mut r = Rig::new();
    let t = 0x4000u16;
    r.w16(game::TYPE_REC, t);
    r.w8(usize::from(t) + 0x12, 7); // kind 7 — skips the kind-4 gates
    r.w16(usize::from(t) + 6, 0x64); // cost 100
    let f = r
        .r16(game::REC_BASE)
        .wrapping_add(r.r16(game::REC_COUNT).wrapping_mul(0x3A));
    for o in [0x18usize, 0x1C, 0x20] {
        r.w8(usize::from(f) + o, 0); // free bays
    }
    r.w16(usize::from(f) + 0x36, 0x3E8); // funds 1000
    r.w8(usize::from(t) + 0x22, 0xFF); // empty type name → terminator
    r.keys(&[0x1C]); // Enter accepts the prefill
    game::dispatch_action(&mut r.ctx(), 0xD17E);
    let m = u16::try_from(game::MACH_BASE).unwrap_or(0);
    assert_eq!(r.r16(game::REINF_REC), m);
    assert_eq!(r.r8(usize::from(m) + game::M_TYPE), 7);
    assert_eq!(r.r8(usize::from(f) + 0x18), 1); // bay = index + 1
    assert_eq!(r.r16(usize::from(f) + 0x36), 0x3E8 - 0x64);
    assert_eq!(r.r16(game::SEL_REC), f);
    assert!(!r.host.0.contains(&Call::Native(0xD17E)));
}

#[test]
fn gal_build_cancel_leaves_the_slot_free() {
    let mut r = Rig::new();
    let t = 0x4000u16;
    r.w16(game::TYPE_REC, t);
    r.w8(usize::from(t) + 0x12, 7);
    r.w16(usize::from(t) + 6, 0);
    let f = r
        .r16(game::REC_BASE)
        .wrapping_add(r.r16(game::REC_COUNT).wrapping_mul(0x3A));
    for o in [0x18usize, 0x1C, 0x20] {
        r.w8(usize::from(f) + o, 0);
    }
    r.w16(usize::from(f) + 0x36, 0x3E8);
    r.w8(usize::from(t) + 0x22, 0xFF);
    r.keys(&[0x1]); // Esc cancels the name edit
    game::dispatch_action(&mut r.ctx(), 0x642E);
    let m = u16::try_from(game::MACH_BASE).unwrap_or(0);
    assert_eq!(r.r8(usize::from(m) + game::M_TYPE), 0); // uncommitted
    assert_eq!(r.r16(usize::from(f) + 0x36), 0x3E8); // funds untouched
}
