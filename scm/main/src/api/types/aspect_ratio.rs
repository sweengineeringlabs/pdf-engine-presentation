/// Slide aspect ratio.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum AspectRatio {
    /// Widescreen 16:9 slides.
    Widescreen16x9,
    /// Traditional 4:3 slides.
    Standard4x3,
}
