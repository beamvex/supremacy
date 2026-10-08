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
