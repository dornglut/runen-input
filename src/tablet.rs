use crate::{
    contact::ContactPhase,
    evidence::{DeliveryRole, EvidenceStatus, ObservationOrigin, SourceTime},
    identity::{ContactId, ToolId},
    measurement::{AnalogMeasurement, Point2, Vector2},
};

/// Backend-neutral tablet/stylus tool category.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InputToolKind {
    /// Mouse-like tool.
    Mouse,
    /// Pen/stylus tool.
    Pen,
    /// Brush tool.
    Brush,
    /// Marker tool.
    Marker,
    /// Airbrush tool.
    Airbrush,
    /// Eraser tool.
    Eraser,
    /// Finger/direct-touch tool.
    Finger,
    /// Tool kind is not established.
    Unknown,
}

/// Knowledge about whether a tablet source can establish one capability.
///
/// Unknown means support has not been established. It is distinct from
/// Unsupported: an observation may carry concrete evidence while capability
/// metadata remains unknown, but explicit Unsupported knowledge cannot
/// coexist with evidence that requires the capability.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub enum CapabilityKnowledge {
    /// Support has not been established.
    #[default]
    Unknown,
    /// Support is explicitly established.
    Supported,
    /// Lack of support is explicitly established.
    Unsupported,
}

/// Independently established capability knowledge for one tablet observation.
///
/// Capability knowledge describes what the source can establish. It does not
/// require every sample to contain a value for every supported capability.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct TabletCapabilities {
    /// Pressure capability knowledge.
    pub pressure: CapabilityKnowledge,
    /// Tilt capability knowledge.
    pub tilt: CapabilityKnowledge,
    /// Twist capability knowledge.
    pub twist: CapabilityKnowledge,
    /// Tangential-pressure capability knowledge.
    pub tangential_pressure: CapabilityKnowledge,
    /// Hover/proximity capability knowledge.
    pub hover: CapabilityKnowledge,
    /// Eraser capability knowledge.
    pub eraser: CapabilityKnowledge,
    /// Barrel-control capability knowledge.
    pub barrel_controls: CapabilityKnowledge,
    /// Historical/coalesced sample capability knowledge.
    pub historical_samples: CapabilityKnowledge,
    /// Predicted-sample capability knowledge.
    pub predicted_samples: CapabilityKnowledge,
}

/// Tablet/stylus proximity/contact state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContactPresence {
    /// Tool is in range but not contacting the surface.
    Hover,
    /// Tool is contacting the surface.
    Contact,
    /// Tool is out of range.
    OutOfRange,
}

/// Physical tablet/stylus controls active for one observation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct PhysicalTabletControls {
    /// Eraser control/state is active.
    pub eraser: bool,
    /// Primary barrel control is active.
    pub barrel_primary: bool,
    /// Secondary barrel control is active.
    pub barrel_secondary: bool,
}

/// Stylus tilt around two axes, expressed in degrees.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct StylusTilt {
    /// X-axis tilt in degrees.
    pub x_degrees: f32,
    /// Y-axis tilt in degrees.
    pub y_degrees: f32,
}

impl StylusTilt {
    /// Creates a stylus tilt measurement in degrees.
    pub const fn new(x_degrees: f32, y_degrees: f32) -> Self {
        Self {
            x_degrees,
            y_degrees,
        }
    }
}

/// Backend-neutral tablet/stylus observation.
#[derive(Debug, Clone, PartialEq)]
pub struct TabletObservation {
    /// Contact lifetime identity.
    pub contact: ContactId,
    /// Optional tool identity when established.
    pub tool: Option<ToolId>,
    /// Tool category.
    pub tool_kind: InputToolKind,
    /// Contact lifecycle phase.
    pub phase: ContactPhase,
    /// Proximity/contact state.
    pub presence: ContactPresence,
    /// Absolute position in an explicit coordinate space.
    pub position: Point2,
    /// Source-provided movement delta.
    pub delta: Vector2,
    /// Optional pressure measurement.
    pub pressure: Option<AnalogMeasurement>,
    /// Optional tangential-pressure measurement.
    pub tangential_pressure: Option<AnalogMeasurement>,
    /// Optional stylus tilt measurement.
    pub tilt: Option<StylusTilt>,
    /// Optional twist measurement.
    pub twist: Option<AnalogMeasurement>,
    /// Physical tool controls active for this observation.
    pub controls: PhysicalTabletControls,
    /// Capability knowledge supplied independently from this sample's values.
    pub capabilities: TabletCapabilities,
    /// Optional source-provided timestamp.
    pub source_time: Option<SourceTime>,
    /// Certainty class for this observation.
    pub evidence: EvidenceStatus,
    /// Current versus historical/coalesced delivery role.
    pub delivery: DeliveryRole,
    /// Source-report versus reconciliation provenance.
    pub origin: ObservationOrigin,
}
