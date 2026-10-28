//! The uimode-2 detail screen's field fills — the `0x2493` body's
//! initial prints (`0x4C8B`/`0x5994`/`0x55F2` + the four STORES
//! columns), the `0x29D9` machine header via the single-bay
//! auto-select, the dirty-word reprint dispatch and the `0x2759`
//! meter sweep.

mod common;

use common::rig::Rig;
use supremacy::game::{self, Call};

/// Record 0 colonised and selected; the frame loop's exit edge armed.
fn rig() -> Rig {
    let mut r = Rig::new();
    let rec = r.r16(game::REC_BASE);
    r.w16(game::REC_CUR, rec);
    r.w16(usize::from(rec) + 0xC, 0xA);
    r.w16(game::BUTTONS, 2);
    r
}

/// `put` a word field on record `rec`.
fn field(r: &mut Rig, rec: u16, f: usize, v: u16) {
    r.w16(usize::from(rec) + f, v);
}

/// `true` when a glyph landed anywhere on text row `dx`.
fn on_row(r: &Rig, dx: u16) -> bool {
    r.host
        .0
        .iter()
        .any(|c| matches!(c, Call::Glyph(_, _, d) if *d == dx))
}

#[test]
fn detail_paints_name_and_stores() {
    let mut r = rig();
    let rec = r.r16(game::REC_BASE);
    for (i, ch) in b"KESTREL".iter().enumerate() {
        r.w8(usize::from(rec) + 0xE + i, *ch);
    }
    r.w8(usize::from(rec) + 0xE + 7, 0xFF);
    for f in [0x18usize, 0x1C, 0x20] {
        r.w8(usize::from(rec) + f, 0); // empty bays
    }
    field(&mut r, rec, 0x2C, 1234); // food
    field(&mut r, rec, 0x30, 56); // minerals
    field(&mut r, rec, 0x32, 7); // fuel
    field(&mut r, rec, 0x34, 890); // energy
    game::dispatch_action(&mut r.ctx(), 0x9179);
    for (i, &ch) in b"KESTREL".iter().enumerate() {
        let bx = u16::try_from(3 + i).unwrap_or(0);
        assert!(r.host.0.contains(&Call::Glyph(ch, bx, 0xE)), "name {ch}");
    }
    let want = [
        (b' ', 0x30, 0xB4),
        (b'1', 0x31, 0xB4),
        (b'4', 0x34, 0xB4), // " 1234" food
        (b'5', 0x3B, 0xB4),
        (b'6', 0x3C, 0xB4), // "   56" minerals
        (b'7', 0x44, 0xB4), // "    7" fuel
        (b'8', 0x4A, 0xB4),
        (b'0', 0x4C, 0xB4), // "  890" energy
    ];
    for (ch, bx, dx) in want {
        assert!(
            r.host.0.contains(&Call::Glyph(ch, bx, dx)),
            "{ch} @{bx},{dx}"
        );
    }
    assert!(!on_row(&r, 0xBE)); // no machine row without a docked unit
    assert!(r.host.0.contains(&Call::Bar(0x18, 4, 4, 1))); // food 1234/0xFA
    assert!(r.host.0.contains(&Call::Bar(0x24, 4, 3, 1))); // energy 890/0xFA
    assert!(!r
        .host
        .0
        .iter()
        .any(|c| matches!(c, Call::Bar(_, 0x10, _, _))));
}

#[test]
fn detail_single_bay_autoselects_the_machine() {
    let mut r = rig();
    let rec = r.r16(game::REC_BASE);
    let m = u16::try_from(game::MACH_BASE).unwrap_or(0);
    r.w8(usize::from(rec) + 0x18, 1); // bay A → machine 0
    r.w8(usize::from(rec) + 0x1C, 0);
    r.w8(usize::from(rec) + 0x20, 0);
    r.w8(usize::from(m) + game::M_TYPE, 3);
    let t = u16::try_from(game::TYPE_BASE + 2 * game::TYPE_STRIDE).unwrap_or(0);
    field(&mut r, t, 4, 7); // type stat — 0x686C 5-column print
    field(&mut r, rec, 0x2A, 300); // population
    field(&mut r, m, 0x24, 42); // crew
    field(&mut r, m, 0xC, 5); // kind word
    field(&mut r, m, 0xA, 10);
    field(&mut r, m, 0xE, 20);
    field(&mut r, m, 0x10, 30);
    field(&mut r, m, 0x12, 40);
    game::dispatch_action(&mut r.ctx(), 0x2429);
    assert_eq!(r.r16(game::SEL_MACH), m);
    assert_eq!(r.r8(0x91DD), 0xFF); // ANIM armed by 0x29D9
    let want = [
        (b'3', 0x18, 0x10),
        (b'0', 0x19, 0x10), // "  300" population
        (b'4', 0x13, 0x7A),
        (b'2', 0x14, 0x7A), // "  42" crew
        (b'5', 0x1A, 0x40), // "    5" kind word
        (b' ', 0x16, 0x28),
        (b'7', 0x1A, 0x28), // "    7" type +4 stat — 5 columns wide
        (b'1', 0x33, 0xBE),
        (b'0', 0x34, 0xBE), // "   10" machine food row
        (b'1', 0x18, 0x38),
        (b'0', 0x1A, 0x38), // "  100" load total
    ];
    for (ch, bx, dx) in want {
        assert!(
            r.host.0.contains(&Call::Glyph(ch, bx, dx)),
            "{ch} @{bx},{dx}"
        );
    }
    assert!(r.host.0.contains(&Call::Bar(0x21, 0x10, 1, 1))); // load 30/0x19
    assert!(r.host.0.contains(&Call::Rect(0x24, 0xC, 0, 0x92))); // strip clear
}

#[test]
fn detail_multiple_bays_skip_the_autoselect() {
    let mut r = rig();
    let rec = r.r16(game::REC_BASE);
    r.w8(usize::from(rec) + 0x18, 1);
    r.w8(usize::from(rec) + 0x1C, 2); // two docked — no auto-select
    r.w8(usize::from(rec) + 0x20, 0);
    game::dispatch_action(&mut r.ctx(), 0x9179);
    assert_eq!(r.r16(game::SEL_MACH), 0);
    assert_eq!(r.r8(0x91DD), 0);
    assert!(!on_row(&r, 0xBE));
}

/// `rec` with bays `[1,2,0]` (machines 0+1 docked — no auto-select)
/// and machine 1 typed `3`, returned `(rec, m1)`.
fn two_bays(r: &mut Rig) -> (u16, u16) {
    let rec = r.r16(game::REC_BASE);
    let m1 = u16::try_from(game::MACH_BASE + game::MACH_STRIDE).unwrap_or(0);
    r.w8(usize::from(rec) + 0x18, 1);
    r.w8(usize::from(rec) + 0x1C, 2);
    r.w8(usize::from(rec) + 0x20, 0);
    r.w8(usize::from(m1) + game::M_TYPE, 3);
    (rec, m1)
}

#[test]
fn bay_hotspot_selects_the_docked_machine() {
    let mut r = rig();
    let (_, m1) = two_bays(&mut r);
    game::dispatch_action(&mut r.ctx(), 0x2429);
    assert_eq!(r.r16(game::SEL_MACH), 0);
    game::dispatch_action(&mut r.ctx(), 0x2ADD); // mach_sel 2
    assert_eq!(r.r16(game::SEL_MACH), m1);
    assert_eq!(r.r8(0x91DD), 0xFF); // ANIM armed by the 0x29D9 header
    assert_eq!(r.r16(game::HOT_COUNT), 0x17); // the full 23-record list
}

#[test]
fn bay_hotspot_on_an_empty_bay_only_messages() {
    let mut r = rig();
    let _ = two_bays(&mut r);
    game::dispatch_action(&mut r.ctx(), 0x2429);
    r.host.0.clear();
    game::dispatch_action(&mut r.ctx(), 0x2AEA); // mach_sel 3 — empty
    assert_eq!(r.r16(game::SEL_MACH), 0);
    assert!(r.host.0.contains(&Call::Native(0xA369))); // the 0x6F2D wait ran
    assert!(!r.host.0.iter().any(|c| matches!(c, Call::Sound(_))));
}

/// Single-bay rig entered (auto-select on machine 0, type `3`), its
/// type record `t` returned alongside `(rec, m)`.
fn one_bay(r: &mut Rig) -> (u16, u16, u16) {
    let rec = r.r16(game::REC_BASE);
    let m = u16::try_from(game::MACH_BASE).unwrap_or(0);
    let t = u16::try_from(game::TYPE_BASE + 2 * game::TYPE_STRIDE).unwrap_or(0);
    r.w8(usize::from(rec) + 0x18, 1);
    r.w8(usize::from(m) + game::M_TYPE, 3);
    game::dispatch_action(&mut r.ctx(), 0x2429);
    (rec, m, t)
}

#[test]
fn xfer_in_moves_up_to_five_within_capacity() {
    let mut r = rig();
    let (rec, m, t) = one_bay(&mut r);
    field(&mut r, t, 4, 20); // stores capacity
    field(&mut r, rec, 0x2C, 8); // planet food
    game::dispatch_action(&mut r.ctx(), 0x265F); // xfer_in A
    assert_eq!(r.r16(usize::from(rec) + 0x2C), 3);
    assert_eq!(r.r16(usize::from(m) + 0xA), 5);
    assert_eq!(r.r8(0x91CA), 0); // re-armed for a held press
}

#[test]
fn xfer_in_a_full_machine_moves_nothing() {
    let mut r = rig();
    let (rec, m, t) = one_bay(&mut r);
    field(&mut r, t, 4, 5);
    field(&mut r, m, 0xA, 5); // load already at capacity
    field(&mut r, rec, 0x2C, 100);
    game::dispatch_action(&mut r.ctx(), 0x265F);
    assert_eq!(r.r16(usize::from(rec) + 0x2C), 100);
    assert_eq!(r.r16(usize::from(m) + 0xA), 5);
    assert_eq!(r.r8(0x91E1), 0xFF); // the 0x262C full flag
}

#[test]
fn xfer_out_moves_five_and_honours_the_planet_cap() {
    let mut r = rig();
    let (rec, m, _t) = one_bay(&mut r);
    field(&mut r, m, 0xA, 12);
    field(&mut r, rec, 0x2C, 100);
    game::dispatch_action(&mut r.ctx(), 0x2AF7); // xfer_out A
    assert_eq!(r.r16(usize::from(m) + 0xA), 7);
    assert_eq!(r.r16(usize::from(rec) + 0x2C), 105);
    field(&mut r, rec, 0x2C, 0x7530);
    game::dispatch_action(&mut r.ctx(), 0x2AF7); // planet full — skip
    assert_eq!(r.r16(usize::from(m) + 0xA), 7);
    assert_eq!(r.r16(usize::from(rec) + 0x2C), 0x7530);
}

#[test]
fn people_transfer_moves_five_each_way() {
    let mut r = rig();
    let (rec, m, t) = one_bay(&mut r);
    field(&mut r, t, 0x1E, 50); // crew capacity
    field(&mut r, rec, 0x2A, 100);
    game::dispatch_action(&mut r.ctx(), 0x4C9B); // in
    assert_eq!(r.r16(usize::from(m) + 0x24), 5);
    assert_eq!(r.r16(usize::from(rec) + 0x2A), 95);
    game::dispatch_action(&mut r.ctx(), 0x4CD8); // out
    assert_eq!(r.r16(usize::from(m) + 0x24), 0);
    assert_eq!(r.r16(usize::from(rec) + 0x2A), 100);
}

#[test]
fn fuel_slider_loads_a_step_and_ramps_the_repeat() {
    let mut r = rig();
    let (rec, m, t) = one_bay(&mut r);
    field(&mut r, t, 0xC, 100); // fuel capacity
    field(&mut r, m, 0x26, 10);
    field(&mut r, rec, 0x32, 40);
    r.w8(game::FRAME, 5);
    game::dispatch_action(&mut r.ctx(), 0x4D12); // planet → machine
    assert_eq!(r.r16(usize::from(m) + 0x26), 11); // step 0xC − 0xB
    assert_eq!(r.r16(usize::from(rec) + 0x32), 39);
    assert_eq!(r.r8(0x91F6), 0xB); // ramped reload armed
    r.w8(game::FRAME, 6);
    game::dispatch_action(&mut r.ctx(), 0x4D12); // held — delay eats it
    assert_eq!(r.r16(usize::from(m) + 0x26), 11);
    assert_eq!(r.r8(0x91F7), 0xA);
    r.w8(game::FRAME, 8);
    game::dispatch_action(&mut r.ctx(), 0x4D12); // gap — fresh press
    assert_eq!(r.r16(usize::from(m) + 0x26), 12);
    r.w8(game::FRAME, 11);
    game::dispatch_action(&mut r.ctx(), 0x4D0B); // machine → planet
    assert_eq!(r.r16(usize::from(m) + 0x26), 11);
    assert_eq!(r.r16(usize::from(rec) + 0x32), 39);
}

#[test]
fn xfer_all_loads_the_type_payload_off_the_population() {
    let mut r = rig();
    let (rec, m, t) = one_bay(&mut r);
    field(&mut r, t, 0xA, 30);
    field(&mut r, rec, 0x2A, 100);
    game::dispatch_action(&mut r.ctx(), 0x4DBB);
    assert_eq!(r.r16(usize::from(m) + 0x14), 30);
    assert_eq!(r.r16(usize::from(rec) + 0x2A), 70);
}

#[test]
fn xfer_all_refuses_a_live_payload_and_a_shortfall() {
    let mut r = rig();
    let (rec, m, t) = one_bay(&mut r);
    field(&mut r, t, 0xA, 30);
    field(&mut r, m, 0x14, 7); // already loaded → 0x6A25
    field(&mut r, rec, 0x2A, 100);
    game::dispatch_action(&mut r.ctx(), 0x4DBB);
    assert_eq!(r.r16(usize::from(m) + 0x14), 7);
    assert_eq!(r.r16(usize::from(rec) + 0x2A), 100);
    field(&mut r, m, 0x14, 0);
    field(&mut r, rec, 0x2A, 10); // borrow → 0x6A51, pop restored
    game::dispatch_action(&mut r.ctx(), 0x4DBB);
    assert_eq!(r.r16(usize::from(m) + 0x14), 0);
    assert_eq!(r.r16(usize::from(rec) + 0x2A), 10);
}

#[test]
fn unload_all_flushes_every_store_to_the_planet() {
    let mut r = rig();
    let (rec, m, _t) = one_bay(&mut r);
    field(&mut r, m, 0xA, 12);
    field(&mut r, m, 0xE, 3);
    field(&mut r, m, 0x12, 200);
    field(&mut r, m, 0x24, 7); // crew
    game::dispatch_action(&mut r.ctx(), 0x4E0A);
    for f in [0xAusize, 0xE, 0x12, 0x24] {
        assert_eq!(r.r16(usize::from(m) + f), 0, "field {f}");
    }
    assert_eq!(r.r16(usize::from(rec) + 0x2C), 12);
    assert_eq!(r.r16(usize::from(rec) + 0x30), 3);
    assert_eq!(r.r16(usize::from(rec) + 0x34), 200);
    assert_eq!(r.r16(usize::from(rec) + 0x2A), 7);
}

#[test]
fn scuttle_on_y_relinks_refunds_and_clears_the_bay() {
    let mut r = rig();
    let (rec, m, t) = one_bay(&mut r);
    r.w8(usize::from(m) + game::M_HOST, 0);
    field(&mut r, m, 0xC, 9); // refund word
    field(&mut r, m, 0x26, 40); // fuel salvage
    field(&mut r, t, 0xE, 7); // energy salvage
    field(&mut r, t, 0x10, 11); // minerals salvage
    let ship = u16::try_from(game::FLEET_BASE).unwrap_or(0);
    field(&mut r, ship, 2, m); // fleet slot owns the machine
    r.w8(usize::from(ship) + 6, 0xFF);
    r.keys(&[0x15]); // 'Y' make + break
    game::dispatch_action(&mut r.ctx(), 0x2B9B);
    assert_eq!(r.r16(usize::from(ship) + 2), rec); // relinked to host
    assert_eq!(r.r8(usize::from(ship) + 6), 0xFD);
    assert_eq!(r.r16(usize::from(rec) + 0x36), 9);
    assert_eq!(r.r16(usize::from(rec) + 0x32), 40);
    assert_eq!(r.r16(usize::from(rec) + 0x34), 7);
    assert_eq!(r.r16(usize::from(rec) + 0x30), 11);
    assert_eq!(r.r8(usize::from(rec) + 0x18), 0); // bay byte cleared
    assert_eq!(r.r8(usize::from(m) + game::M_TYPE), 0); // machine freed
    assert_eq!(r.r16(game::SEL_MACH), 0);
}

#[test]
fn scuttle_on_n_aborts_without_touching_the_machine() {
    let mut r = rig();
    let (rec, m, _t) = one_bay(&mut r);
    r.keys(&[0x31]); // 'N' make + break
    game::dispatch_action(&mut r.ctx(), 0x2B9B);
    assert_eq!(r.r8(usize::from(m) + game::M_TYPE), 3);
    assert_eq!(r.r8(usize::from(rec) + 0x18), 1);
    assert_eq!(r.r16(game::SEL_MACH), m);
}

#[test]
fn dirty_bits_reprint_one_field_per_frame() {
    let mut r = rig();
    let rec = r.r16(game::REC_BASE);
    field(&mut r, rec, 0x2C, 42); // food changed under the screen
    r.w16(game::DIRTY0, 0x4000);
    r.w16(game::DIRTY1, 2); // lower priority — stays armed this frame
    game::dispatch_action(&mut r.ctx(), 0x9179);
    assert_eq!(r.r16(game::DIRTY0) & 0x4000, 0); // serviced
    assert_eq!(r.r16(game::SNAP0) & 0x4000, 0x4000); // snapshot kept it
    assert_eq!(r.r16(game::DIRTY1) & 2, 2); // one bit per frame
    let want = [(b' ', 0x30, 0xB4), (b'4', 0x33, 0xB4), (b'2', 0x34, 0xB4)];
    for (ch, bx, dx) in want {
        assert!(
            r.host.0.contains(&Call::Glyph(ch, bx, dx)),
            "{ch} @{bx},{dx}"
        );
    }
}
