//! Emission helpers — write/compare generated `.rs` files and split
//! overlong `match` bodies across per-chunk files (the project caps
//! files at 200 lines).

use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode, Stdio};

/// The lint header every generated file carries.
const HEADER: &str = "#![allow(\n    clippy::too_many_lines,\n    clippy::cast_sign_loss,\n    clippy::match_same_arms,\n    clippy::unreachable,\n    missing_docs\n)]\n";

/// Accumulates writes; `--check` compares instead of writing.
pub struct Out {
    root: PathBuf,
    check: bool,
    drift: bool,
}

impl Out {
    /// `root` = the crate's `src/`; `check` = verify-only mode.
    pub fn new(root: PathBuf, check: bool) -> Self {
        Self {
            root,
            check,
            drift: false,
        }
    }

    /// Write (or verify) `rel` under `src/` with content `text`; the
    /// lint header is prepended and the result `rustfmt`-normalised so
    /// `--check` compares committed form, not emitter form.
    pub fn file(&mut self, rel: &str, text: &str) {
        let text = fmt(&format!("{HEADER}{text}"));
        let p = self.root.join(rel);
        if self.check {
            check(&p, &text, &mut self.drift);
            return;
        }
        if let Some(d) = p.parent() {
            std::fs::create_dir_all(d).unwrap_or_else(|e| panic!("{d:?}: {e}"));
        }
        std::fs::write(&p, text).unwrap_or_else(|e| panic!("{p:?}: {e}"));
        println!("{}", p.display());
    }

    /// Exit code — `3` when `--check` saw drift.
    pub fn finish(self) -> ExitCode {
        if self.check && self.drift {
            eprintln!("generated sources differ — run `cargo run --bin xgen`");
            return ExitCode::from(3);
        }
        ExitCode::SUCCESS
    }
}

fn check(p: &Path, text: &str, drift: &mut bool) {
    match std::fs::read_to_string(p) {
        Ok(old) if old == text => println!("{}: ok", p.display()),
        Ok(_) => {
            eprintln!("{}: drift", p.display());
            *drift = true;
        }
        Err(e) => {
            eprintln!("{}: {e}", p.display());
            *drift = true;
        }
    }
}

/// `rustfmt` (edition 2021) `text` → committed form.
fn fmt(text: &str) -> String {
    let mut c = Command::new("rustfmt")
        .args(["--edition", "2021"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("xgen needs `rustfmt` on PATH");
    c.stdin
        .take()
        .expect("stdin")
        .write_all(text.as_bytes())
        .expect("rustfmt stdin");
    let o = c.wait_with_output().expect("rustfmt");
    assert!(
        o.status.success(),
        "rustfmt: {}",
        String::from_utf8_lossy(&o.stderr)
    );
    String::from_utf8(o.stdout).expect("rustfmt emitted non-utf8")
}
