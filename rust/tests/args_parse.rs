//! `args::parse` against the PSP tails the shipped batch files produce.

mod common;

use common::psp_tail;
use supremacy::args::{parse, Sound, Video};

#[test]
fn supre_bat_gives_mcga_mouse_adlib() {
    let c = parse(&psp_tail("M M A")); // SUPRE.BAT: GAME M M A
    assert_eq!(c.video, Video::Mcga);
    assert!(!c.keyboard_mouse);
    assert_eq!(c.sound, Sound::AdLib);
    assert_eq!(c.timer_variant, 0x1D0);
}

#[test]
fn bare_invocation_uses_defaults() {
    let c = parse(&psp_tail(""));
    assert_eq!(c.video, Video::Mcga);
    assert_eq!(c.sound, Sound::PcSpeaker);
    assert_eq!(c.timer_variant, 0);
}

#[test]
fn every_video_letter_maps() {
    for (ch, want) in [
        ('C', Video::Cga),
        ('T', Video::Tga),
        ('E', Video::Ega),
        ('M', Video::Mcga),
    ] {
        assert_eq!(parse(&psp_tail(&format!("{ch}   "))).video, want);
    }
    assert_eq!(parse(&psp_tail("X   ")).video, Video::Mcga);
}

#[test]
fn every_sound_letter_maps() {
    let cases = [
        ('P', Sound::PcSpeaker, 0),
        ('T', Sound::Tandy, 0xD5),
        ('A', Sound::AdLib, 0x1D0),
        ('R', Sound::Roland, 0x1D0),
        ('Z', Sound::PcSpeaker, 0),
    ];
    for (ch, want, tvar) in cases {
        let c = parse(&psp_tail(&format!("X Y {ch}")));
        assert_eq!((c.sound, c.timer_variant), (want, tvar));
    }
}

#[test]
fn k_selects_keyboard_mouse() {
    assert!(parse(&psp_tail("X K ")).keyboard_mouse);
    assert!(!parse(&psp_tail("X M ")).keyboard_mouse);
}

#[test]
fn template_offsets_match_templates() {
    use supremacy::args::Video::*;
    for (v, want) in [(Cga, 0xD3A2), (Tga, 0xE4FD), (Ega, 0xC247), (Mcga, 0xB0EC)] {
        assert_eq!(v.template_seg(), want);
    }
}
