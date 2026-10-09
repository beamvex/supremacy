//! Diffing a DOS dump against the port replay — per-region counts plus
//! the first few mismatched bytes.

/// One compared artefact.
pub struct Diff {
    /// Artefact name (`D01.BIN`, `P05.BIN`, …).
    pub name: String,
    /// Bytes present on both sides.
    pub len: usize,
    /// Mismatching byte count within `len`.
    pub diffs: usize,
    /// Up to 8 `(ofs, dos, port)` mismatches, in order.
    pub first: Vec<(usize, u8, u8)>,
    /// Side missing the file, if any (`"dos"`/`"port"`).
    pub missing: Option<&'static str>,
}

/// Compare one artefact pair (`port` may be empty = not emitted).
#[must_use]
pub fn compare(name: &str, dos: &[u8], port: &[u8]) -> Diff {
    let len = dos.len().min(port.len());
    let mut first = Vec::new();
    let mut diffs = 0usize;
    for i in 0..len {
        if dos[i] != port[i] {
            diffs += 1;
            if first.len() < 8 {
                first.push((i, dos[i], port[i]));
            }
        }
    }
    diffs += dos.len().abs_diff(port.len());
    Diff {
        name: name.to_owned(),
        len,
        diffs,
        first,
        missing: if port.is_empty() {
            Some("port")
        } else if dos.is_empty() {
            Some("dos")
        } else {
            None
        },
    }
}

/// The whole comparison for a run.
#[derive(Default)]
pub struct Report {
    /// One entry per compared artefact, script order.
    pub diffs: Vec<Diff>,
}

impl Report {
    /// Total mismatched bytes across artefacts.
    #[must_use]
    pub fn total(&self) -> usize {
        self.diffs.iter().map(|d| d.diffs).sum()
    }

    /// Artefacts that compared clean.
    #[must_use]
    pub fn clean(&self) -> usize {
        self.diffs.iter().filter(|d| d.diffs == 0).count()
    }

    /// Plain-text rendering (`report.txt` / stdout).
    #[must_use]
    pub fn render(&self) -> String {
        use std::fmt::Write;
        let mut s = format!(
            "difftest: {} artefacts, {} clean, {} differing bytes\n",
            self.diffs.len(),
            self.clean(),
            self.total()
        );
        for d in &self.diffs {
            let miss = d
                .missing
                .map(|m| format!(" missing:{m}"))
                .unwrap_or_default();
            let _ = writeln!(
                s,
                "  {:9} {:6}B  {:8} diffs{}",
                d.name, d.len, d.diffs, miss
            );
            for &(o, a, b) in &d.first {
                let _ = writeln!(s, "      +{o:05X}: dos {a:02X} port {b:02X}");
            }
        }
        s
    }
}
