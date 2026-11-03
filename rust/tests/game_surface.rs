//! The uimode-7 surface screen's bay→surface transfer (GAM-76) —
//! drives the `0xABA4` list's `surf_slot` bay pick and `surf_assign`
//! deploy through the real `Game` + `Pump` input path: a unit aboard
//! the selected docking-bay machine lands on the planet record.

mod common;

use common::game_dir;
use supremacy::args::{Sound, Video};
use supremacy::assets::{heap_segment, load_set};
use supremacy::front::Game;
use supremacy::game::{self, FLEET_BASE, FLEET_STRIDE, MACH_BASE, REC_BASE, REC_CUR};
use supremacy::palette::{from_dac, DAC_OFS};

/// The four `ds:0x4FD0` cargo slots `0x57BC` fills.
const CARGO: u16 = 0x4FD0;
/// The `ds:0x9AB1` surface table `0x4190` fills.
const SURF: u16 = 0x9AB1;

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

/// Colonised record 0 with a type-`1` troop carrier docked in bay A
/// and fleet unit 0 aboard it (`+0x2` host, `+0x6` `0x2|0x4`) — the
/// rest of the fleet table cleared so the rebuilds are exact.
fn setup(g: &mut Game) -> (u16, u16, u16) {
    let rec = g.st.word(REC_BASE);
    g.st.set_word(usize::from(rec) + 0xC, 0xA);
    g.st.set_byte(usize::from(rec) + 0x18, 1);
    g.st.set_word(REC_CUR, rec);
    g.st.set_word(game::SEL_REC, rec);
    let m = u16::try_from(MACH_BASE).unwrap_or(0);
    for (i, &ch) in b"CRUISER.".iter().enumerate() {
        g.st.set_byte(usize::from(m) + i, ch);
    }
    g.st.set_byte(usize::from(m) + 8, 0xFF);
    g.st.set_byte(usize::from(m) + game::M_TYPE, 1);
    for i in 0..0x18u16 {
        let u =
            u16::try_from(FLEET_BASE).unwrap_or(0) + i * u16::try_from(FLEET_STRIDE).unwrap_or(0);
        g.st.set_word(usize::from(u) + 2, 0);
        g.st.set_byte(usize::from(u) + 6, 0);
    }
    let u = u16::try_from(FLEET_BASE).unwrap_or(0);
    g.st.set_word(usize::from(u), 0x40);
    g.st.set_word(usize::from(u) + 2, m);
    g.st.set_byte(usize::from(u) + 6, 0x6);
    (rec, m, u)
}

/// The `cs:0x4036` entry with `steps` scripted through the vsync hook
/// exactly the way `window::install_vsync` feeds `Pump`.
fn run(g: &mut Game, steps: impl FnMut(u16, &mut supremacy::front::Pump) + 'static) {
    let mut n = 0u16;
    let mut steps = steps;
    g.host.set_vsync(Some(Box::new(move |h| {
        n += 1;
        steps(n, &mut h.pump);
    })));
    game::dispatch_action(&mut g.ctx(), 0xAD86);
    g.host.set_vsync(None);
}

/// Bay A's hotspot at `(0x1C,0x2A)` then the first cargo row at
/// `(0x5E,0x20)`, a right-button exit — the asm's pick-then-assign
/// flow (list records 24 and 27).
#[test]
fn surface_assign_deploys_bay_unit() {
    let mut g = game();
    let (rec, m, u) = setup(&mut g);
    run(&mut g, |n, p| match n {
        5 => {
            p.move_to(0x1C, 0x2A);
            p.set_buttons(true, false);
        }
        9 => p.set_buttons(false, false),
        13 => {
            p.move_to(0x5E, 0x20);
            p.set_buttons(true, false);
        }
        17 => p.set_buttons(false, false),
        21 => p.set_buttons(false, true),
        25 => p.set_buttons(false, false),
        _ => {}
    });
    assert_eq!(
        g.st.word(game::SEL_MACH),
        m,
        "surf_slot selects the bay machine"
    );
    assert_eq!(g.st.byte(0x91DD), 0xFF, "the 0x42AB latch arms");
    // `surf_assign` lands the unit on the planet: `+0x2` = the record,
    // the `0x4` aboard bit dropped, `0x2` kept.
    assert_eq!(g.st.word(usize::from(u) + 2), rec);
    assert_eq!(g.st.byte(usize::from(u) + 6) & 0x6, 0x2);
    // The `0x57BC`/`0x4190` rebuilds: cargo slot empty, surface cell
    // 0 holds the unit.
    assert_eq!(g.vm.r16(&g.st, CARGO), 0);
    assert_eq!(g.st.word(usize::from(SURF)), u);
}

/// Clicking a cargo row with no bay selected takes the `0x4257`
/// "THAT'S EMPTY!" refuse — no unit fields move and the loop still
/// exits on the right button.
#[test]
fn surface_assign_empty_row_refuses() {
    let mut g = game();
    let (_rec, m, u) = setup(&mut g);
    run(&mut g, |n, p| match n {
        5 => {
            p.move_to(0x5E, 0x20);
            p.set_buttons(true, false);
        }
        9 => p.set_buttons(false, false),
        60 => p.set_buttons(false, true),
        64 => p.set_buttons(false, false),
        _ => {}
    });
    assert_eq!(g.st.word(usize::from(u) + 2), m);
    assert_eq!(g.st.byte(usize::from(u) + 6) & 0x6, 0x6);
    assert_eq!(g.st.word(game::SEL_MACH), 0);
}
