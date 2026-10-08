use super::DosFiles;
use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;

/// `DosFiles` over the host filesystem — a root directory plays the role of
/// the DOS current directory `C:\` (the `GAME/` asset dir in practice).
///
/// Filenames are matched the way FAT would: the literal name first, then
/// uppercased, so case-sensitive host filesystems find the shipped
/// `MCG.BIN`-style names either way.
pub struct HostFs {
    root: PathBuf,
    handles: HashMap<u16, File>,
    next: u16,
}

impl HostFs {
    /// Root the virtual `C:` at `dir`.
    #[must_use]
    pub fn new(root: PathBuf) -> Self {
        Self {
            root,
            handles: HashMap::new(),
            next: 5,
        }
    }

    fn open_file(&self, name: &str) -> Option<File> {
        File::open(self.root.join(name))
            .or_else(|_| File::open(self.root.join(name.to_uppercase())))
            .ok()
    }

    fn register(&mut self, f: File) -> u16 {
        let h = self.next;
        self.next = self.next.wrapping_add(1);
        self.handles.insert(h, f);
        h
    }
}

impl DosFiles for HostFs {
    fn find_first(&mut self, name: &str) -> bool {
        self.root.join(name).is_file() || self.root.join(name.to_uppercase()).is_file()
    }

    fn open(&mut self, name: &str) -> Option<u16> {
        let f = self.open_file(name)?;
        Some(self.register(f))
    }

    fn create(&mut self, name: &str) -> Option<u16> {
        let f = File::create(self.root.join(name)).ok()?;
        Some(self.register(f))
    }

    fn open_write(&mut self, name: &str) -> Option<u16> {
        let f = File::options()
            .write(true)
            .open(self.root.join(name))
            .ok()?;
        Some(self.register(f))
    }

    fn write(&mut self, handle: u16, buf: &[u8]) -> usize {
        self.handles
            .get_mut(&handle)
            .and_then(|f| f.write(buf).ok())
            .unwrap_or(0)
    }

    fn read(&mut self, handle: u16, buf: &mut [u8]) -> usize {
        self.handles
            .get_mut(&handle)
            .and_then(|f| f.read(buf).ok())
            .unwrap_or(0)
    }

    fn close(&mut self, handle: u16) {
        self.handles.remove(&handle);
    }
}
