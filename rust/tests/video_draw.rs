//! `Screen::draw_image` against fixture hashes from
//! `decompiled/tools/frame_hash.py` — a Python model of the per-mode draw
//! routines plus canvases composited from the verified reference PNGs.

mod common;

use common::{fnv, game_dir, read, root};
use supremacy::args::Video;
use supremacy::assets::{heap_segment, load_set};
use supremacy::video::Screen;

fn mode_of(set: &str) -> Video {
    match set {
        "CGA" => Video::Cga,
        "TGA" => Video::Tga,
        "EGA" => Video::Ega,
        _ => Video::Mcga,
    }
}

fn frames() -> Vec<(String, String, usize, usize, usize, u64, u64)> {
    let text = String::from_utf8(read(&root().join("rust/tests/fixtures/frames.tsv"))).unwrap();
    text.lines()
        .map(|l| {
            let f: Vec<&str> = l.split('\t').collect();
            let (lo, hi) = f[2].split_once('-').unwrap();
            (
                f[0].into(),
                f[1].into(),
                lo.parse().unwrap(),
                hi.parse().unwrap(),
                f[3].parse().unwrap(),
                u64::from_str_radix(f[4], 16).unwrap(),
                u64::from_str_radix(f[5], 16).unwrap(),
            )
        })
        .collect()
}

#[test]
fn mode_parameters_match_templates() {
    assert_eq!(Video::Mcga.int10_mode(), 0x13,);
    assert_eq!(
        (Video::Ega.int10_mode(), Video::Ega.vram_segment()),
        (0x0D, 0xA000)
    );
    assert_eq!(
        (Video::Cga.int10_mode(), Video::Cga.vram_segment()),
        (0x05, 0xB800)
    );
    assert_eq!(
        (Video::Tga.int10_mode(), Video::Tga.vram_segment()),
        (0x09, 0xB800)
    );
    assert!(Video::Mcga.programs_palette() && !Video::Ega.programs_palette());
}

#[test]
fn every_case_matches_fixture() {
    for (name, set_name, lo, hi, buf_len, buf_fnv, px_fnv) in frames() {
        let mode = mode_of(&set_name);
        let set = load_set(&game_dir(), &set_name, heap_segment()).unwrap();
        let mut screen = Screen::new(mode);
        assert_eq!(screen.buf.len(), buf_len, "{name}");
        for i in lo..=hi {
            screen.draw_image(&set, i);
        }
        assert_eq!(fnv(&screen.buf), buf_fnv, "{name} buffer");
        assert_eq!(fnv(&screen.pixels()), px_fnv, "{name} pixels");
    }
}
