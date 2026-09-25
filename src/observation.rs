use std::fmt;

use crate::{
    contact::ContactInput,
    continuity::ContinuityLoss,
    identity::InputContext,
    keyboard::KeyboardInput,
    measurement::{Point2, RelativeMotionUnit, Vector2},
    pointer::{PointerButtonInput, ScrollInput},
    tablet::TabletObservation,
};

/// One backend-neutral device-input observation.
#[derive(Debug, Clone, PartialEq)]
pub enum InputObservation {
    /// Keyboard observation.
    Keyboard(KeyboardInput),
    /// Pointer-button observation.
    PointerButton(PointerButtonInput),
    /// Absolute pointer/cursor position for the source.
    AbsolutePointerPosition {
        /// Observed absolute position.
        position: Point2,
    },
    /// Relative motion independent from absolute pointer position.
    RelativeMotion {
        /// Relative motion delta.
        delta: Vector2,
        /// Unit domain for the relative motion.
        unit: RelativeMotionUnit,
    },
    /// Two-dimensional scroll observation.
    Scroll(ScrollInput),
    /// Touch/contact observation.
    Contact(ContactInput),
    /// Tablet/stylus observation.
    Tablet(TabletObservation),
    /// Scoped continuity-loss evidence.
    ContinuityLoss(ContinuityLoss),
}

/// Atomic logical source update admitted as one validation/reduction unit.
#[derive(Debug, Clone, PartialEq)]
pub struct InputObservationGroup {
    /// Source/device scope shared by all observations in the group.
    pub context: InputContext,
    /// Chronologically compatible observations belonging to this logical source update.
    pub observations: Vec<InputObservation>,
}

impl InputObservationGroup {
    /// Creates an observation group from one context and observation sequence.
    pub fn new(context: InputContext, observations: Vec<InputObservation>) -> Self {
        Self {
            context,
            observations,
        }
    }

    /// Creates an observation group containing exactly one observation.
    pub fn single(context: InputContext, observation: InputObservation) -> Self {
        Self::new(context, vec![observation])
    }
}

/// Rejection returned before an invalid observation group mutates confirmed state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputError {
    /// An observation contains a non-finite numeric value.
    NonFiniteObservation,
    /// An analog value lies outside its declared measurement domain.
    InvalidMeasurement,
    /// Concrete tablet evidence contradicts explicit unsupported capability knowledge.
    UnsupportedTabletCapabilityEvidence,
    /// Tablet source-time context does not match the enclosing observation-group context.
    SourceTimeContextMismatch,
    /// Source-time unit metadata is invalid, such as zero-frequency native ticks.
    InvalidSourceTimeUnit,
    /// Device-scoped continuity loss was supplied without a device-bearing context.
    DeviceContinuityLossRequiresDevice,
}

impl fmt::Display for InputError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::NonFiniteObservation => "input observation contains a non-finite numeric value",
            Self::InvalidMeasurement => {
                "input observation contains a value outside its measurement domain"
            }
            Self::UnsupportedTabletCapabilityEvidence => {
                "tablet observation contradicts explicit unsupported capability knowledge"
            }
            Self::SourceTimeContextMismatch => {
                "tablet source-time context does not match the observation-group context"
            }
            Self::InvalidSourceTimeUnit => "input source-time unit metadata is invalid",
            Self::DeviceContinuityLossRequiresDevice => {
                "device continuity loss requires a device-bearing input context"
            }
        })
    }
}

impl std::error::Error for InputError {}
