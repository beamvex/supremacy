//! `extract-img` — regenerate the committed `rust/data/*.bin` blobs
//! (GAM-55) from the shipped `GAME.EXE`: the crate's own
//! [`exepack::unpack`][supremacy::exepack::unpack] produces the verified
//! load image, then each [`img::Region`][supremacy::img::Region] range
//! (a file offset minus the reference's `0xC00`-byte MZ header) is
//! sliced out — byte-exact provenance, no manual hex-surgery.
//!
//! ```text
//! extract-img [GAME.EXE] [--out DIR] [--check]
//! ```
//!
//! Defaults: `../GAME/GAME.EXE` and `data/` next to the crate
//! manifest. `--check` compares the slices against the committed blobs
//! without writing — CI proof the blobs match the packed original
//! (exit 3 on any mismatch or short image).

use std::path::PathBuf;
use std::process::ExitCode;

use supremacy::exepack::unpack;
use supremacy::img::{MZ_HDR, REGIONS};

struct Opt {
    exe: PathBuf,
    out: PathBuf,
    check: bool,
}

fn main() -> ExitCode {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let mut o = Opt {
        exe: root.join("../GAME/GAME.EXE"),
        out: root.join("data"),
        check: false,
    };
    let mut it = std::env::args().skip(1);
    while let Some(a) = it.next() {
        match a.as_str() {
            "--out" => o.out = it.next().map_or_else(|| o.out.clone(), PathBuf::from),
            "--check" => o.check = true,
            s => o.exe = PathBuf::from(s),
        }
    }
    run(&o)
}

fn run(o: &Opt) -> ExitCode {
    let packed = match std::fs::read(&o.exe) {
        Ok(b) => b,
        Err(e) => return fail("exe", &e.to_string()),
    };
    let u = match unpack(&packed) {
        Ok(u) => u,
        Err(e) => return fail("unpack", &e.to_string()),
    };
    let image = u.image;
    if image.len() + MZ_HDR != supremacy::img::IMAGE_LEN {
        return fail(
            "unpack",
            &format!(
                "image len {:#x} != {:#x}",
                image.len() + MZ_HDR,
                supremacy::img::IMAGE_LEN
            ),
        );
    }
    if o.check {
        check(&image)
    } else {
        write(&image, &o.out)
    }
}

/// Slice `r`'s range out of the load image (file ofs − the MZ header).
fn slice<'a>(image: &'a [u8], r: &supremacy::img::Region) -> Option<&'a [u8]> {
    image.get(r.ofs - MZ_HDR..r.ofs - MZ_HDR + r.bytes.len())
}

fn write(image: &[u8], out: &std::path::Path) -> ExitCode {
    for r in &REGIONS {
        let Some(b) = slice(image, r) else {
            return fail(r.name, "range past end of image");
        };
        let p = out.join(format!("{}.bin", r.name));
        if let Err(e) = std::fs::write(&p, b) {
            return fail(r.name, &e.to_string());
        }
        println!("{} -> {} ({:#x} bytes)", r.name, p.display(), b.len());
    }
    ExitCode::SUCCESS
}

fn check(image: &[u8]) -> ExitCode {
    let mut bad = false;
    for r in &REGIONS {
        match slice(image, r) {
            Some(b) if b == r.bytes => println!("{}: ok ({:#x} bytes)", r.name, b.len()),
            Some(_) => {
                eprintln!("{}: blob differs from GAME.EXE slice", r.name);
                bad = true;
            }
            None => {
                eprintln!("{}: range past end of image", r.name);
                bad = true;
            }
        }
    }
    if bad {
        ExitCode::from(3)
    } else {
        ExitCode::SUCCESS
    }
}

fn fail(what: &str, e: &str) -> ExitCode {
    eprintln!("{what}: {e}");
    ExitCode::FAILURE
}
