#![allow(missing_docs)]

use pdf_engine_presentation::ValidationError;

/// @covers: ValidationError
#[test]
fn test_validation_error_holds_itemized_violations() {
    let error = ValidationError {
        violations: vec![
            "deck has no slides".to_string(),
            "title is empty".to_string(),
        ],
    };
    assert_eq!(error.violations.len(), 2);
    assert_eq!(error.violations[0], "deck has no slides");
}
