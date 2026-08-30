use crate::api::types::slide_element::SlideElement;

/// Parsed presentation slide.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Slide {
    /// Elements in source order.
    pub elements: Vec<SlideElement>,
}
