//! Headless smoke mode for CI (GAM-24) — `supremacy-smoke <GAME-dir>
//! [V] [K] [S] [--set STEM] [--exe PATH] [--preset N] [--seed N]
//! [--galaxy] [--ticks N] [--script F.scr] [--expect "k=v,…"]
//! [--frame F.ppm]`.
//!
//! Boots a [`front::Game`][supremacy::front::Game] like `supremacy-run`
//! — minus `minifb`/`cpal`, so it builds and runs with no display or
//! audio device — steps `--ticks` frames (default 300), applies
//! `--script` ops on their tick (the `difftest` text format; `done`
//! ends the run early), then prints the `smoke::Digest` line.
//! `--expect` compares a `key=value` spec, exit 3 on mismatch;
//! `--frame` dumps the final VRAM through the palette as a PPM.

use std::path::{Path, PathBuf};
use std::process::ExitCode;

use supremacy::args::{parse, Config, Video};
use supremacy::assets::{heap_segment, load_set};
use supremacy::difftest as dt;
use supremacy::front::Game;
use supremacy::palette::{from_dac, Palette, DAC_LEN, DAC_OFS};
use supremacy::platform::{DosFiles, MemFs};
use supremacy::smoke::{self, Digest};

/// Parsed command line.
struct Opt {
    /// Positionals — `[0]` is the game dir, the rest the V/K/S letters.
    pos: Vec<String>,
    set: Option<String>,
    exe: Option<PathBuf>,
    script: Option<PathBuf>,
    frame: Option<PathBuf>,
    expect: Option<String>,
    preset: usize,
    seed: u32,
    ticks: u16,
    galaxy: bool,
}

fn main() -> ExitCode {
    let o = Opt::parse();
    if o.pos.is_empty() {
        eprintln!(
            "usage: supremacy-smoke <GAME-dir> [V] [K] [S] [--set S] [--exe f] \
             [--preset N] [--seed N] [--galaxy] [--ticks N] [--script f.scr] \
             [--expect \"video=…,state=…\"] [--frame f.ppm]"
        );
        return ExitCode::from(2);
    }
    run(&o)
}

impl Opt {
    fn parse() -> Self {
        let mut o = Self {
            pos: Vec::new(),
            set: None,
            exe: None,
            script: None,
            frame: None,
            expect: None,
            preset: 3,
            seed: 0x42,
            ticks: 300,
            galaxy: false,
        };
        let mut it = std::env::args().skip(1);
        while let Some(a) = it.next() {
            o.flag(&a, &mut it);
        }
        o
    }

    fn flag(&mut self, a: &str, it: &mut impl Iterator<Item = String>) {
        match a {
            "--set" => self.set = it.next(),
            "--exe" => self.exe = it.next().map(PathBuf::from),
            "--script" => self.script = it.next().map(PathBuf::from),
            "--frame" => self.frame = it.next().map(PathBuf::from),
            "--expect" => self.expect = it.next(),
            "--preset" => self.preset = num(it.next(), 3),
            "--seed" => self.seed = num(it.next(), 0x42),
            "--ticks" => self.ticks = num(it.next(), 300),
            "--galaxy" => self.galaxy = true,
            _ => self.pos.push(a.to_owned()),
        }
    }
}

fn num<T: std::str::FromStr>(v: Option<String>, d: T) -> T {
    v.and_then(|s| s.parse().ok()).unwrap_or(d)
}

fn run(o: &Opt) -> ExitCode {
    let dir = PathBuf::from(&o.pos[0]);
    let tail = tail_bytes(o.pos.get(1), o.pos.get(2), o.pos.get(3));
    let cfg = parse(&tail);
    let stem = o.set.clone().unwrap_or_else(|| cfg.video.stem().to_owned());
    let Some(mut g) = boot(&dir, &stem, o, cfg) else {
        return ExitCode::FAILURE;
    };
    let d = smoke::run(&mut g, &ops(o), o.ticks);
    println!("smoke {d}");
    if let Some(f) = &o.frame {
        let _ = write_ppm(f, &g);
    }
    expect(&d, o.expect.as_deref())
}

/// `Game` from the dir/exe/args; `MemFs` keeps savegames off disk.
fn boot(dir: &Path, stem: &str, o: &Opt, cfg: Config) -> Option<Game> {
    let img = match load_img(o.exe.clone()) {
        Ok(i) => i,
        Err(e) => {
            eprintln!("exe: {e}");
            return None;
        }
    };
    let set = match load_set(dir, stem, heap_segment()) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("{stem}: {e}");
            return None;
        }
    };
    let pal = palette(&img, cfg.video);
    let files = Box::new(MemFs::new()) as Box<dyn DosFiles>;
    let mut g = Game::new(
        img,
        set,
        pal,
        cfg.video,
        cfg.sound,
        cfg.keyboard_mouse,
        o.preset,
        o.seed,
        files,
    );
    g.host.set_assets(dir, stem);
    if o.galaxy {
        g.enter_galaxy();
    }
    Some(g)
}

/// The `--script` ops, or none — same text format `difftest` parses.
fn ops(o: &Opt) -> Vec<dt::Rec> {
    o.script
        .as_ref()
        .map(|p| dt::parse(&std::fs::read_to_string(p).unwrap_or_default()).0)
        .unwrap_or_default()
}

/// The unpacked image — `--exe` reads an alternate file, otherwise the
/// committed `data/*.bin` blobs (GAM-55).
fn load_img(exe: Option<PathBuf>) -> std::io::Result<Vec<u8>> {
    exe.map_or_else(|| Ok(supremacy::img::image()), std::fs::read)
}

/// Mode palette — the `run.rs` selection: MCGA from the image's DAC
/// table, CGA palette 1, the stock 16-colour set for EGA/Tandy.
fn palette(img: &[u8], v: Video) -> Palette {
    match v {
        Video::Mcga => from_dac(img.get(DAC_OFS..DAC_OFS + DAC_LEN).unwrap_or(&[])),
        Video::Cga => Palette::cga(),
        Video::Ega | Video::Tga => Palette::ega16(),
    }
}

/// `--expect` — `Ok` exit on no spec or all-match, 3 on any mismatch.
fn expect(d: &Digest, spec: Option<&str>) -> ExitCode {
    let Some(spec) = spec else {
        return ExitCode::SUCCESS;
    };
    let bad = d.check(spec);
    for m in &bad {
        eprintln!("expect: {m}");
    }
    if bad.is_empty() {
        ExitCode::SUCCESS
    } else {
        ExitCode::from(3)
    }
}

/// The final frame through the palette — a `320×200` P6 PPM.
fn write_ppm(path: &Path, g: &Game) -> std::io::Result<()> {
    let mut b = b"P6 320 200 255\n".to_vec();
    for &i in &g.host.scr.pixels() {
        b.extend_from_slice(&g.host.pal.rgb(usize::from(i)));
    }
    std::fs::write(path, b)
}

/// PSP-style command tail — see `supremacy.rs::tail_bytes`.
fn tail_bytes(v: Option<&String>, k: Option<&String>, s: Option<&String>) -> Vec<u8> {
    let letter = |o: Option<&String>| o.and_then(|s| s.bytes().next()).unwrap_or(b' ');
    let body = [letter(v), b' ', letter(k), b' ', letter(s)];
    let mut t = vec![u8::try_from(body.len() + 1).unwrap_or(0x7F), b' '];
    t.extend_from_slice(&body);
    t
}
