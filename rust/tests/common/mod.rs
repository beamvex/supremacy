//! Shared test helpers: fixture paths, the manifest table, FNV-1a, and
//! minimal MZ parsing for cross-checking `to_exe`/`unpack` output.
//!
//! Compiled once per integration test — not every helper is used by every
//! crate.
#![allow(dead_code)]

use std::fs;
use std::path::{Path, PathBuf};

pub mod gsim;
pub mod rig;

/// Repo root (crate lives in `<root>/rust`).
pub fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("..")
}

/// The shipped game directory with the `.BIN`/`.GPH` asset sets.
pub fn game_dir() -> PathBuf {
    root().join("GAME")
}

/// Read a file or fail the test with a useful message.
pub fn read(p: &Path) -> Vec<u8> {
    fs::read(p).unwrap_or_else(|e| panic!("{}: {e}", p.display()))
}

/// FNV-1a 64 — same parameters as the Python fixture generator.
pub fn fnv(b: &[u8]) -> u64 {
    let mut h = 0xcbf2_9ce4_8422_2325u64;
    for &x in b {
        h ^= u64::from(x);
        h = h.wrapping_mul(0x0000_0100_0000_01b3);
    }
    h
}

/// One row of `fixtures/manifest.tsv` (generated from
/// `decompiled/assets/manifest.json` + the Python reference decoder).
pub struct Row {
    /// Asset set name, e.g. `MCG`, `EGAIN`.
    pub set: String,
    /// `ofs_para` from the `.GPH` record.
    pub ofs_para: u16,
    /// Expected decoded byte count (`w*h`, ×4 planes for EGA).
    pub decoded: usize,
    /// Expected FNV-1a of the decoded bytes.
    pub hash: u64,
}

/// All 1,703 manifest rows.
pub fn manifest() -> Vec<Row> {
    let text = String::from_utf8(read(&root().join("rust/tests/fixtures/manifest.tsv"))).unwrap();
    text.lines().map(parse_row).collect()
}

fn parse_row(l: &str) -> Row {
    let f: Vec<&str> = l.split('\t').collect();
    Row {
        set: f[0].into(),
        ofs_para: f[6].parse().unwrap(),
        decoded: f[9].parse().unwrap(),
        hash: u64::from_str_radix(f[10], 16).unwrap(),
    }
}

/// PSP command tail for `args::parse` — `[len, ' ', bytes…]` as DOS writes
/// it at `es:0x80`.
pub fn psp_tail(s: &str) -> Vec<u8> {
    let mut v = vec![u8::try_from(s.len()).unwrap_or(0x7F), b' '];
    v.extend_from_slice(s.as_bytes());
    v
}

/// Load-image bytes of an MZ executable (everything after the header).
pub fn load_image(exe: &[u8]) -> &[u8] {
    let hdr = usize::from(u16le(exe, 0x08)) * 16;
    exe.get(hdr..).unwrap_or(&[])
}

/// Relocation sites of an MZ executable as linear image offsets
/// (`seg*16 + ofs` pairs from the table at `[0x18]`).
pub fn mz_relocs(exe: &[u8]) -> Vec<u32> {
    let n = usize::from(u16le(exe, 0x06));
    let tab = usize::from(u16le(exe, 0x18));
    let mut out = Vec::with_capacity(n);
    for i in 0..n {
        let ofs = u32::from(u16le(exe, tab + i * 4));
        let seg = u32::from(u16le(exe, tab + i * 4 + 2));
        out.push(seg * 16 + ofs);
    }
    out.sort_unstable();
    out
}

fn u16le(b: &[u8], o: usize) -> u16 {
    u16::from_le_bytes([b[o], b[o + 1]])
}
