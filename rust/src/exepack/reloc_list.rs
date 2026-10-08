use super::reloc_ofs::table_ofs;
use super::word::word;

/// Parse the packed relocation stream (`0x23B22`-`0x23B44`): 16 batches, one
/// per `0x1000` paragraphs, each a count word followed by that many `u16`
/// offsets. Returns the fixup sites as linear offsets into the unpacked
/// image (`dx * 16 + entry`; the `0xFFFF` special case at `0x23B46` is just
/// uncanonicalised `seg:ofs` arithmetic).
///
/// # Errors
/// None — a truncated table yields the entries decoded so far.
#[must_use]
pub fn reloc_list(block: &[u8]) -> Vec<u32> {
    let mut out = Vec::new();
    let mut si = table_ofs(block);
    for dx in (0u32..=0xF000).step_by(0x1000) {
        let Ok(n) = word(block, si) else { break };
        si += 2;
        for _ in 0..n {
            if let Ok(ofs) = word(block, si) {
                out.push(dx * 16 + u32::from(ofs));
                si += 2;
            }
        }
    }
    out
}
