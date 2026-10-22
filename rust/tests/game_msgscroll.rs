//! The `0x6B86`/`[0x1270]` message-window scroll (GAM-59): each ticker
//! `0xFD` boundary scrolls the (x160..304, y91..189) region up one
//! 6px line, so earlier lines ride above the `0xB8` type row instead
//! of being overwritten by the next record.

mod common;

use common::game_dir;
use supremacy::args::{Sound, Video};
use supremacy::assets::{heap_segment, load_set};
use supremacy::front::Game;
use supremacy::game;
use supremacy::palette::{from_dac, DAC_OFS};
use supremacy::platform::MemFs;

fn game(mode: Video) -> Game {
    let img = supremacy::img::image();
    let set = load_set(&game_dir(), mode.stem(), heap_segment()).unwrap();
    let pal = from_dac(&img[DAC_OFS..DAC_OFS + 768]);
    Game::new(
        img,
        set,
        pal,
        mode,
        Sound::PcSpeaker,
        true,
        3,
        0x42,
        Box::new(MemFs::new()),
    )
}

/// Nonzero pixels of the `y` row inside the message window.
fn row_nz(g: &Game, y: usize) -> usize {
    let px = g.host.scr.pixels();
    px[y * 320 + 160..y * 320 + 304]
        .iter()
        .filter(|p| **p != 0)
        .count()
}

/// Nonzero pixels inside the whole `[0x127C]` snow-grid footprint —
/// the painter covers x `0xA0..0x130`, y `0x5A..0xC0`, a couple of
/// rows past the `0x5B..0xBE` scroll span at both ends.
fn win_nz(g: &Game) -> usize {
    (0x5A..0xC0usize).map(|y| row_nz(g, y)).sum()
}

/// The boot ticker message (`ds:0x6E30`, enqueued by `newgame_body`):
/// after its `0xFD` boundaries the typed lines must have scrolled up
/// out of the `0xB8` type row in every video mode.
#[test]
fn ticker_scrolls_lines_up() {
    for mode in [Video::Mcga, Video::Ega, Video::Cga, Video::Tga] {
        let mut g = game(mode);
        {
            let mut c = g.ctx();
            game::enqueue(&mut c, 0x6E30);
        }
        for _ in 0..2000 {
            g.step();
        }
        let above: usize = (91..184).map(|y| row_nz(&g, y)).sum();
        let cur = row_nz(&g, 184);
        assert!(
            above > 0,
            "{mode:?}: earlier lines should scroll above the type row"
        );
        assert!(cur > 0, "{mode:?}: the current line should type at y184");
    }
}

/// GAM-60 — the `0x32`-frame new-game wait fills the ticker window
/// with the `[0x127C]` snow-grid (MCG only — the other modes' twin
/// slot is unported). The `0x6E30` stream opens on a `0xFD`, so the
/// first `[0x1270]` scroll runs before the first glyph: it must sweep
/// the painted rect rather than carry the dots up behind the text.
#[test]
fn first_scroll_sweeps_the_window_snow() {
    for mode in [Video::Mcga, Video::Ega, Video::Cga, Video::Tga] {
        let mut g = game(mode);
        {
            let mut c = g.ctx();
            game::act_newgame(&mut c);
        }
        assert_eq!(
            win_nz(&g) > 0,
            mode == Video::Mcga,
            "{mode:?}: only the MCG painter fills the window"
        );
        // Step 0 takes the `0xFD` (blink arm), step 1 runs the scroll —
        // then glyphs start typing at y `0xB8`. Everything above the
        // type row must be black once the scroll has run.
        for _ in 0..4 {
            g.step();
        }
        assert_eq!(
            (0x5A..0xB8usize).map(|y| row_nz(&g, y)).sum::<usize>(),
            0,
            "{mode:?}: the first scroll must sweep the snow"
        );
        for _ in 0..60 {
            g.step();
        }
        let above: usize = (91..184).map(|y| row_nz(&g, y)).sum();
        assert!(above > 0, "{mode:?}: lines still scroll up");
        assert!(row_nz(&g, 184) > 0, "{mode:?}: the line still types");
    }
}
