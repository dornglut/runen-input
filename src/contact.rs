use crate::{
    identity::ContactId,
    measurement::{AnalogMeasurement, Point2},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContactPhase {
    Begin,
    Update,
    End,
    Cancel,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ContactInput {
    pub contact: ContactId,
    pub phase: ContactPhase,
    pub position: Point2,
    pub pressure: Option<AnalogMeasurement>,
    pub altitude_angle_radians: Option<f32>,
}
