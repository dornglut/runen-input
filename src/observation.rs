use crate::{
    contact::ContactInput,
    continuity::ContinuityLoss,
    identity::InputContext,
    keyboard::KeyboardInput,
    measurement::{Point2, RelativeMotionUnit, Vector2},
    pointer::{PointerButtonInput, ScrollInput},
    tablet::TabletObservation,
};

#[derive(Debug, Clone, PartialEq)]
pub enum InputObservation {
    Keyboard(KeyboardInput),
    PointerButton(PointerButtonInput),
    AbsolutePointerPosition {
        position: Point2,
    },
    RelativeMotion {
        delta: Vector2,
        unit: RelativeMotionUnit,
    },
    Scroll(ScrollInput),
    Contact(ContactInput),
    Tablet(TabletObservation),
    ContinuityLost(ContinuityLoss),
}

#[derive(Debug, Clone, PartialEq)]
pub struct InputObservationGroup {
    pub context: InputContext,
    pub observations: Vec<InputObservation>,
}

impl InputObservationGroup {
    pub fn new(context: InputContext, observations: Vec<InputObservation>) -> Self {
        Self {
            context,
            observations,
        }
    }

    pub fn single(context: InputContext, observation: InputObservation) -> Self {
        Self::new(context, vec![observation])
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputError {
    NonFiniteObservation,
    InvalidMeasurement,
    SourceTimeContextMismatch,
    DeviceContinuityRequiresDevice,
}
