/// Scoped evidence that previously confirmed input continuity can no longer be trusted.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContinuityLoss {
    /// Invalidate confirmed state associated with the entire source.
    Source,
    /// Invalidate confirmed state for the device named by the enclosing input context.
    Device,
}
