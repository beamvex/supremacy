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

// ---------------------------------------------------------------------------
// The `0x9D60` list's bay/cell actions (GAM-76, round 3) — the
// "DOCKING BAY TO PLANET SURFACE TRANSFERENCE SYSTEM" rows: `bay_pick`
// moves a docked unit into the held cells, `cell_load` docks it back,
// `cell_sel` marks it RUNNING — all through the real `Game` + `Pump`
// input path.

use common::game_dir;
use supremacy::args::{Sound, Video};
use supremacy::assets::{heap_segment, load_set};
use supremacy::front::{Game, Pump};
use supremacy::palette::{from_dac, DAC_OFS};

/// `ds:` base of the six held-unit cells `0x59EB` fills.
const CELLS: u16 = 0x5017;

fn game() -> Game {
    let img = supremacy::img::image();
    Game::new(
        img.clone(),
        load_set(&game_dir(), "MCG", heap_segment()).unwrap(),
        from_dac(&img[DAC_OFS..DAC_OFS + 768]),
        Video::Mcga,
        Sound::PcSpeaker,
        false,
        3,
        0x42,
        Box::new(supremacy::platform::MemFs::new()),
    )
}

/// Colonised record, `SEL_REC`/`REC_CUR` on it, bay A docked with
/// machine 0 hosted on the record's serial — returns `(rec, m)`.
fn docked(g: &mut Game) -> (u16, u16) {
    let rec = g.st.word(game::REC_BASE);
    g.st.set_word(usize::from(rec) + 0xC, 0xA);
    g.st.set_byte(usize::from(rec) + 0x18, 1);
    g.st.set_byte(usize::from(rec) + 0x1C, 0);
    g.st.set_byte(usize::from(rec) + 0x20, 0);
    g.st.set_word(game::REC_CUR, rec);
    g.st.set_word(game::SEL_REC, rec);
    let m = u16::try_from(game::MACH_BASE).unwrap_or(0);
    for (i, &ch) in b"MINER...".iter().enumerate() {
        g.st.set_byte(usize::from(m) + i, ch);
    }
    g.st.set_byte(usize::from(m) + 8, 0xFF);
    g.st.set_byte(usize::from(m) + game::M_TYPE, 6);
    g.st.set_byte(usize::from(m) + game::M_FLAGS, 0);
    g.st.set_byte(
        usize::from(m) + game::M_HOST,
        g.st.byte(usize::from(rec) + game::F_SERIAL),
    );
    (rec, m)
}

/// The `cs:0x5867` entry with `steps` scripted through the vsync
/// hook — same `Pump` contract `game_surface` drives.
fn run(g: &mut Game, steps: impl FnMut(u16, &mut Pump) + 'static) {
    let mut n = 0u16;
    let mut steps = steps;
    g.host.set_vsync(Some(Box::new(move |h| {
        n += 1;
        steps(n, &mut h.pump);
    })));
    game::dispatch_action(&mut g.ctx(), 0xC5B7);
    g.host.set_vsync(None);
}

/// Click `rect` at `n`, release two beats later, right-exit at the
/// tail — the debounced `menu::service` press/release shape.
fn click(p: &mut Pump, x: u16, y: u16) {
    p.move_to(x, y);
    p.set_buttons(true, false);
}

/// Bay A's `(5,140)`–`(55,150)` row → the unit leaves the bay into
/// held cell 0: `+0x1F | 0x10`, `ds:0x5017` filled, `0x91DD` armed.
#[test]
fn status_bay_pick_moves_unit_to_cells() {
    let mut g = game();
    let (rec, m) = docked(&mut g);
    run(&mut g, |n, p| match n {
        5 => click(p, 0x20, 0x92),
        9 => p.set_buttons(false, false),
        13 => p.set_buttons(false, true),
        17 => p.set_buttons(false, false),
        _ => {}
    });
    assert_eq!(g.st.byte(usize::from(rec) + 0x18), 0, "bay A cleared");
    assert_eq!(g.st.byte(usize::from(m) + game::M_FLAGS) & 0x10, 0x10);
    assert_eq!(g.st.byte(0x91DD), 0xFF, "the redraw latch arms");
    assert_eq!(g.vm.r16(&g.st, CELLS), m, "held cell 0");
}

/// A held unit at entry packs into cell 0; the `(73,117)`–`(110,150)`
/// `cell_load` row re-docks it into the first free bay.
#[test]
fn status_cell_load_redocks_held_unit() {
    let mut g = game();
    let (rec, m) = docked(&mut g);
    g.st.set_byte(usize::from(rec) + 0x18, 0);
    g.st.set_byte(usize::from(m) + game::M_FLAGS, 0x10);
    run(&mut g, |n, p| match n {
        5 => click(p, 0x5A, 0x85),
        9 => p.set_buttons(false, false),
        13 => p.set_buttons(false, true),
        17 => p.set_buttons(false, false),
        _ => {}
    });
    assert_eq!(g.st.byte(usize::from(rec) + 0x18), 1, "bay A re-docked");
    assert_eq!(g.st.byte(usize::from(m) + game::M_FLAGS) & 0x30, 0);
    assert_eq!(g.vm.r16(&g.st, CELLS), 0, "held cell emptied");
}

/// The `(75,106)`–`(110,114)` `cell_sel` row toggles `+0x1F` `0x20`
/// — the RUNNING mark — on a crewed kind-`6` unit.
#[test]
fn status_cell_sel_marks_running() {
    let mut g = game();
    let (rec, m) = docked(&mut g);
    g.st.set_byte(usize::from(rec) + 0x18, 0);
    g.st.set_byte(usize::from(m) + game::M_FLAGS, 0x10);
    g.st.set_word(usize::from(m) + 0x14, 5);
    run(&mut g, |n, p| match n {
        5 => click(p, 0x5A, 0x6E),
        9 => p.set_buttons(false, false),
        13 => p.set_buttons(false, true),
        17 => p.set_buttons(false, false),
        _ => {}
    });
    assert_eq!(g.st.byte(usize::from(m) + game::M_FLAGS) & 0x20, 0x20);
    assert_eq!(g.vm.r16(&g.st, CELLS), m, "still held");
}

/// Empty bay B's pick takes the `0x5B3C` refuse — no flags, no cell.
#[test]
fn status_bay_pick_empty_refuses() {
    let mut g = game();
    let (rec, m) = docked(&mut g);
    run(&mut g, |n, p| match n {
        5 => click(p, 0x20, 0xA0),
        9 => p.set_buttons(false, false),
        13 => p.set_buttons(false, true),
        17 => p.set_buttons(false, false),
        _ => {}
    });
    assert_eq!(g.st.byte(usize::from(rec) + 0x18), 1, "bay A untouched");
    assert_eq!(g.st.byte(usize::from(m) + game::M_FLAGS) & 0x10, 0);
    assert_eq!(g.vm.r16(&g.st, CELLS), 0);
}

/// The held cells' `[0x128C]` icon puts carry the patched draw
/// position — `si = 0x78`, `ah = bl/2 − 5` off the post-name `bl` —
/// so the `0x15C`-based slot icons land beside their rows instead of
/// piling onto the records' shipped `(0,0)` origins (GAM-78).
#[test]
fn status_cells_icons_carry_positions() {
    let mut r = Rig::new();
    r.vm = game::Vm::from_image(&supremacy::img::image());
    let rec = r.r16(game::REC_BASE);
    r.w16(game::REC_CUR, rec);
    r.w16(game::SEL_REC, rec);
    r.w16(usize::from(rec) + 0xC, 0xA);
    let serial = r.r8(usize::from(rec) + 0x28);
    for (i, ty) in [(0usize, 2u8), (1, 5)] {
        let m = game::MACH_BASE + i * game::MACH_STRIDE;
        r.w8(m + game::M_TYPE, ty);
        r.w8(m + game::M_FLAGS, 0x10);
        r.w8(m + game::M_HOST, serial);
        for (k, &ch) in b"UNITX...".iter().enumerate() {
            r.w8(m + k, ch);
        }
    }
    r.w16(game::BUTTONS, 2);
    game::dispatch_action(&mut r.ctx(), 0xC5B7);
    let icons: Vec<Call> = r
        .host
        .0
        .iter()
        .copied()
        .filter(|c| matches!(c, Call::Icon(..)))
        .collect();
    assert_eq!(icons.len(), 6, "two held cells + four pads");
    assert!(icons.iter().all(|c| matches!(c, Call::Icon(_, _, 0x78))));
    // "UNITX..." prints 8 cols → `bl = 0x1B`, `ah = 0x1B/2 − 5 = 8`.
    assert!(icons.contains(&Call::Icon(0x15D, 8, 0x78)));
    // Cell 1 resumes at `bx = 0x1C` (the 9-col blank tail), so
    // `bl = 0x24`, `ah = 0x24/2 − 5 = 0xD`.
    assert!(icons.contains(&Call::Icon(0x160, 0xD, 0x78)));
    assert!(icons.iter().any(|c| matches!(c, Call::Icon(0x163, _, _))));
}
