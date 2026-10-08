/// The four `int 21h` wrappers the whole binary funnels through.
///
/// Call pattern is always `find_first` → `open` → `read` (in `0xFFF0`-byte
/// chunks by the `.BIN` loader) → `close`. Handles start at 5 like a real
/// DOS process (0–4 are the standard devices). `None`/`false` mirror the
/// carry-flag error return the wrappers test.
pub trait DosFiles {
    /// `int 21/AH=4E` — does `name` exist? (findfirst; wildcards unused).
    fn find_first(&mut self, name: &str) -> bool;
    /// `int 21/AH=3D AL=0` — open read-only, returning a handle.
    fn open(&mut self, name: &str) -> Option<u16>;
    /// `int 21/AH=3F` — read into `buf`, returning the byte count
    /// (`0` = EOF, the loader's loop exit).
    fn read(&mut self, handle: u16, buf: &mut [u8]) -> usize;
    /// `int 21/AH=3C CX=0` — create/truncate `name`, returning a handle.
    fn create(&mut self, name: &str) -> Option<u16>;
    /// `int 21/AH=3D AL=1` — open write-only, returning a handle.
    fn open_write(&mut self, name: &str) -> Option<u16>;
    /// `int 21/AH=40` — write `buf`, returning the byte count.
    fn write(&mut self, handle: u16, buf: &[u8]) -> usize;
    /// `int 21/AH=3E` — close the handle.
    fn close(&mut self, handle: u16);
}
