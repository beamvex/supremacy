use super::DosFiles;
use std::collections::HashMap;

/// In-memory `DosFiles` — a name→bytes map with fake handles, for tests
/// and headless runs where no host filesystem should be touched.
/// Handles start at 5 like [`HostFs`][super::HostFs].
#[derive(Default)]
pub struct MemFs {
    /// The virtual `C:` directory.
    pub files: HashMap<String, Vec<u8>>,
    /// When set, `open`/`create`/`find_first` all fail — the DOS
    /// carry-flag error path.
    pub fail: bool,
    handles: HashMap<u16, (String, usize)>,
    next: u16,
}

impl MemFs {
    /// Fresh empty filesystem.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    fn register(&mut self, name: &str) -> u16 {
        let h = self.next;
        self.next = self.next.wrapping_add(1);
        self.handles.insert(h, (name.to_owned(), 0));
        h
    }
}

impl DosFiles for MemFs {
    fn find_first(&mut self, name: &str) -> bool {
        !self.fail && self.files.contains_key(name)
    }

    fn open(&mut self, name: &str) -> Option<u16> {
        (!self.fail && self.files.contains_key(name)).then(|| self.register(name))
    }

    fn create(&mut self, name: &str) -> Option<u16> {
        if self.fail {
            return None;
        }
        self.files.insert(name.to_owned(), Vec::new());
        Some(self.register(name))
    }

    fn open_write(&mut self, name: &str) -> Option<u16> {
        self.open(name)
    }

    fn write(&mut self, handle: u16, buf: &[u8]) -> usize {
        let Some((name, pos)) = self.handles.get_mut(&handle) else {
            return 0;
        };
        let f = self.files.entry(name.clone()).or_default();
        if f.len() < *pos + buf.len() {
            f.resize(*pos + buf.len(), 0);
        }
        f[*pos..*pos + buf.len()].copy_from_slice(buf);
        *pos += buf.len();
        buf.len()
    }

    fn read(&mut self, handle: u16, buf: &mut [u8]) -> usize {
        let Some((name, pos)) = self.handles.get_mut(&handle) else {
            return 0;
        };
        let n = self
            .files
            .get(name)
            .map_or(0, |f| f.len().saturating_sub(*pos).min(buf.len()));
        buf[..n].copy_from_slice(&self.files[name][*pos..*pos + n]);
        *pos += n;
        n
    }

    fn close(&mut self, handle: u16) {
        self.handles.remove(&handle);
    }
}
