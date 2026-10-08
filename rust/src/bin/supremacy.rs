//! Runnable shell for the Supremacy port — wires `args` → `assets` →
//! `video` → `palette` and dumps a rendered frame as PPM.
//!
//! Usage: `supremacy <GAME-dir> [V] [K] [S] [--set STEM] [--idx N]
//! [--exe PATH] [--out frame.ppm]`
//!
//! `V`/`K`/`S` are the single-letter DOS arguments (`GAME M M A` =
//! `supremacy GAME M ' ' A`); the MCGA palette comes from the master DAC
//! table in `GAME.unpacked.exe` (`--exe`, default `../decompiled` next to
//! the game dir).

use std::path::PathBuf;
use std::process::ExitCode;

use supremacy::args::{parse, Video};
use supremacy::assets::load_set;
use supremacy::palette::{from_dac, Palette};
use supremacy::video::Screen;

fn main() -> ExitCode {
    let mut pos: Vec<String> = Vec::new();
    let mut set_stem: Option<String> = None;
    let mut idx = 0usize;
    let mut exe: Option<PathBuf> = None;
    let mut out = PathBuf::from("frame.ppm");
    let mut it = std::env::args().skip(1);
    while let Some(a) = it.next() {
        match a.as_str() {
            "--set" => set_stem = it.next(),
            "--idx" => idx = it.next().and_then(|v| v.parse().ok()).unwrap_or(0),
            "--exe" => exe = it.next().map(PathBuf::from),
            "--out" => out = it.next().map_or_else(|| out.clone(), PathBuf::from),
            _ => pos.push(a),
        }
    }
    let Some(dir) = pos.first() else {
        eprintln!("usage: supremacy <GAME-dir> [V] [K] [S] [--set S] [--idx N] [--out f.ppm]");
        return ExitCode::from(2);
    };
    let dir = PathBuf::from(dir);
    let tail = tail_bytes(pos.get(1), pos.get(2), pos.get(3));
    let cfg = parse(&tail);
    run(&dir, cfg, set_stem, idx, exe, &out)
}

/// Build a PSP-style command tail from the letter arguments — cell 0 =
/// count, 1 = space, then the letters at the fixed `0x82`/`0x84`/`0x86`
/// offsets the parser reads.
fn tail_bytes(v: Option<&String>, k: Option<&String>, s: Option<&String>) -> Vec<u8> {
    let letter = |o: Option<&String>| o.and_then(|s| s.bytes().next()).unwrap_or(b' ');
    let body = [letter(v), b' ', letter(k), b' ', letter(s)];
    let mut t = vec![u8::try_from(body.len() + 1).unwrap_or(0x7F), b' '];
    t.extend_from_slice(&body);
    t
}

fn run(
    dir: &std::path::Path,
    cfg: supremacy::args::Config,
    stem: Option<String>,
    idx: usize,
    exe: Option<PathBuf>,
    out: &std::path::Path,
) -> ExitCode {
    let stem = stem.unwrap_or_else(|| cfg.video.stem().to_owned());
    let set = match load_set(dir, &stem, 0x3A22) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("{stem}: {e}");
            return ExitCode::FAILURE;
        }
    };
    let mut scr = Screen::new(cfg.video);
    scr.draw_image(&set, idx);
    let pal = match palette(dir, cfg.video, exe) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("palette: {e}");
            return ExitCode::FAILURE;
        }
    };
    let r = write_ppm(out, &scr.pixels(), &pal);
    match r {
        Ok(()) => {
            println!(
                "{} idx {} -> {} ({} mode, sound {:?})",
                stem,
                idx,
                out.display(),
                cfg.video.stem(),
                cfg.sound
            );
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("{}: {e}", out.display());
            ExitCode::FAILURE
        }
    }
}

/// Palette for the mode — MCGA reads the master DAC table out of the
/// unpacked exe (dgroup `0x4994`), EGA/Tandy use the stock 16-colour set,
/// CGA palette 1 (per `tools/extract_png.py`).
fn palette(dir: &std::path::Path, v: Video, exe: Option<PathBuf>) -> std::io::Result<Palette> {
    Ok(match v {
        Video::Mcga => {
            let p = exe.unwrap_or_else(|| dir.join("../decompiled/GAME.unpacked.exe"));
            let img = std::fs::read(&p)?;
            from_dac(
                img.get(supremacy::palette::DAC_OFS..supremacy::palette::DAC_OFS + 768)
                    .unwrap_or(&[]),
            )
        }
        Video::Cga => Palette::cga(),
        Video::Ega | Video::Tga => Palette::ega16(),
    })
}

fn write_ppm(path: &std::path::Path, px: &[u8], pal: &Palette) -> std::io::Result<()> {
    let mut b = b"P6 320 200 255\n".to_vec();
    for &i in px {
        b.extend_from_slice(&pal.rgb(usize::from(i)));
    }
    std::fs::write(path, b)
}
