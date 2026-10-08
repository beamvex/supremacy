//! End-to-end asset-set loading: fixed-up records + streams that decode to
//! the manifest's expected values.

mod common;

use common::{fnv, game_dir, manifest};
use supremacy::assets::{heap_segment, load_set};
use supremacy::lzss::decode;

#[test]
fn heap_segment_is_3a22() {
    assert_eq!(heap_segment(), 0x3A22);
}

#[test]
fn mcg_set_loads_and_decodes() {
    let set = load_set(&game_dir(), "MCG", heap_segment()).unwrap();
    assert_eq!(set.records.len(), 356);
    assert_eq!(set.records[0].ofs_para, heap_segment());
    let rows = manifest();
    let row = rows.iter().find(|r| r.set == "MCG").unwrap();
    let dec = decode(set.stream(0));
    assert_eq!((dec.len(), fnv(&dec)), (row.decoded, row.hash));
}

#[test]
fn streams_start_on_paragraph_boundaries() {
    let set = load_set(&game_dir(), "MCG", heap_segment()).unwrap();
    let raw = std::fs::read(game_dir().join("MCG.BIN")).unwrap();
    assert_eq!(set.stream(0)[..8], raw[..8]);
}
