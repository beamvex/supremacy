/// Heap segment formula from the loaders (`0x320C6`-`0x320D5`):
/// `(0x9CC4 - 0x7D39) >> 4`, round up one paragraph, `+ 0x3829`
/// (the program's `dest_len`). Evaluates to `0x3A22` — the paragraph right
/// after the loaded image where the `.BIN` data lands.
#[must_use]
pub fn heap_segment() -> u16 {
    ((0x9CC4 - 0x7D39) >> 4) + 1 + 0x3829
}
