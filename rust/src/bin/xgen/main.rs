//! `xgen` — the GAM-80 transpiler. Reads the committed `data/*.bin`
//! blobs (the shipped `GAME.EXE` regions) and rewrites the game's five
//! bytecode streams as checked-in native Rust tables/functions:
//!
//! - `data/dgroup.bin` `ds:0x51FB+` event scripts → `src/game/script/`
//! - `data/cs.bin` `cs:` draw-sequencer records → `src/game/seqtab/`
//! - `data/dgroup.bin` print strings with `0x01..0x04` control bytes
//!   → `src/game/texttab/` (resolved draw lists)
//! - `data/low.bin` PC-speaker + Tandy song streams →
//!   `src/audio/{pctab,ttab}/`
//! - `data/dgroup.bin` `ss:0x202F0` `menu_text` blocks →
//!   `src/front/text/stream/`
//!
//! Generated files carry a `@generated` header — regenerate with
//! `cargo run --bin xgen` after refreshing the blobs; `--check`
//! verifies the tree is current (exit 3 on drift).

use std::path::PathBuf;
use std::process::ExitCode;

mod blob;
mod menus;
mod out;
mod scripts;
mod seqs;
mod songs;
mod texts;

fn main() -> ExitCode {
    let root = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".into()));
    let check = std::env::args().any(|a| a == "--check");
    let b = blob::Blobs::load(&root.join("data"));
    let mut o = out::Out::new(root.join("src"), check);
    scripts::emit(&b, &mut o);
    seqs::emit(&b, &mut o);
    texts::emit(&b, &mut o);
    songs::emit(&b, &mut o);
    menus::emit(&b, &mut o);
    o.finish()
}
