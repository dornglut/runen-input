use crate::digital::DigitalState;

/// Backend-neutral physical pointer-button identity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PointerButton {
    /// Physical left button.
    Left,
    /// Physical right button.
    Right,
    /// Physical middle button.
    Middle,
    /// Physical back button.
    Back,
    /// Physical forward button.
    Forward,
    /// Other source-provided button identity.
    Other(u16),
}

/// One pointer-button state observation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PointerButtonInput {
    /// Physical button identity.
    pub button: PointerButton,
    /// Observed pressed/released state.
    pub state: DigitalState,
}

/// Measurement domain of scroll displacement.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScrollDomain {
    /// Scroll units are present but not otherwise established.
    Unspecified,
    /// Logical line-step units.
    Lines,
    /// Window/target physical pixels.
    WindowPhysicalPixels,
}

/// Optional lifecycle phase supplied for a scroll gesture.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScrollPhase {
    /// Gesture began.
    Begin,
    /// Gesture updated.
    Update,
    /// Gesture ended normally.
    End,
    /// Gesture was cancelled.
    Cancel,
}

/// Two-dimensional scroll displacement with independently optional axes.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ScrollDelta {
    /// Horizontal displacement when measured; None means absent, not measured zero.
    pub horizontal: Option<f32>,
    /// Vertical displacement when measured; None means absent, not measured zero.
    pub vertical: Option<f32>,
}

impl ScrollDelta {
    /// Creates a scroll delta with only a measured vertical component.
    pub const fn vertical_only(vertical: f32) -> Self {
        Self {
            horizontal: None,
            vertical: Some(vertical),
        }
    }

    /// Creates a scroll delta with both axes measured.
    pub const fn two_dimensional(horizontal: f32, vertical: f32) -> Self {
        Self {
            horizontal: Some(horizontal),
            vertical: Some(vertical),
        }
    }
}

/// Backend-neutral scroll observation.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ScrollInput {
    /// Scroll displacement.
    pub delta: ScrollDelta,
    /// Domain/unit of the displacement.
    pub domain: ScrollDomain,
    /// Optional source-provided gesture phase.
    pub phase: Option<ScrollPhase>,
}
