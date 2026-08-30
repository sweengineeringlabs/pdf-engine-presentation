#![allow(missing_docs)]

use pdf_engine_presentation::AspectRatio;

/// @covers: AspectRatio
#[test]
fn test_aspect_ratio_variants_are_distinct() {
    assert_ne!(AspectRatio::Widescreen16x9, AspectRatio::Standard4x3);
    let original = AspectRatio::Widescreen16x9;
    let copied = original;
    assert_eq!(original, copied);
}
