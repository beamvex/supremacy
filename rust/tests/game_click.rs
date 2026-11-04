//! GAM-77 — the click path through the real `Game` + `Pump`.
//!
//! `[0x9CCB]` is both the pending-code latch and `menu_text`'s
//! stream-end mark: the credits crawl's last page writes `0xFF` into
//! it and the attract jumps to the picker on `!= 0`. A `Pump::write`
//! that only delivered on `== 0` left the picker on a dead latch —
//! `cs:0xA18E` consumes `== 1` only, so every button edge queued
//! behind the flag forever ("none of the mouse clicks work"). The
//! int-9/int-33 ISRs write the cell unconditionally, so the pump now
//! delivers over every value except a live pending code (`== 1`).

mod common;

use common::{game_dir, gsim};
use supremacy::args::{Sound, Video};
use supremacy::assets::{heap_segment, load_set};
use supremacy::front::{Game, Phase, Pump};
use supremacy::game::Vm;
use supremacy::game::{HOT_COUNT, HOT_LIST, KEY_CODE, KEY_PEND, UIMODE};
use supremacy::palette::{from_dac, DAC_OFS};

/// `u16` cast for the `ds:` cell constants.
fn a(k: usize) -> u16 {
    u16::try_from(k).unwrap_or(0)
}

/// Boot the `*IN` title flow.
fn game_title() -> Game {
    let img = supremacy::img::image();
    Game::new_title(
        img.clone(),
        load_set(&game_dir(), "MCGIN", heap_segment()).unwrap(),
        from_dac(&img[DAC_OFS..DAC_OFS + 768]),
        Video::Mcga,
        Sound::PcSpeaker,
        false,
        0x42,
        Box::new(supremacy::platform::MemFs::new()),
    )
}

/// Step until `HOT_LIST` is `want` (bounded so a hang fails, not spins).
fn until_list(g: &mut Game, want: u16) {
    for _ in 0..0x14000 {
        if g.st.word(HOT_LIST) == want {
            return;
        }
        g.step();
    }
    panic!("hotspot list {want:#06x} never installed");
}

/// The `+0xC` action word of hotspot record `i` of `list`.
fn action(g: &Game, list: u16, i: u16) -> u16 {
    g.vm.r16(&g.st, list + i * 0x14 + 0xC)
}

/// Centre of hotspot record `i`'s `x1,y1,x2,y2` rect.
fn centre(g: &Game, list: u16, i: u16) -> (u16, u16) {
    let r = |o| g.vm.r16(&g.st, list + i * 0x14 + o);
    (r(0) + (r(4) - r(0)) / 2, r(2) + (r(6) - r(2)) / 2)
}

/// Centre of the `list` record whose action word is `act`.
fn hotspot(g: &Game, list: u16, act: u16) -> (u16, u16) {
    for i in 0..g.st.word(HOT_COUNT) {
        if action(g, list, i) == act {
            return centre(g, list, i);
        }
    }
    panic!("list {list:#06x} has no {act:#06x} record");
}

/// Left-click at `(x, y)` fed through the live vsync hook — the same
/// feed `window::install_vsync` supplies — then stepped `cap` frames.
fn click(g: &mut Game, x: u16, y: u16, cap: usize) {
    let mut n = 0usize;
    g.host.set_vsync(Some(Box::new(move |h| {
        n += 1;
        match n {
            2 => {
                h.pump.move_to(x, y);
                h.pump.set_buttons(true, false);
            }
            6 => h.pump.set_buttons(false, false),
            _ => {}
        }
    })));
    for _ in 0..cap {
        g.step();
    }
    g.host.set_vsync(None);
}

/// The `0xFF` stream-end mark is not a pending code — the queued edge
/// overwrites it, like the ISRs overwrite the cell (GAM-77).
#[test]
fn queued_code_overwrites_the_stream_end_flag() {
    let (mut vm, mut st) = (Vm::new(), gsim::galaxy());
    let mut p = Pump::new(false);
    vm.w8(&mut st, a(KEY_PEND), 0xFF);
    p.set_buttons(true, false);
    p.write(&mut vm, &mut st);
    assert_eq!(
        (vm.r8(&st, a(KEY_PEND)), vm.r8(&st, a(KEY_CODE))),
        (1, 0x7E)
    );
}

/// A live pending code (`[0x9CCB] == 1`) is *not* clobbered — the
/// queue holds the next edge until the consumer drains the latch.
#[test]
fn queued_code_waits_behind_a_pending_code() {
    let (mut vm, mut st) = (Vm::new(), gsim::galaxy());
    let mut p = Pump::new(false);
    vm.w8(&mut st, a(KEY_PEND), 1);
    vm.w8(&mut st, a(KEY_CODE), 0x1C);
    p.set_buttons(true, false);
    p.write(&mut vm, &mut st);
    assert_eq!(vm.r8(&st, a(KEY_CODE)), 0x1C); // pending code kept
    vm.w8(&mut st, a(KEY_PEND), 0); // consumer drains
    p.write(&mut vm, &mut st);
    assert_eq!(vm.r8(&st, a(KEY_CODE)), 0x7E); // edge delivered
}

/// GAM-77 — run the attract to its end with no input (the `0xFF`
/// dead-latch state the report hit), then click through the whole
/// front end: picker "start" → shell row → galaxy sidebar. Every
/// click must dispatch on the frame it lands.
#[test]
fn clicks_dispatch_after_the_attract_stream_end() {
    let mut g = game_title();
    until_list(&mut g, 0x9D36); // the picker, via the crawl's 0xFF mark
    assert_eq!(g.vm.r8(&g.st, a(KEY_PEND)), 0xFF, "expected the dead latch");
    let (x, y) = hotspot(&g, 0x9D36, 0xF6D4); // the start record
    click(&mut g, x, y, 4);
    assert!(g.vm.title.is_none(), "start click never left the picker");
    assert_eq!(g.st.byte(UIMODE), 0);
    until_list(&mut g, 0xA0AC);
    let (x, y) = hotspot(&g, 0xA0AC, 0xA659); // the shell's galaxy row
    click(&mut g, x, y, 4);
    assert_eq!(g.phase, Phase::Galaxy);
    assert_eq!(g.st.byte(UIMODE), 5);
}
