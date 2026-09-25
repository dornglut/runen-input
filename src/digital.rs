/// Observed binary control state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DigitalState {
    /// The control is pressed or active.
    Pressed,
    /// The control is released or inactive.
    Released,
}
