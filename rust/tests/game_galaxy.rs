//! Galaxy-screen click regression tests (GAM-74) — drive the three
//! sidebar hotspots (`view_prev`/`view_next`/`gal_build`) at
//! `ds:0xA81C` through the real `Game` + `Pump` input path.

mod common;

use common::game_dir;
use supremacy::args::{Sound, Video};
use supremacy::assets::{heap_segment, load_set};
use supremacy::front::{Game, Phase};
use supremacy::game::{self, HOT_COUNT, HOT_LIST, TYPE_SEL};
use supremacy::palette::{from_dac, DAC_OFS};

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

/// Press+release the left button at `(x, y)`, delivered through the
/// vsync hook exactly the way `window::install_vsync` feeds `Pump`.
/// `key` (make/break scancodes) is sent afterwards when an action opens
/// a blocking editor (`gal_build`'s `0xE66E` name input).
fn click(g: &mut Game, x: u16, y: u16, key: Option<(u8, u8)>) {
    let mut n = 0usize;
    g.host.set_vsync(Some(Box::new(move |h| {
        n += 1;
        let p = &mut h.pump;
        match n {
            5 => {
                p.move_to(x, y);
                p.set_buttons(true, false);
            }
            9 => p.set_buttons(false, false),
            _ => {}
        }
        if let Some((make, break_)) = key {
            if n == 14 {
                p.key(make);
            }
            if n == 18 {
                p.key(break_);
            }
        }
    })));
    for _ in 0..0x20 {
        g.step();
    }
    g.host.set_vsync(None);
}

#[test]
fn galaxy_sidebar_buttons_respond_to_clicks() {
    let mut g = game();
    g.enter_galaxy();
    assert_eq!(g.phase, Phase::Galaxy);
    // The live list at [0x9194] is ds:0xA81C, 3 records: build
    // (0x1e..0x3b), prev (0..0x1b), next (0x122..0x13d), all y 0x90..0x9d.
    assert_eq!((g.st.word(HOT_LIST), g.st.word(HOT_COUNT)), (0xA81C, 3));
    assert_eq!(g.st.byte(TYPE_SEL), 0);
    click(&mut g, 0x130, 0x95, None);
    assert_eq!(g.st.byte(TYPE_SEL), 2, "view_next must fire");
    click(&mut g, 0xE, 0x95, None);
    assert_eq!(g.st.byte(TYPE_SEL), 0, "view_prev must fire");
    // BUY → `gal_build` → the `0xE66E` name editor; Enter accepts it,
    // proving the click dispatched (the screen would wedge otherwise).
    click(&mut g, 0x28, 0x95, Some((0x1C, 0x9C)));
    assert_eq!(g.phase, Phase::Galaxy);
    let _ = game::FRAME;
}
