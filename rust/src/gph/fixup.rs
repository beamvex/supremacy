use super::Record;

/// Add the heap segment to every record's `ofs_para` in place — the loop at
/// `0x320D8` (`mov bp,[si+0x160]` / `add bp,ax` / `mov [si+0x160],bp`),
/// turning file-relative paragraph offsets into absolute segments for the
/// renderer.
pub fn fix_up(records: &mut [Record], heap_seg: u16) {
    for r in records {
        r.ofs_para = r.ofs_para.wrapping_add(heap_seg);
    }
}
