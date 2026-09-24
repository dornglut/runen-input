//! Backend-neutral device-input observation and confirmed-state semantics.
//!
//! The public contract exposes semantic device observations and one deterministic
//! confirmed-state reducer. Platform acquisition, product actions, UI interaction,
//! text/IME, ECS hosting, and product policy remain outside this crate.

mod input;

pub use input::*;
