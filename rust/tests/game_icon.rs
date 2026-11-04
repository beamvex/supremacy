//! The `[0x128C]` positioned icon put (GAM-78) — `Host::icon` patches
//! the `0x15C`–`0x163` records' shipped `(0,0)` origins to `(ah·8, si)`
//! before the `[0x125A]` draw, so the pixels land inside the patched
//! rect instead of piling onto the top-left corner.

mod common;

use common::game_dir;
use supremacy::args::{Sound, Video};
use supremacy::assets::{heap_segment, load_set};
use supremacy::front::Game;
use supremacy::game::{Call, VmHost};
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

/// Pixels a `Call::Icon(img, ah, si)` changed, as `(x, y)` points.
fn painted(g: &mut Game, call: Call) -> Vec<(usize, usize)> {
    let before = g.host.scr.buf.clone();
    g.host.svc(call);
    (0..200usize)
        .flat_map(|y| (0..320usize).map(move |x| (x, y)))
        .filter(|&(x, y)| g.host.scr.buf[y * 320 + x] != before[y * 320 + x])
        .collect()
}

/// Two positioned puts: `0x163` at `ah = 8`, `0x15D` at `ah = 0xD`
/// (the `stact::cells` args) — every painted pixel stays inside the
/// `40×29` icon rect at `(ah·8, 0x78)`, nothing touches the shipped
/// `(0,0)` origin.
#[test]
fn icon_put_paints_inside_patched_rect() {
    let mut g = game();
    for (call, x0, x1) in [
        (Call::Icon(0x163, 8, 0x78), 8 * 8, 8 * 8 + 40),
        (Call::Icon(0x15D, 0xD, 0x78), 0xD * 8, 0xD * 8 + 40),
    ] {
        let px = painted(&mut g, call);
        assert!(!px.is_empty(), "{call:?} drew nothing");
        assert!(
            px.iter()
                .all(|&(x, y)| (x0..x1).contains(&x) && (0x78..0x78 + 29).contains(&y)),
            "{call:?} painted outside ({x0}..{x1}, 0x78..0x95)"
        );
    }
}
