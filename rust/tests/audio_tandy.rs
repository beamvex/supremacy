//! GAM-29/38 — the Tandy `int 0x80` driver (`audio::Tandy`, file
//! `0x1950`): the `ah`-dispatch contract (tick/stop/song/sfx/resume
//! plus the `ah ≥ 0x20` cell peek/poke), the four-channel tracker
//! rendered to the SN76489 on `0xC0`, and the `0x61 | 0x60` enable
//! write each tick makes.

mod common;

use supremacy::args::Sound;
use supremacy::audio::{Ports, Tandy};
use supremacy::front::audio::{Audio, Drv};

fn img() -> Vec<u8> {
    supremacy::img::image()
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
fn stop_mutes_all_four_attenuators() {
    let mut t = Tandy::new(Rec::new(), img().as_slice());
    t.dispatch(1, 0);
    // `0x9F` stepping `0x20` — the mute loop hits every attenuator.
    assert_eq!(
        t.ports.outs(),
        [(0xC0, 0x9F), (0xC0, 0xBF), (0xC0, 0xDF), (0xC0, 0xFF)]
    );
}

#[test]
fn cell_path_peeks_and_pokes_driver_state() {
    let mut t = Tandy::new(Rec::new(), img().as_slice());
    assert_eq!(t.dispatch(0x20, 0), 0); // `[0x23]` mus — idle
    t.dispatch(0x30, 0x77); // `[0x23]` = 0x77 (ah 0x30+i writes)
    assert_eq!(t.dispatch(0x20, 0), 0x77);
    t.dispatch(0x32, 0x12); // `[0x25]` pace cell
    assert_eq!(t.dispatch(0x22, 0), 0x12);
}

#[test]
fn song_load_arms_music() {
    let mut t = Tandy::new(Rec::new(), img().as_slice());
    t.dispatch(2, 0); // ah=2, al=0 — `cs:0x93E` song record 0
                      // `stop` ran first, then `[0x23]` armed (0xFF from the
                      // `dec`-style `mus -= 1`).
    assert_eq!(t.dispatch(0x20, 0), 0xFF);
    // Song tempo copied into both cells.
    assert_eq!(t.dispatch(0x24, 0), t.dispatch(0x25, 0));
}

#[test]
fn tick_writes_the_enable_bits_every_pass() {
    let mut t = Tandy::new(Rec::new(), img().as_slice());
    t.dispatch(0, 0);
    // `inb 0x61` read back the `last` shadow → `outb 0x61, 0x60`.
    assert!(t.ports.ops.contains(&(true, 0x61, 0)));
    assert!(t.ports.ops.contains(&(false, 0x61, 0x60)));
}

#[test]
fn sfx_sweep_emits_psg_latches() {
    let mut t = Tandy::new(Rec::new(), img().as_slice());
    t.dispatch(3, 0); // ah=3, al=0 — `cs:0x4EA` sfx record 0
    assert_eq!(t.dispatch(0x21, 0), 0xFF); // `[0x24]` armed
    t.ports.ops.clear();
    for _ in 0..8 {
        t.dispatch(0, 0);
    }
    let psg: Vec<u8> = t
        .ports
        .outs()
        .iter()
        .filter(|&&(p, _)| p == 0xC0)
        .map(|&(_, v)| v)
        .collect();
    // The sweep writes attenuation latches on its rotate selects.
    assert!(psg.iter().any(|&v| v & 0x90 == 0x90));
}

#[test]
fn song_tick_drives_psg_tone_writes() {
    let mut t = Tandy::new(Rec::new(), img().as_slice());
    t.dispatch(2, 0);
    t.ports.ops.clear();
    for _ in 0..40 {
        t.dispatch(0, 0);
    }
    let outs = t.ports.outs();
    let psg: Vec<u8> = outs
        .iter()
        .filter(|&&(p, _)| p == 0xC0)
        .map(|&(_, v)| v)
        .collect();
    // Channel tone latches (`0x80/0xA0` + low nibble) and attenuators.
    assert!(psg.iter().any(|&v| v & 0xF0 == 0x80 || v & 0xF0 == 0xA0));
    assert!(psg.iter().any(|&v| v & 0x90 == 0x90));
}

#[test]
fn cell_path_reaches_the_sfx_block() {
    let mut t = Tandy::new(Rec::new(), img().as_slice());
    t.dispatch(3, 0); // sfx record 0: bcnt 2, pcnt 1, sel 0x55, cur 0x03F0
    assert_eq!(t.dispatch(0x29, 0), 0xEA); // `[0x2C]` srec lo (0x4EA)
    assert_eq!(t.dispatch(0x2B, 0), 0x02); // `[0x2E]` bcnt
    assert_eq!(t.dispatch(0x2C, 0), 0x01); // `[0x2F]` pcnt
    assert_eq!(t.dispatch(0x2D, 0), 0x55); // `[0x30]` sel
    assert_eq!(t.dispatch(0x2F, 0), 0xF0); // `[0x32]` cur lo
    t.dispatch(0x3B, 7); // `[0x2E]` = 7
    assert_eq!(t.dispatch(0x2B, 0), 7);
}

#[test]
fn beat_overflow_renders_nothing_but_the_gate() {
    let mut t = Tandy::new(Rec::new(), img().as_slice());
    t.dispatch(2, 0); // arm the music engine
    t.dispatch(0x32, 0xFF); // `[0x25]` pace — every other tick overflows
    t.dispatch(0, 0); // `[0x2A]` = 0xFF — still a normal tick
    t.ports.ops.clear();
    t.dispatch(0, 0); // overflow → `jc` to the gate: no `0xC0` traffic
    assert!(t.ports.ops.iter().all(|&(is_in, p, _)| p == 0x61 || is_in));
    assert!(t.ports.outs().iter().all(|&(p, _)| p == 0x61));
}

#[test]
fn sfx_sweep_runs_once_per_tick() {
    let mut t = Tandy::new(Rec::new(), img().as_slice());
    t.dispatch(3, 0);
    t.ports.ops.clear();
    t.dispatch(0, 0);
    // ch2's sfx `ret` pops the render call — one `0xE7` noise write,
    // not one per sweep channel.
    let noise = t
        .ports
        .outs()
        .iter()
        .filter(|&&(p, v)| p == 0xC0 && v == 0xE7)
        .count();
    assert_eq!(noise, 1);
}

#[test]
fn sfx_transient_one_skips_the_channel() {
    let mut t = Tandy::new(Rec::new(), img().as_slice());
    t.dispatch(0x31, 1); // `[0x24]` = 1 — masked `0x81` without bit7
    t.dispatch(0, 0);
    // `jns` to the `0x1CBC` `ret`: ch0/1 mute as music, the render
    // stops at ch2 — no ch2/ch3 writes at all.
    assert_eq!(t.ports.outs(), [(0xC0, 0x9F), (0xC0, 0xBF), (0x61, 0x60)]);
}

#[test]
fn sfx_end_falls_back_to_music_and_finishes_the_pass() {
    let mut t = Tandy::new(Rec::new(), img().as_slice());
    t.dispatch(3, 0);
    t.dispatch(0x41, 1); // `[0x34]` end = 1 — next tick disarms
    t.dispatch(0, 0);
    // The `end` hit renders ch2 as music and the pass continues —
    // all four `vlatch|0xF` mutes land.
    assert_eq!(
        t.ports.outs(),
        [
            (0xC0, 0x9F),
            (0xC0, 0xBF),
            (0xC0, 0xDF),
            (0xC0, 0xFF),
            (0x61, 0x60)
        ]
    );
}

#[test]
fn engine_runs_the_tandy_driver() {
    let mut a = Audio::new(Sound::Tandy, img().as_slice(), 44_100);
    a.chan(0x8A80, 0); // the `ah=2` song wrapper
    a.render(&mut vec![0f32; 4410]);
    assert!(matches!(a.drv, Drv::Tandy(_)));
}

#[test]
fn tone_write_sends_the_full_10_bit_divisor() {
    // Zeroed module: `d16` reads are all `0`, so the sfx sweep's
    // `twrite(0xC0, cur | d16)` emits exactly the poked `cur` —
    // `0x169` latches `0xC0|9` then `(0x169>>4)&0x3F` = `0x16` (the
    // asm's `shr dx,1 ×2` / `shr al,1 ×2` at `0x1E05`).
    let img = vec![0u8; 0x2900];
    let mut t = Tandy::new(Rec::new(), img.as_slice());
    t.dispatch(0x31, 0xFF); // `[0x24]` sfx armed
    t.dispatch(0x3B, 0xFF); // `[0x2E]` bcnt — no wrap this tick
    t.dispatch(0x3C, 0xFF); // `[0x2F]` pcnt — no wrap this tick
    t.dispatch(0x3F, 0x69); // `[0x32]` cur lo
    t.dispatch(0x40, 0x01); // `[0x33]` cur hi
    t.dispatch(0, 0);
    let psg: Vec<u8> = t
        .ports
        .outs()
        .iter()
        .filter(|&&(p, _)| p == 0xC0)
        .map(|&(_, v)| v)
        .collect();
    // ch0/1 `vlatch|0xF` mutes, then the ch2 sweep pair + `0xE7`
    // noise + the `vol` writes on the `vsel`-picked channels.
    assert_eq!(psg, [0x9F, 0xBF, 0xC9, 0x16, 0xE7, 0xDF, 0xFF]);
}

#[test]
fn bus_synthesises_the_psg_while_enabled() {
    use supremacy::audio::Ports;
    use supremacy::front::audio::Bus;
    let mut b = Bus::new(44_100);
    b.outb(0x61, 0x60); // the tick tail's Tandy sound-enable
    b.outb(0xC0, 0x89); // ch0 tone latch, low nibble 9
    b.outb(0xC0, 0x16); // data byte → divisor 0x169 (~310 Hz)
    b.outb(0xC0, 0x90); // ch0 attenuator = 0 (full)
    let pcm: Vec<f32> = (0..4410).map(|_| b.sample()).collect();
    assert!(pcm.iter().any(|&s| s > 0.0));
    assert!(pcm.iter().any(|&s| s < 0.0));
}

#[test]
fn bus_psg_silent_without_the_enable_or_volume() {
    use supremacy::audio::Ports;
    use supremacy::front::audio::Bus;
    // No `0x61 | 0x60` — the PSG stays gated off.
    let mut b = Bus::new(44_100);
    b.outb(0xC0, 0x89);
    b.outb(0xC0, 0x16);
    b.outb(0xC0, 0x90);
    assert!((0..512).all(|_| b.sample() == 0.0));
    // Enabled but attenuator `0xF` — silent either way.
    b.outb(0x61, 0x60);
    b.outb(0xC0, 0x9F);
    assert!((0..512).all(|_| b.sample() == 0.0));
}

#[test]
fn engine_renders_audible_tandy_output() {
    let mut a = Audio::new(Sound::Tandy, img().as_slice(), 44_100);
    a.chan(0x8A80, 0); // song record 0 through the `ah=2` wrapper
    let mut pcm = vec![0f32; 44_100 * 2];
    a.render(&mut pcm);
    assert!(pcm.iter().any(|&s| s != 0.0));
}
