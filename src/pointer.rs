use crate::digital::DigitalState;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PointerButton {
    Left,
    Right,
    Middle,
    Back,
    Forward,
    Other(u16),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PointerButtonInput {
    pub button: PointerButton,
    pub state: DigitalState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScrollDomain {
    Unspecified,
    Lines,
    WindowPhysicalPixels,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScrollPhase {
    Begin,
    Update,
    End,
    Cancel,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ScrollDelta {
    pub horizontal: Option<f32>,
    pub vertical: Option<f32>,
}

impl ScrollDelta {
    pub const fn vertical_only(vertical: f32) -> Self {
        Self {
            horizontal: None,
            vertical: Some(vertical),
        }
    }

    pub const fn two_dimensional(horizontal: f32, vertical: f32) -> Self {
        Self {
            horizontal: Some(horizontal),
            vertical: Some(vertical),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ScrollInput {
    pub delta: ScrollDelta,
    pub domain: ScrollDomain,
    pub phase: Option<ScrollPhase>,
}
