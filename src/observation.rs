use crate::{
    contact::ContactPhase,
    identity::{ContactId, InputContext},
    keyboard::KeyboardInput,
    measurement::{AnalogMeasurement, Point2, RelativeMotionUnit, Vector2},
    pointer::{PointerButtonInput, ScrollDelta, ScrollDomain, ScrollPhase},
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
    Scroll {
        delta: ScrollDelta,
        domain: ScrollDomain,
        phase: Option<ScrollPhase>,
    },
    Contact {
        contact: ContactId,
        phase: ContactPhase,
        position: Point2,
        pressure: Option<AnalogMeasurement>,
        altitude_angle_radians: Option<f32>,
    },
    Tablet(TabletObservation),
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
}

