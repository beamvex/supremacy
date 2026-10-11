//! GAM-31 — the frontend audio path: the ported PC-speaker `int 0x80`
//! driver (`audio::Pc`) emitting PIT writes on a recording bus, the
//! host `Bus` synthesising a square wave off the `0x42`/`0x61` port
//! state, and `front::audio::Audio` mapping `Call::Sfx`/`Chan`/`Sound`
//! to driver ops with the `[0x91D3]`/`[0x155]` gates the asm wrappers
//! implement.

mod common;

use common::{read, root};
use supremacy::args::Sound;
use supremacy::audio::{Pc, Ports};
use supremacy::front::audio::{Audio, Bus};
use supremacy::game::{Call, VmHost};

fn img() -> Vec<u8> {
    read(&root().join("decompiled/GAME.unpacked.exe"))
}

/// Records every `in`/`out`; `in` returns `last` (`audio_dev.rs` shape).
struct Rec {
    ops: Vec<(bool, u16, u8)>,
    last: u8,
}

impl Rec {
    fn new() -> Self {
        Self {
            ops: Vec::new(),
            last: 0,
        }
    }
    fn outs(&self) -> Vec<(u16, u8)> {
        self.ops
            .iter()
            .filter(|op| !op.0)
            .map(|op| (op.1, op.2))
            .collect()
    }
}

impl Ports for Rec {
    fn outb(&mut self, port: u16, val: u8) {
        self.ops.push((false, port, val));
    }
    fn inb(&mut self, port: u16) -> u8 {
        self.ops.push((true, port, self.last));
        self.last
    }
}

#[test]
fn sfx_load_then_tick_programs_the_pit() {
    let mut p = Pc::new(Rec::new(), img().as_slice());
    p.dispatch(3, 0); // ah=3, al=0 — record 0 (cs:0x461)
    p.dispatch(0, 0); // the int-8 service (ah=0)
    let outs = p.ports.outs();
    // cur 0x2580 + da 0x240 + add 0x2E0 → 0x2AA0 → 0x42 lo/hi.
    assert_eq!(
        outs[..4],
        [(0x43, 0xB6), (0x42, 0xA0), (0x42, 0x2A), (0x61, 3)]
    );
}

#[test]
fn song_load_arms_music_and_ticks() {
    let mut p = Pc::new(Rec::new(), img().as_slice());
    p.dispatch(2, 0); // ah=2, al=0 — song 0 (cs:0x6EB record)
    for _ in 0..4 {
        p.dispatch(0, 0);
    }
    let outs = p.ports.outs();
    assert!(outs.contains(&(0x43, 0xB6)));
    assert!(outs.iter().any(|&(p, _)| p == 0x42));
    assert!(outs.contains(&(0x61, 3)));
}

#[test]
fn sfx_end_disarms_the_sweep() {
    let mut p = Pc::new(Rec::new(), img().as_slice());
    p.dispatch(3, 0); // record 0 — end byte 0x0C
    for _ in 0..0x0D {
        p.dispatch(0, 0);
    }
    p.ports.ops.clear();
    // Past `end`, the sweep is off and the divisor stops changing.
    for _ in 0..8 {
        p.dispatch(0, 0);
    }
    assert!(!p.ports.outs().iter().any(|&(p, _)| p == 0x42));
}

#[test]
fn bus_square_wave_tracks_the_gate() {
    let mut b = Bus::new(44_100);
    // ~440 Hz: divisor = 1_193_182 / 440 ≈ 0x0A98.
    b.outb(0x43, 0xB6);
    b.outb(0x42, 0x98);
    b.outb(0x42, 0x0A);
    b.outb(0x61, 0x03);
    assert_eq!(b.divisor(), 0x0A98);
    let pcm: Vec<f32> = (0..4410).map(|_| b.sample()).collect();
    let crossings = pcm
        .windows(2)
        .filter(|w| w[0].signum() != w[1].signum())
        .count();
    // ~440 Hz square → ~88 sign flips in 0.1 s.
    assert!((70..=110).contains(&crossings));
    b.outb(0x61, 0);
    assert!((0..441).all(|_| b.sample() == 0.0));
}

#[test]
fn engine_sfx_reaches_the_bus() {
    let mut a = Audio::new(Sound::PcSpeaker, img().as_slice(), 44_100);
    a.sfx(0x0000, 0x0F); // the 0x8577 block — record 0
    let mut pcm = vec![0f32; 4410];
    a.render(&mut pcm);
    assert_eq!(a.drv.bus().divisor(), 0x2AA0);
    assert!(pcm.iter().any(|&s| s != 0.0));
}

#[test]
fn chan_8a80_is_gated_on_quiet() {
    let mut a = Audio::new(Sound::PcSpeaker, img().as_slice(), 44_100);
    a.set_quiet(0xFF); // [0x91D3] = 0xFF — the wrapper must not fire
    a.chan(0x8A80, 0);
    let mut pcm = vec![0f32; 22_050];
    a.render(&mut pcm);
    assert!(pcm.iter().all(|&s| s == 0.0));
    a.set_quiet(0);
    a.chan(0x8A80, 0); // unmuted — song 0 loads and ticks
    a.render(&mut pcm);
    assert!(pcm.iter().any(|&s| s != 0.0));
    assert_ne!(a.drv.bus().divisor(), 0);
}

#[test]
fn adlib_calls_route_to_the_far_exports() {
    let mut a = Audio::new(Sound::AdLib, img().as_slice(), 44_100);
    a.sfx(0x0026, 0x0F); // 0x8A5B + 0x8A3B + [cs:0x127C] select
    a.chan(0x8A5B, 0); // int 80 skipped (dev > 2) + far [cs:0x1280]
    a.chan(0x8A80, 0); // int-80 wrapper — gated out, nothing fires
    let mut pcm = vec![0f32; 4410];
    a.render(&mut pcm);
    // The exports route to the ported A/R driver — no drops.
    assert_eq!(a.dropped, 0);
}

#[test]
fn sfx_is_gated_on_quiet() {
    // 0x8577 returns early on [0x91D3] == 0xFF — before the exports.
    let mut a = Audio::new(Sound::PcSpeaker, img().as_slice(), 44_100);
    a.set_quiet(0xFF);
    a.sfx(0x0026, 0x0F);
    let mut pcm = vec![0f32; 4410];
    a.render(&mut pcm);
    assert!(pcm.iter().all(|&s| s == 0.0));
    // The int-8 service still runs — its idle path programs the
    // driver's `cx=2` divisor (1.19 MHz / 2, far past Nyquist), which
    // is how the asm silences an unarmed channel.
    assert_eq!(a.drv.bus().divisor(), 2);
}

#[test]
fn host_svc_sfx_renders_real_samples() {
    // The host path itself: Call::Sfx → Audio → driver → bus → pcm.
    let i = img();
    let set = supremacy::assets::load_set(
        &common::game_dir(),
        "MCG",
        supremacy::assets::heap_segment(),
    )
    .unwrap();
    let pal = supremacy::palette::from_dac(
        &i[supremacy::palette::DAC_OFS..supremacy::palette::DAC_OFS + 768],
    );
    let mut h = supremacy::front::Host::new(
        supremacy::video::Screen::new(supremacy::args::Video::Mcga),
        pal,
        set,
        i,
        false,
        Sound::PcSpeaker,
    );
    h.svc(Call::Sfx(0x0026, 0x0F)); // kind-7 ambient command
    let mut pcm = vec![0f32; 8820];
    h.snd.render(&mut pcm);
    assert_ne!(h.snd.drv.bus().divisor(), 0);
    assert!(pcm.iter().any(|&s| s != 0.0));
    assert_eq!(h.dropped, 0); // the sound calls aren't dropped anymore
}
