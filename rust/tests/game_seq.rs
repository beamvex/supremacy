//! The `0x5C1E` draw sequencer — delay countdown, phase/period gating,
//! and the `{image | 0x1F4 delay | 0 link | 0xFFFF end}` record stream.

mod common;

use common::rig::Rig;
use supremacy::game::{self, Call};

#[test]
fn delay_and_draw() {
    let mut r = Rig::new();
    r.w8(game::SEQ_PERIOD, 1); // fire every frame
    r.w16(game::SEQ_PTR, 0x200);
    r.rec(0x200, 0x40); // draw image 0x40
    r.rec(0x204, 0xFFFF); // end
    game::seq_step(&mut r.ctx());
    assert_eq!(r.host.0, [Call::Image(0x40)]);
    assert_eq!(r.r16(game::SEQ_PTR), 0x204);
    game::seq_step(&mut r.ctx());
    assert_eq!(r.r16(game::SEQ_PTR), 0); // end clears cursor+aux
}

#[test]
fn delay_op_loads_from_next_record() {
    let mut r = Rig::new();
    r.w8(game::SEQ_PERIOD, 2); // fire every 2nd frame
    r.w16(game::SEQ_PTR, 0x200);
    r.rec(0x200, 0x01F4); // delay: the NEXT record's op field = count
    r.rec(0x204, 3);
    r.rec(0x208, 0x40);
    r.rec(0x20C, 0xFFFF);
    game::seq_step(&mut r.ctx()); // phase 1 — gated, nothing runs
    assert!(r.host.0.is_empty());
    game::seq_step(&mut r.ctx()); // phase wraps → delay op loads 3
    assert_eq!(r.r16(game::SEQ_DELAY), 3);
    assert_eq!(r.r16(game::SEQ_PTR), 0x208);
    for _ in 0..3 {
        game::seq_step(&mut r.ctx()); // delay counts down, phase held
    }
    assert_eq!(r.r16(game::SEQ_DELAY), 0);
    game::seq_step(&mut r.ctx()); // phase 1 — still gated at delay 0
    assert!(r.host.0.is_empty());
    game::seq_step(&mut r.ctx()); // phase 0 → draw
    assert_eq!(r.host.0, [Call::Image(0x40)]);
}

#[test]
fn link_chases_pointer() {
    let mut r = Rig::new();
    r.w8(game::SEQ_PERIOD, 1);
    r.w16(game::SEQ_PTR, 0x200);
    r.rec(0x200, 0); // link → si = next record's op word
    r.rec(0x204, 0x300);
    r.rec(0x300, 0x55);
    game::seq_step(&mut r.ctx());
    assert_eq!(r.host.0, [Call::Image(0x55)]);
    assert_eq!(r.r16(game::SEQ_PTR), 0x304);
}

#[test]
fn seq_cmd_skips_the_delay_and_period_gates() {
    // `0x5C49` runs one command straight through `[0x91A0]` and the
    // `[0x91D6]`/`[0x91D7]` gates — the `0x5C97` tail-call.
    let mut r = Rig::new();
    r.w8(game::SEQ_PERIOD, 0xFF);
    r.w16(game::SEQ_DELAY, 9);
    r.w16(game::SEQ_PTR, 0x200);
    r.rec(0x200, 0x40);
    r.rec(0x204, 0xFFFF);
    game::seq_cmd(&mut r.ctx());
    assert_eq!(r.host.0, [Call::Image(0x40)]);
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
    let tab = 0x698 + 0xE; // owner 1 × stride 0xE
    r.rec(tab, 0x200); // stream head → [0x9198]
    r.rec(tab + 0x4, 0x1111); // params → 0x9277/0x927D/0x927F/0x9281
    r.rec(tab + 0x6, 0x2222);
    r.rec(tab + 0x8, 0x3333);
    r.rec(tab + 0xA, 0x4444);
    r.rec(tab + 0xC, 7); // period byte → [0x91D6]
    r.rec(0x200, 0xFFFF); // the armed stream — ends at once
    game::planet_panel(&mut r.ctx());
    assert!(r.host.0.contains(&Call::Rect(0x18, 0x13, 0x3, 0x33)));
    assert!(r.host.0.contains(&Call::Refresh));
    assert_eq!(r.r8(game::SEQ_PERIOD), 7);
    assert_eq!(r.r16(0x9277), 0x1111);
    assert_eq!(r.r16(0x9281), 0x4444);
    assert_eq!(r.r16(game::SEQ_PTR), 0); // the 0x5C49 ran the end op
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
    r.w16(game::SEQ_PTR, 0x200);
    r.rec(0x200, 0x40);
    game::seq_step(&mut r.ctx());
    assert!(r.host.0.is_empty()); // disabled — cursor untouched
    assert_eq!(r.r16(game::SEQ_PTR), 0x200);
    r.w8(game::SEQ_PERIOD, 1);
    r.w16(game::SEQ_PTR, 0);
    game::seq_step(&mut r.ctx()); // null stream — phase still advances
    assert_eq!(r.r8(game::SEQ_PHASE), 0);
    assert!(r.host.0.is_empty());
}
