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

/// Knowledge about whether a tablet source can establish one capability.
///
/// `Unknown` means support has not been established. It is distinct from
/// `Unsupported`: an observation may carry concrete evidence while capability
/// metadata remains unknown, but explicit `Unsupported` knowledge cannot
/// coexist with evidence that requires the capability.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub enum CapabilityKnowledge {
    #[default]
    Unknown,
    Supported,
    Unsupported,
}

/// Independently established capability knowledge for one tablet observation.
///
/// Capability knowledge describes what the source can establish. It does not
/// require every sample to contain a value for every supported capability.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct TabletCapabilities {
    pub pressure: CapabilityKnowledge,
    pub tilt: CapabilityKnowledge,
    pub twist: CapabilityKnowledge,
    pub tangential_pressure: CapabilityKnowledge,
    pub hover: CapabilityKnowledge,
    pub eraser: CapabilityKnowledge,
    pub barrel_controls: CapabilityKnowledge,
    pub historical_samples: CapabilityKnowledge,
    pub predicted_samples: CapabilityKnowledge,
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
    /// Capability knowledge supplied independently from this sample's values.
    pub capabilities: TabletCapabilities,
    pub source_time: Option<SourceTime>,
    pub evidence: EvidenceStatus,
    pub delivery: DeliveryRole,
    pub origin: ObservationOrigin,
}
