//! The main-loop port — `frame_step` phase order, the `[0x91EE]`
//! refresh block, ambient sound select, and the `[0x9CDA]&2` exit edge.

mod common;

use common::rig::Rig;
use supremacy::game::{self, Call, Frame, Tick};

#[test]
fn frame_order_and_counter() {
    let mut r = Rig::new();
    let out = game::frame_step(&mut r.ctx());
    assert!(matches!(out, Frame::Run(Tick::Machine(_))));
    assert_eq!(r.r8(game::FRAME), 1);
    let h = &r.host.0;
    assert_eq!(h[0], Call::Ui(0x2CA9)); // vsync first
    assert!(h.contains(&Call::Native(0xA369))); // input phase
}

#[test]
fn exit_edge_returns_to_shell() {
    let mut r = Rig::new();
    r.w16(game::BUTTONS, 2);
    r.w8(0x91EA, 0xFF);
    r.w16(game::HOT_SEL, 7);
    assert_eq!(game::frame_step(&mut r.ctx()), Frame::Shell);
    assert_eq!(r.r8(0x91EA), 0);
    assert_eq!(r.r16(game::SEL_SAVE), 7);
    assert!(r.host.0.contains(&Call::Mouse(8, 0, 0xBC)));
}

#[test]
fn redraw_block_runs_select_and_present() {
    let mut r = Rig::new();
    r.w8(game::REDRAW, 1);
    r.w8(game::TYPE_SEL, 2);
    let t = game::TYPE_BASE + 2 * game::TYPE_STRIDE;
    r.w16(t, 0x1234); // type image index at +0
    r.w16(t + 0x18, 0x500); // seq list ptr
    r.w8(t + 0x1C, 9); // seq period
    game::frame_step(&mut r.ctx());
    assert_eq!(r.r16(game::TYPE_REC), u16::try_from(t).unwrap_or(0));
    assert_eq!(r.r16(game::SEQ_PTR), 0x500);
    assert_eq!(r.r8(game::SEQ_PERIOD), 9);
    let want = [
        Call::Slot(0x1260, 0),
        Call::Slot(0x125C, 0),
        Call::Mouse(8, 0x90, 0xBC),
        Call::Image(0x1234),
        Call::Image(0x56),
        Call::Present(5),
    ];
    assert!(want.iter().all(|w| r.host.0.contains(w)));
    assert_eq!(r.r8(game::REDRAW), 0);
}

#[test]
fn ambient_sound_selects_by_type_and_frame() {
    let mut r = Rig::new();
    r.w8(game::SND_ON, 1);
    let tb = game::TYPE_BASE;
    r.w16(game::TYPE_REC, u16::try_from(tb).unwrap_or(0));
    r.w8(tb + 0x12, 7); // type 7 → every 32 frames
    r.w8(game::FRAME, 0x20);
    game::frame_step(&mut r.ctx());
    assert!(r.host.0.contains(&Call::Sfx(0x26, 0x0F)));
}

#[test]
fn ambient_muted_when_flag_ff() {
    let mut r = Rig::new();
    r.w8(game::SND_ON, 0xFF);
    let tb = game::TYPE_BASE;
    r.w16(game::TYPE_REC, u16::try_from(tb).unwrap_or(0));
    r.w8(tb + 0x12, 7);
    game::frame_step(&mut r.ctx());
    assert!(!r.host.0.iter().any(|c| matches!(c, Call::Sfx(..))));
}

#[test]
fn ambient_type6_alternates_halves() {
    let mut r = Rig::new();
    r.w8(game::SND_ON, 1);
    let tb = game::TYPE_BASE;
    r.w16(game::TYPE_REC, u16::try_from(tb).unwrap_or(0));
    r.w8(tb + 0x12, 6);
    game::frame_step(&mut r.ctx()); // frame 0 → first pair
    assert!(r.host.0.contains(&Call::Sfx(0x2A, 0x0B)));
    r.w8(game::FRAME, 0x80);
    game::frame_step(&mut r.ctx()); // frame 0x80 → second pair
    assert!(r.host.0.contains(&Call::Sfx(0x21, 0x14)));
}

#[test]
fn dirty_flags_serve_one_bit_per_frame() {
    let mut r = Rig::new();
    r.w16(game::DIRTY0, 0x8004);
    r.w16(game::DIRTY1, 2);
    game::frame_step(&mut r.ctx()); // services bit 2 only
    assert_eq!(r.r16(game::DIRTY0), 0x8000);
    assert_eq!(r.r16(game::SNAP0), 0x8004);
    assert_eq!(r.r16(game::DIRTY1), 2);
    game::frame_step(&mut r.ctx()); // then bit 15
    assert_eq!(r.r16(game::DIRTY0), 0);
    game::frame_step(&mut r.ctx()); // then DIRTY1 bit 1
    assert_eq!(r.r16(game::DIRTY1), 0);
    for want in [
        Call::Native(0x3CAD),
        Call::Native(0x3CDB),
        Call::Native(0x3CF6),
    ] {
        assert!(r.host.0.contains(&want), "missing {want:?}");
    }
}
