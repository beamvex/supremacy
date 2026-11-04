//! The `0x5C1E` draw sequencer — delay countdown, phase/period gating,
//! and the compiled `seqtab` command stream (GAM-80: the `cs:` records
//! are `SeqOp` tables, so tests run the shipped stream addresses).

mod common;

use common::rig::Rig;
use supremacy::game::{self, Call};

#[test]
fn image_draws_and_advances_the_cursor() {
    // `0x360` is a real stream head: `Img(0xB8)` then the next record.
    let mut r = Rig::new();
    r.w8(game::SEQ_PERIOD, 1); // fire every frame
    r.w16(game::SEQ_PTR, 0x360);
    game::seq_step(&mut r.ctx());
    assert_eq!(r.host.0, [Call::Image(0xB8)]);
    assert_eq!(r.r16(game::SEQ_PTR), 0x364);
}

#[test]
fn end_record_clears_the_cursor() {
    // `0xA06` ships `End` — the `0xFFFF` record.
    let mut r = Rig::new();
    r.w8(game::SEQ_PERIOD, 1);
    r.w16(game::SEQ_PTR, 0xA06);
    game::seq_step(&mut r.ctx());
    assert_eq!(r.r16(game::SEQ_PTR), 0);
    assert!(r.host.0.is_empty()); // end draws nothing
}

#[test]
fn delay_op_reloads_the_countdown() {
    // `0x9F6` ships `Delay(0x12C)`: period 2 so the gate is visible.
    let mut r = Rig::new();
    r.w8(game::SEQ_PERIOD, 2); // fire every 2nd frame
    r.w16(game::SEQ_PTR, 0x9F6);
    game::seq_step(&mut r.ctx()); // phase 1 — gated, nothing runs
    assert!(r.host.0.is_empty());
    game::seq_step(&mut r.ctx()); // phase wraps → delay op loads
    assert_eq!(r.r16(game::SEQ_DELAY), 0x12C);
    assert_eq!(r.r16(game::SEQ_PTR), 0x9FE);
    for _ in 0..0x12C {
        game::seq_step(&mut r.ctx()); // delay counts down, phase held
    }
    assert_eq!(r.r16(game::SEQ_DELAY), 0);
    game::seq_step(&mut r.ctx()); // phase 1 — still gated
    assert!(r.host.0.is_empty());
    game::seq_step(&mut r.ctx()); // phase 0 → `Delay(0)` at 0x9FE
    assert_eq!(r.r16(game::SEQ_PTR), 0xA06);
    game::seq_step(&mut r.ctx()); // gated
    game::seq_step(&mut r.ctx()); // phase 0 → `End` at 0xA06
    assert_eq!(r.r16(game::SEQ_PTR), 0);
}

#[test]
fn link_record_chases_the_pointer() {
    // `0x41C` ships `Go(0x3CC)` → draws `Img(0x41)` from there.
    let mut r = Rig::new();
    r.w8(game::SEQ_PERIOD, 1);
    r.w16(game::SEQ_PTR, 0x41C);
    game::seq_step(&mut r.ctx());
    assert_eq!(r.host.0, [Call::Image(0x41)]);
    assert_eq!(r.r16(game::SEQ_PTR), 0x3D0);
}

#[test]
fn seq_cmd_skips_the_delay_and_period_gates() {
    // `0x5C49` runs one command straight through `[0x91A0]` and the
    // `[0x91D6]`/`[0x91D7]` gates — the `0x5C97` tail-call.
    let mut r = Rig::new();
    r.w8(game::SEQ_PERIOD, 0xFF);
    r.w16(game::SEQ_DELAY, 9);
    r.w16(game::SEQ_PTR, 0x360);
    game::seq_cmd(&mut r.ctx());
    assert_eq!(r.host.0, [Call::Image(0xB8)]);
    assert_eq!(r.r16(game::SEQ_DELAY), 9); // countdown untouched
}

/// `cs:0x5C97` — the `uimode == 0` planet panel arms the selected
/// record's `cs:` stream and runs one command immediately.
#[test]
fn planet_panel_arms_the_viewscreen_stream() {
    let mut r = Rig::new();
    let rec = r.r16(game::REC_BASE);
    r.w16(game::REC_CUR, rec);
    r.w16(usize::from(rec) + game::F_KIND, 2);
    r.w8(usize::from(rec) + game::F_OWNER, 1);
    let tab = 0x698 + 0xE; // owner 1 slot — `cs_word` serves the fake image
    r.rec(tab, 0x4D4); // compiled stream head → `Img 0x5B`
    game::planet_panel(&mut r.ctx());
    assert!(r.host.0.contains(&Call::Rect(0x18, 0x13, 0x3, 0x33)));
    assert!(r.host.0.contains(&Call::Refresh));
    // Owner-1's shipped stream head is `0x4D4` (`Img 0x5B` → `0x4D8`).
    assert!(r.host.0.contains(&Call::Image(0x5B)));
    assert_eq!(r.r16(game::SEQ_PTR), 0x4D8);
}

#[test]
fn planet_panel_kind_seven_prints_classified() {
    let mut r = Rig::new();
    let rec = r.r16(game::REC_BASE);
    r.w16(game::REC_CUR, rec);
    r.w16(usize::from(rec) + game::F_KIND, 7);
    game::planet_panel(&mut r.ctx());
    assert!(!r.host.0.contains(&Call::Refresh)); // never armed
    assert_eq!(r.r16(game::SEQ_PTR), 0);
}

#[test]
fn period_ff_disables_and_zero_ptr_idles() {
    let mut r = Rig::new();
    r.w8(game::SEQ_PERIOD, 0xFF);
    r.w16(game::SEQ_PTR, 0x360);
    game::seq_step(&mut r.ctx());
    assert!(r.host.0.is_empty()); // disabled — cursor untouched
    assert_eq!(r.r16(game::SEQ_PTR), 0x360);
    r.w8(game::SEQ_PERIOD, 1);
    r.w16(game::SEQ_PTR, 0);
    game::seq_step(&mut r.ctx()); // null stream — phase still advances
    assert_eq!(r.r8(game::SEQ_PHASE), 0);
    assert!(r.host.0.is_empty());
}

#[test]
fn off_stream_cursor_ends_cleanly() {
    // A cursor that is not a compiled position (stale pointer) ends
    // like the `0xFFFF` record — the cursor and aux clear.
    let mut r = Rig::new();
    r.w8(game::SEQ_PERIOD, 1);
    r.w16(game::SEQ_PTR, 0x7777); // not a compiled stream position
    game::seq_step(&mut r.ctx());
    assert_eq!(r.r16(game::SEQ_PTR), 0);
    assert_eq!(r.r16(game::SEQ_AUX), 0);
}
