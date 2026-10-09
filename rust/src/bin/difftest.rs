//! DOSBox-X differential-testing driver (GAM-25) — runs the original
//! `GAME.EXE` under DOSBox-X and the port on the same input script and
//! diffs the guest-state dumps.
//!
//! ```text
//! difftest <script.scr> [--root DIR] [--out DIR] [--dosbox PATH]
//!          [--args "M K P"] [--timeout SECS] [--settle N]
//!          [--port-only] [--no-boot]
//! ```
//!
//! Writes `difftest/SCRIPT.BIN` + `difftest/out/dosbox-x.conf`, spawns
//! DOSBox-X (skipped when no binary is found or `--port-only`), waits
//! for `DONE.BIN`, then replays the script on the port — booting from
//! the `D01`/`D02`/`D03` checkpoint dumps when present — and diffs the
//! artefact sets into `out/report.txt`.

use std::path::PathBuf;
use std::process::ExitCode;
use std::time::Duration;

use supremacy::args::{Sound, Video};
use supremacy::assets::{heap_segment, load_set};
use supremacy::difftest as dt;
use supremacy::front::Game;
use supremacy::palette::from_dac;
use supremacy::platform::MemFs;

struct Opt {
    script: PathBuf,
    root: PathBuf,
    out: PathBuf,
    dosbox: Option<PathBuf>,
    args: String,
    timeout: u64,
    settle: u16,
    port_only: bool,
    no_boot: bool,
}

fn main() -> ExitCode {
    let mut it = std::env::args().skip(1);
    let mut o = Opt {
        script: PathBuf::new(),
        root: PathBuf::from(".."),
        out: PathBuf::new(),
        dosbox: None,
        args: "M K P".to_owned(),
        timeout: 240,
        settle: 8,
        port_only: false,
        no_boot: false,
    };
    while let Some(a) = it.next() {
        match a.as_str() {
            "--root" => o.root = it.next().map_or(o.root, PathBuf::from),
            "--out" => o.out = it.next().map_or(o.out, PathBuf::from),
            "--dosbox" => o.dosbox = it.next().map(PathBuf::from),
            "--args" => o.args = it.next().unwrap_or_default(),
            "--timeout" => o.timeout = it.next().and_then(|v| v.parse().ok()).unwrap_or(240),
            "--settle" => o.settle = it.next().and_then(|v| v.parse().ok()).unwrap_or(8),
            "--port-only" => o.port_only = true,
            "--no-boot" => o.no_boot = true,
            s => o.script = PathBuf::from(s),
        }
    }
    if o.script.as_os_str().is_empty() {
        eprintln!("usage: difftest <script.scr> [--root DIR] [--out DIR] [--port-only]");
        return ExitCode::from(2);
    }
    if o.out.as_os_str().is_empty() {
        o.out = o.root.join("difftest/out");
    }
    run(&o)
}

fn run(o: &Opt) -> ExitCode {
    let script = std::fs::read_to_string(&o.script).unwrap_or_default();
    let s = dt::parse(&script);
    println!("{} ops parsed", s.0.len());
    write_script_bin(o, &s);
    let dos_ok = dos_pass(o);
    let Some(mut g) = game(o, dos_ok) else {
        return ExitCode::FAILURE;
    };
    let port_dir = o.out.join("port");
    let _ = std::fs::remove_dir_all(&port_dir);
    let boot_tick = if dos_ok && !o.no_boot {
        dt::boot_tick(&s.0)
    } else {
        0
    };
    match dt::run(&mut g, &s.0, &port_dir, boot_tick, o.settle) {
        Ok(steps) => println!("port replay: {steps} steps (from tick {boot_tick})"),
        Err(e) => {
            eprintln!("port replay: {e}");
            return ExitCode::FAILURE;
        }
    }
    report(o, &s)
}

fn write_script_bin(o: &Opt, s: &dt::Script) {
    let bin = o.root.join("difftest/SCRIPT.BIN");
    if let Err(e) = std::fs::write(&bin, s.to_bin()) {
        eprintln!("{}: {e}", bin.display());
    }
}

fn dos_pass(o: &Opt) -> bool {
    if o.port_only {
        return false;
    }
    let bin = o.dosbox.clone().or_else(dt::find_dosbox);
    let Some(bin) = bin else {
        eprintln!("dosbox-x not found — port-only pass");
        return false;
    };
    let _ = std::fs::remove_dir_all(&o.out);
    let _ = std::fs::create_dir_all(&o.out);
    let conf_path = o.out.join("dosbox-x.conf");
    let conf = dt::conf(
        &o.root.canonicalize().unwrap_or_else(|_| o.root.clone()),
        &o.args,
    );
    if std::fs::write(&conf_path, conf).is_err() {
        return false;
    }
    println!("dos pass: {}", bin.display());
    let t = Duration::from_secs(o.timeout);
    dt::run_dos(&bin, &conf_path, &o.out, t).is_ok()
}

fn game(o: &Opt, boot: bool) -> Option<Game> {
    let img = std::fs::read(o.root.join("decompiled/GAME.unpacked.exe")).ok()?;
    let set = load_set(&o.root.join("GAME"), "MCG", heap_segment()).ok()?;
    let pal = from_dac(img.get(0x15034..0x15034 + 768).unwrap_or(&[]));
    let files = Box::new(MemFs::new()) as Box<dyn supremacy::platform::DosFiles>;
    if boot && !o.no_boot {
        if let Some((ds, seed)) = dt::read_boot(&o.out) {
            println!("booting port from DOS checkpoint (rng {seed:08X})");
            return Some(dt::from_dumps(
                img,
                set,
                pal,
                Video::Mcga,
                Sound::PcSpeaker,
                true,
                &ds,
                seed,
                files,
            ));
        }
    }
    Some(Game::new(
        img,
        set,
        pal,
        Video::Mcga,
        Sound::PcSpeaker,
        true,
        3,
        0x42,
        files,
    ))
}

fn report(o: &Opt, s: &dt::Script) -> ExitCode {
    let mut r = dt::Report::default();
    for rec in &s.0 {
        let Some(name) = artefact(rec.op) else {
            continue;
        };
        let dos = std::fs::read(o.out.join(&name)).unwrap_or_default();
        let port = std::fs::read(o.out.join("port").join(&name)).unwrap_or_default();
        r.diffs.push(dt::compare(&name, &dos, &port));
    }
    let text = r.render();
    let _ = std::fs::write(o.out.join("report.txt"), &text);
    print!("{text}");
    if o.port_only || r.diffs.iter().all(|d| d.diffs == 0) {
        ExitCode::SUCCESS
    } else {
        ExitCode::from(3)
    }
}

fn artefact(op: dt::Op) -> Option<String> {
    match op {
        dt::Op::Dump { id, .. } => Some(dt::dump_name(id)),
        dt::Op::Pal { id } => Some(dt::pal_name(id)),
        dt::Op::Stat { id } => Some(dt::stat_name(id)),
        _ => None,
    }
}
