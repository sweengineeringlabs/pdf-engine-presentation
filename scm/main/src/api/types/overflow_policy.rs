/// Policy used when estimated slide content exceeds the fixed canvas.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum OverflowPolicy {
    /// Reject the deck with an actionable validation error.
    Reject,
    /// Keep the fixed canvas and clip content at its boundary.
    Clip,
}
