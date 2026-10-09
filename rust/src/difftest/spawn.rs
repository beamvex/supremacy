//! Locating and driving the DOSBox-X process — headless via SDL's
//! dummy video/audio drivers, waited on the `DONE.BIN` marker then
//! killed (the game may sit in its loop after the last dump).

use std::path::{Path, PathBuf};
use std::process::{Child, Command};
use std::time::{Duration, Instant};

use super::dump::DONE_NAME;

/// Everything that can go wrong launching/waiting on DOSBox-X.
#[derive(Debug)]
pub enum DosError {
    /// No `dosbox-x` binary found (`DOSBOX_X` env, `PATH`, common spots).
    NotFound,
    /// `spawn`/`wait` failed.
    Io(std::io::Error),
    /// `DONE.BIN` never appeared inside the timeout.
    Timeout,
}

impl std::fmt::Display for DosError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotFound => write!(f, "dosbox-x not found (set DOSBOX_X)"),
            Self::Io(e) => write!(f, "dosbox-x: {e}"),
            Self::Timeout => write!(f, "timed out waiting for DONE.BIN"),
        }
    }
}

impl std::error::Error for DosError {}

/// Find a `dosbox-x` binary: `DOSBOX_X` env, then `PATH`, then a few
/// usual install locations (Homebrew cask, `/Applications`).
#[must_use]
pub fn find_dosbox() -> Option<PathBuf> {
    if let Some(p) = std::env::var_os("DOSBOX_X").map(PathBuf::from) {
        if p.is_file() {
            return Some(p);
        }
    }
    for cand in [
        "/Applications/dosbox-x.app/Contents/MacOS/dosbox-x",
        "/opt/homebrew/bin/dosbox-x",
        "/tmp/dosboxx/dosbox-x/dosbox-x.app/Contents/MacOS/dosbox-x",
    ] {
        let p = PathBuf::from(cand);
        if p.is_file() {
            return Some(p);
        }
    }
    which("dosbox-x")
}

fn which(name: &str) -> Option<PathBuf> {
    let path = std::env::var_os("PATH")?;
    std::env::split_paths(&path)
        .map(|d| d.join(name))
        .find(|p| p.is_file())
}

/// Spawn DOSBox-X with `conf`, wait for `DONE.BIN` in `out` (or
/// `timeout`), then kill the process. Headless: `SDL_VIDEODRIVER`/
/// `SDL_AUDIODRIVER=dummy`.
///
/// # Errors
/// `Io` on spawn/wait failure, `Timeout` when the marker never appears.
pub fn run_dos(bin: &Path, conf: &Path, out: &Path, timeout: Duration) -> Result<(), DosError> {
    let mut child = spawn(bin, conf).map_err(DosError::Io)?;
    let deadline = Instant::now() + timeout;
    let done = out.join(DONE_NAME);
    while Instant::now() < deadline {
        if done.is_file() {
            break;
        }
        if let Ok(Some(_)) = child.try_wait() {
            break;
        }
        std::thread::sleep(Duration::from_millis(200));
    }
    let _ = child.kill();
    let _ = child.wait();
    if done.is_file() {
        Ok(())
    } else {
        Err(DosError::Timeout)
    }
}

fn spawn(bin: &Path, conf: &Path) -> std::io::Result<Child> {
    Command::new(bin)
        .args(["-conf"])
        .arg(conf)
        .args(["-nomenu", "-silent"])
        .env("SDL_VIDEODRIVER", "dummy")
        .env("SDL_AUDIODRIVER", "dummy")
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn()
}
