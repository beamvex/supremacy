//! GAM-29 — the Tandy `int 0x80` driver (`audio::Tandy`, file
//! `0x1950`): the `ah`-dispatch contract (tick/stop/song/sfx/resume
//! plus the `ah ≥ 0x20` cell peek/poke), the four-channel tracker
//! rendered to the SN76489 on `0xC0`, and the `0x61 | 0x60` enable
//! write each tick makes.

mod common;

use common::{read, root};
use supremacy::args::Sound;
use supremacy::audio::{Ports, Tandy};
use supremacy::front::audio::{Audio, Drv};

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
fn engine_runs_the_tandy_driver() {
    let mut a = Audio::new(Sound::Tandy, img().as_slice(), 44_100);
    a.chan(0x8A80, 0); // the `ah=2` song wrapper
    a.render(&mut vec![0f32; 4410]);
    assert!(matches!(a.drv, Drv::Tandy(_)));
}
