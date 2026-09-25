use crate::{
    identity::ContactId,
    measurement::{AnalogMeasurement, Point2},
};

/// Lifecycle phase for one touch/contact observation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContactPhase {
    /// The contact became active.
    Begin,
    /// The active contact changed while remaining active.
    Update,
    /// The contact ended normally.
    End,
    /// The contact lifetime was cancelled.
    Cancel,
}

/// Backend-neutral touch/contact sample.
#[derive(Debug, Clone, PartialEq)]
pub struct ContactInput {
    /// Runtime/session-scoped contact identity.
    pub contact: ContactId,
    /// Lifecycle phase for this sample.
    pub phase: ContactPhase,
    /// Contact position in its explicit coordinate space.
    pub position: Point2,
    /// Optional measured pressure; absence is distinct from measured zero.
    pub pressure: Option<AnalogMeasurement>,
    /// Optional altitude angle in radians when supplied by the source.
    pub altitude_angle_radians: Option<f32>,
}
