//! Backend-neutral device-input observation and confirmed-state semantics.
//!
//! The public contract exposes semantic device observations and one deterministic
//! confirmed-state reducer. Platform acquisition, product actions, UI interaction,
//! text/IME, ECS hosting, and product policy remain outside this crate.
//!
//! # Getting started
//!
//! The normal public workflow is: create an `InputContext`, construct backend-neutral
//! observations, admit an `InputObservationGroup` through `InputState::admit`, then
//! query confirmed state. The repository's `examples/basic_state.rs` is the canonical
//! executable walkthrough.
//!

mod contact;
mod continuity;
mod digital;
mod evidence;
mod identity;
mod keyboard;
mod measurement;
mod observation;
mod pointer;
mod state;
mod tablet;
mod validation;

pub use contact::{ContactInput, ContactPhase};
pub use continuity::ContinuityLoss;
pub use digital::DigitalState;
pub use evidence::{DeliveryRole, EvidenceStatus, ObservationOrigin, SourceTime, SourceTimeUnit};
pub use identity::{ContactId, InputContext, InputDeviceId, InputSourceId, ToolId};
pub use keyboard::{
    KeyLocation, KeyboardInput, LogicalKey, NativeLogicalKey, NativePhysicalKeyCode,
    PhysicalKeyIdentity,
};
pub use measurement::{
    AnalogMeasurement, CoordinateSpace, MeasurementDomain, Point2, RelativeMotionUnit, Vector2,
};
pub use observation::{InputError, InputObservation, InputObservationGroup};
pub use pointer::{
    PointerButton, PointerButtonInput, ScrollDelta, ScrollDomain, ScrollInput, ScrollPhase,
};
pub use state::InputState;
pub use tablet::{
    ContactPresence, InputToolKind, PhysicalTabletControls, StylusTilt, TabletCapabilities,
    TabletObservation,
};
