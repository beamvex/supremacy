//! Every `.GPH` record in every set: decoded length and FNV-1a hash must
//! match the verified Python decoder (`decompiled/tools/extract_png.py`).

mod common;

use common::{fnv, game_dir, manifest, read};
use std::collections::HashMap;
use supremacy::lzss::{decode, decode_planes};

#[test]
fn all_1703_records_match_reference() {
    let mut bins: HashMap<String, Vec<u8>> = HashMap::new();
    for row in manifest() {
        let bin = bins
            .entry(row.set.clone())
            .or_insert_with(|| read(&game_dir().join(format!("{}.BIN", row.set))));
        let dec = decode_row(bin.as_slice(), &row);
        assert_eq!(
            (dec.len(), fnv(&dec)),
            (row.decoded, row.hash),
            "{} idx {}",
            row.set,
            row.ofs_para
        );
    }
}

fn decode_row(bin: &[u8], row: &common::Row) -> Vec<u8> {
    let i = usize::from(row.ofs_para) * 16;
    if row.set.starts_with("EGA") {
        decode_planes(bin, i, 4).0
    } else {
        decode(&bin[i..])
    }
}
