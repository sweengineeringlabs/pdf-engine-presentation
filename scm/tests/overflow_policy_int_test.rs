#![allow(missing_docs)]

use pdf_engine_presentation::OverflowPolicy;

/// @covers: OverflowPolicy
#[test]
fn test_overflow_policy_variants_are_distinct() {
    assert_ne!(OverflowPolicy::Reject, OverflowPolicy::Clip);
    let original = OverflowPolicy::Reject;
    let copied = original;
    assert_eq!(original, copied);
}
