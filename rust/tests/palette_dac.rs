//! The master DAC table parses to the reference renderer's RGB values.

mod common;

use common::{read, root};
use supremacy::palette::{from_dac, DAC_LEN, DAC_OFS};

#[test]
fn dac_table_matches_reference() {
    let exe = read(&root().join("decompiled/GAME.unpacked.exe"));
    let pal = from_dac(&exe[DAC_OFS..DAC_OFS + DAC_LEN]);
    assert_eq!(pal.rgb(0), [0, 0, 0]);
    assert_eq!(pal.rgb(1), [242, 242, 242]);
    assert_eq!(pal.rgb(16), [210, 0, 0]);
    assert_eq!(pal.rgb(128), [218, 238, 255]);
    assert_eq!(pal.rgb(255), [255, 255, 255]);
}
