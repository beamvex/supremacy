//! Windowed frontend — `supremacy-run <GAME-dir> [V] [K] [S]
//! [--set STEM] [--exe PATH] [--preset N] [--seed N] [--galaxy]`.
//!
//! Loads the unpacked exe, one asset set and the mode palette, builds a
//! [`supremacy::front::Game`] (fresh galaxy via `select_galaxy` +
//! `new_game`), then runs the `minifb` loop. Savegames land in the
//! game dir (`--galaxy` skips the shell's new-game action).

use std::path::PathBuf;
use std::process::ExitCode;

use supremacy::args::parse;
use supremacy::assets::load_set;
use supremacy::front::{window, Game};
use supremacy::palette::{from_dac, Palette};

fn main() -> ExitCode {
    let mut pos: Vec<String> = Vec::new();
    let mut stem: Option<String> = None;
    let mut exe: Option<PathBuf> = None;
    let (mut preset, mut seed, mut galaxy) = (3usize, 0x42u32, false);
    let mut it = std::env::args().skip(1);
    while let Some(a) = it.next() {
        match a.as_str() {
            "--set" => stem = it.next(),
            "--exe" => exe = it.next().map(PathBuf::from),
            "--preset" => preset = it.next().and_then(|v| v.parse().ok()).unwrap_or(3),
            "--seed" => seed = it.next().and_then(|v| v.parse().ok()).unwrap_or(0x42),
            "--galaxy" => galaxy = true,
            _ => pos.push(a),
        }
    }
    let Some(dir) = pos.first() else {
        eprintln!("usage: supremacy-run <GAME-dir> [V] [K] [S] [--set S] [--exe f] [--preset N] [--seed N] [--galaxy]");
        return ExitCode::from(2);
    };
    run(PathBuf::from(dir), pos, stem, exe, preset, seed, galaxy)
}

#[allow(clippy::too_many_arguments)]
fn run(
    dir: PathBuf,
    pos: Vec<String>,
    stem: Option<String>,
    exe: Option<PathBuf>,
    preset: usize,
    seed: u32,
    galaxy: bool,
) -> ExitCode {
    let tail = tail_bytes(pos.get(1), pos.get(2), pos.get(3));
    let cfg = parse(&tail);
    let stem = stem.unwrap_or_else(|| cfg.video.stem().to_owned());
    let img = match load_img(&dir, exe) {
        Ok(i) => i,
        Err(e) => return fail("exe", e),
    };
    let set = match load_set(&dir, &stem, 0x3A22) {
        Ok(s) => s,
        Err(e) => return fail(&stem, e),
    };
    let pal = palette(&img, cfg.video);
    boot(dir, img, set, pal, &cfg, preset, seed, galaxy)
}

#[allow(clippy::too_many_arguments)]
fn boot(
    dir: PathBuf,
    img: Vec<u8>,
    set: supremacy::assets::AssetSet,
    pal: Palette,
    cfg: &supremacy::args::Config,
    preset: usize,
    seed: u32,
    galaxy: bool,
) -> ExitCode {
    let files = Box::new(supremacy::platform::HostFs::new(dir));
    let mut g = Game::new(
        img,
        set,
        pal,
        cfg.video,
        cfg.keyboard_mouse,
        preset,
        seed,
        files,
    );
    if galaxy {
        g.enter_galaxy();
    }
    match window::run(&mut g) {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => fail("minifb", std::io::Error::other(e)),
    }
}

/// Palette for the mode — MCGA reads the master DAC table out of the
/// unpacked exe, EGA/Tandy use the stock 16-colour set, CGA palette 1
/// (same selection as `supremacy.rs::palette`).
fn palette(img: &[u8], v: supremacy::args::Video) -> Palette {
    use supremacy::args::Video;
    match v {
        Video::Mcga => from_dac(
            img.get(supremacy::palette::DAC_OFS..supremacy::palette::DAC_OFS + 768)
                .unwrap_or(&[]),
        ),
        Video::Cga => Palette::cga(),
        Video::Ega | Video::Tga => Palette::ega16(),
    }
}

fn load_img(dir: &std::path::Path, exe: Option<PathBuf>) -> std::io::Result<Vec<u8>> {
    std::fs::read(exe.unwrap_or_else(|| dir.join("../decompiled/GAME.unpacked.exe")))
}

fn fail(what: &str, e: std::io::Error) -> ExitCode {
    eprintln!("{what}: {e}");
    ExitCode::FAILURE
}

/// Build a PSP-style command tail — see `supremacy.rs::tail_bytes`.
fn tail_bytes(v: Option<&String>, k: Option<&String>, s: Option<&String>) -> Vec<u8> {
    let letter = |o: Option<&String>| o.and_then(|s| s.bytes().next()).unwrap_or(b' ');
    let body = [letter(v), b' ', letter(k), b' ', letter(s)];
    let mut t = vec![u8::try_from(body.len() + 1).unwrap_or(0x7F), b' '];
    t.extend_from_slice(&body);
    t
}
