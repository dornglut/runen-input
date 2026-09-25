use crate::{
    contact::ContactPhase,
    evidence::{DeliveryRole, EvidenceStatus, ObservationOrigin, SourceTime},
    identity::{ContactId, ToolId},
    measurement::{AnalogMeasurement, Point2, Vector2},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InputToolKind {
    Mouse,
    Pen,
    Brush,
    Marker,
    Airbrush,
    Eraser,
    Finger,
    Unknown,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct TabletCapabilities {
    pub pressure: bool,
    pub tilt: bool,
    pub twist: bool,
    pub tangential_pressure: bool,
    pub hover: bool,
    pub eraser: bool,
    pub barrel_controls: bool,
    pub historical_samples: bool,
    pub predicted_samples: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContactPresence {
    Hover,
    Contact,
    OutOfRange,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct PhysicalTabletControls {
    pub eraser: bool,
    pub barrel_primary: bool,
    pub barrel_secondary: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct StylusTilt {
    pub x_degrees: f32,
    pub y_degrees: f32,
}

impl StylusTilt {
    pub const fn new(x_degrees: f32, y_degrees: f32) -> Self {
        Self {
            x_degrees,
            y_degrees,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TabletObservation {
    pub contact: ContactId,
    pub tool: Option<ToolId>,
    pub tool_kind: InputToolKind,
    pub phase: ContactPhase,
    pub presence: ContactPresence,
    pub position: Point2,
    pub delta: Vector2,
    pub pressure: Option<AnalogMeasurement>,
    pub tangential_pressure: Option<AnalogMeasurement>,
    pub tilt: Option<StylusTilt>,
    pub twist: Option<AnalogMeasurement>,
    pub controls: PhysicalTabletControls,
    pub capabilities: TabletCapabilities,
    pub source_time: Option<SourceTime>,
    pub evidence: EvidenceStatus,
    pub delivery: DeliveryRole,
    pub origin: ObservationOrigin,
}

