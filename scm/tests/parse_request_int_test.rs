#![allow(missing_docs)]

use pdf_engine_presentation::{AspectRatio, ParseRequest};

/// @covers: ParseRequest
#[test]
fn test_parse_request_holds_source_and_default_aspect_ratio() {
    let request = ParseRequest {
        source: "# Slide".to_string(),
        default_aspect_ratio: AspectRatio::Widescreen16x9,
    };
    assert_eq!(request.source, "# Slide");
    assert_eq!(request.default_aspect_ratio, AspectRatio::Widescreen16x9);
}
