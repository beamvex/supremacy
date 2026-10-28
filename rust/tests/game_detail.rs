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
