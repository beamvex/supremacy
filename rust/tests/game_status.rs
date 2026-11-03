//! The uimode-6 status/docking-bay screen — the `0x595C` bay-name
//! walk over `[0x917C]`'s `+0x18`/`+0x1C`/`+0x20` bytes (GAM-75: a
//! unit bought on the galaxy screen lands in the faction record's
//! first free bay and has to print in the A/B/C slots).

mod common;

use common::rig::Rig;
use supremacy::game::{self, Call};

/// The `0x6B2E` empty-slot marker (`Vm::low` in a `Rig` is blank).
const S_EMPTY: u16 = 0x6B2E;

/// Selected record, colonised, bay A docked, exit edge armed.
fn rig() -> Rig {
    let mut r = Rig::new();
    for (i, &ch) in b"BAY EMPTY\xFF".iter().enumerate() {
        r.w8(usize::from(S_EMPTY) + i, ch);
    }
    let rec = r.r16(game::REC_BASE);
    r.w16(game::SEL_REC, rec);
    r.w16(usize::from(rec) + 0xC, 0xA);
    r.w8(usize::from(rec) + 0x18, 1); // bay A → machine 0
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
fn status_bays_print_docked_machine_name() {
    let mut r = rig();
    let m = u16::try_from(game::MACH_BASE).unwrap_or(0);
    for (i, &ch) in b"BATTLE..".iter().enumerate() {
        r.w8(usize::from(m) + i, ch);
    }
    r.w8(usize::from(m) + 8, 0xFF);
    game::dispatch_action(&mut r.ctx(), 0xC5B7);
    assert!(row_has(&r, b"BATTLE..", 5, 0x8F));
    // Bays B and C are empty — the `0x6B2E` marker prints on the
    // 0x9A/0xA5 rows instead.
    assert!(row_has(&r, b"BAY EMPTY", 5, 0x9A));
    assert!(row_has(&r, b"BAY EMPTY", 5, 0xA5));
}

#[test]
fn status_empty_bays_print_marker() {
    let mut r = rig();
    let rec = r.r16(game::REC_BASE);
    r.w8(usize::from(rec) + 0x18, 0);
    game::dispatch_action(&mut r.ctx(), 0xC5B7);
    for dx in [0x8F, 0x9A, 0xA5] {
        assert!(row_has(&r, b"BAY EMPTY", 5, dx), "marker on {dx:#x}");
    }
    assert!(!r.host.0.contains(&Call::Native(0x595C)));
}
