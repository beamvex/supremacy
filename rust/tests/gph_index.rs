//! `.GPH` parsing against the shipped index files and the verified manifest.

mod common;

use common::{game_dir, read};
use supremacy::gph::{fix_up, parse_index};

#[test]
fn mcg_index_has_356_records() {
    let recs = parse_index(&read(&game_dir().join("MCG.GPH")));
    assert_eq!(recs.len(), 356);
}

#[test]
fn record_zero_fields_match_manifest() {
    let r = parse_index(&read(&game_dir().join("MCG.GPH")))[0];
    assert_eq!(
        (r.x, r.y, r.w, r.h, r.ofs_para, r.mode_flag),
        (0, 6, 320, 192, 0, 0)
    );
}

#[test]
fn fix_up_adds_heap_segment_in_place() {
    let mut recs = parse_index(&read(&game_dir().join("MCG.GPH")));
    fix_up(&mut recs, 0x3A22);
    assert_eq!(recs[0].ofs_para, 0x3A22);
    assert_eq!(recs[1].ofs_para, 0x3A22 + 664);
}

#[test]
fn xor_flag_recognised() {
    let recs = parse_index(&read(&game_dir().join("CGA.GPH")));
    assert!(!recs[0].is_xor());
    assert!(recs[1].is_xor());
}
