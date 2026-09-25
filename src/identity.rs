#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct InputSourceId(u64);

impl InputSourceId {
    /// Creates a runtime/session-scoped source identity. This value is not persistent identity.
    pub const fn new(raw: u64) -> Self {
        Self(raw)
    }

    pub const fn raw(self) -> u64 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct InputDeviceId(u64);

impl InputDeviceId {
    /// Creates a runtime/session-scoped device identity. This value is not persistent identity.
    pub const fn new(raw: u64) -> Self {
        Self(raw)
    }

    pub const fn raw(self) -> u64 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct InputContext {
    pub source: InputSourceId,
    pub device: Option<InputDeviceId>,
}

impl InputContext {
    pub const fn new(source: InputSourceId, device: Option<InputDeviceId>) -> Self {
        Self { source, device }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ToolId(u64);

impl ToolId {
    pub const fn new(raw: u64) -> Self {
        Self(raw)
    }

    pub const fn raw(self) -> u64 {
        self.0
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ContactId(u64);

impl ContactId {
    pub const fn new(raw: u64) -> Self {
        Self(raw)
    }

    pub const fn raw(self) -> u64 {
        self.0
    }
}

