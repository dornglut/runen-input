/// Runtime/session-scoped identity for one input source.
///
/// This is not persistent hardware identity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct InputSourceId(u64);

impl InputSourceId {
    /// Creates a runtime/session-scoped source identity.
    pub const fn new(raw: u64) -> Self {
        Self(raw)
    }

    /// Returns the source-local numeric identity.
    pub const fn raw(self) -> u64 {
        self.0
    }
}

/// Runtime/session-scoped identity for one input device.
///
/// This is not persistent hardware identity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct InputDeviceId(u64);

impl InputDeviceId {
    /// Creates a runtime/session-scoped device identity.
    pub const fn new(raw: u64) -> Self {
        Self(raw)
    }

    /// Returns the device-local numeric identity.
    pub const fn raw(self) -> u64 {
        self.0
    }
}

/// Source/device scope for an admitted observation group.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct InputContext {
    /// Source that produced the observation group.
    pub source: InputSourceId,
    /// Optional device identity within the source.
    pub device: Option<InputDeviceId>,
}

impl InputContext {
    /// Creates an input context from its source and optional device.
    pub const fn new(source: InputSourceId, device: Option<InputDeviceId>) -> Self {
        Self { source, device }
    }
}

/// Runtime/session-scoped identity for one tablet/stylus tool.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ToolId(u64);

impl ToolId {
    /// Creates a runtime/session-scoped tool identity.
    pub const fn new(raw: u64) -> Self {
        Self(raw)
    }

    /// Returns the tool-local numeric identity.
    pub const fn raw(self) -> u64 {
        self.0
    }
}

/// Runtime/session-scoped identity for one contact lifetime.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ContactId(u64);

impl ContactId {
    /// Creates a runtime/session-scoped contact identity.
    pub const fn new(raw: u64) -> Self {
        Self(raw)
    }

    /// Returns the contact-local numeric identity.
    pub const fn raw(self) -> u64 {
        self.0
    }
}
