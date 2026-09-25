use crate::{digital::DigitalState, evidence::ObservationOrigin};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyLocation {
    Standard,
    Left,
    Right,
    Numpad,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum NativePhysicalKeyCode {
    Unidentified,
    Android(u32),
    MacOs(u32),
    Windows(u32),
    Xkb(u32),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PhysicalKeyIdentity {
    Code(String),
    Native(NativePhysicalKeyCode),
}

impl PhysicalKeyIdentity {
    pub fn code(value: impl Into<String>) -> Self {
        Self::Code(value.into())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NativeLogicalKey {
    Unidentified,
    Android(u32),
    MacOs(u32),
    Windows(u32),
    Xkb(u32),
    Web(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LogicalKey {
    Named(String),
    Character(String),
    Native(NativeLogicalKey),
    Dead(Option<char>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeyboardInput {
    pub physical_key: PhysicalKeyIdentity,
    pub logical_key: LogicalKey,
    pub location: KeyLocation,
    pub state: DigitalState,
    pub repeat: bool,
    pub origin: ObservationOrigin,
}
