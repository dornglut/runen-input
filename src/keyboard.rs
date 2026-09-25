use crate::{digital::DigitalState, evidence::ObservationOrigin};

/// Physical location of a keyboard key when known.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyLocation {
    /// Standard location without a left/right/numpad distinction.
    Standard,
    /// Left-side key location.
    Left,
    /// Right-side key location.
    Right,
    /// Numeric-keypad location.
    Numpad,
}

/// Backend-neutral preservation of a platform-native physical key identity.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum NativePhysicalKeyCode {
    /// Native physical identity exists but is otherwise unidentified.
    Unidentified,
    /// Android-native physical key code.
    Android(u32),
    /// macOS-native physical key code.
    MacOs(u32),
    /// Windows-native physical key code.
    Windows(u32),
    /// XKB-native physical key code.
    Xkb(u32),
}

/// Layout-independent physical key identity.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PhysicalKeyIdentity {
    /// Backend-neutral physical key code/token.
    Code(String),
    /// Preserved platform-native physical identity.
    Native(NativePhysicalKeyCode),
}

impl PhysicalKeyIdentity {
    /// Creates a backend-neutral physical key-code identity.
    pub fn code(value: impl Into<String>) -> Self {
        Self::Code(value.into())
    }
}

/// Backend-neutral preservation of a platform-native logical key.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NativeLogicalKey {
    /// Native logical identity exists but is otherwise unidentified.
    Unidentified,
    /// Android-native logical key value.
    Android(u32),
    /// macOS-native logical key value.
    MacOs(u32),
    /// Windows-native logical key value.
    Windows(u32),
    /// XKB-native logical key value.
    Xkb(u32),
    /// Web-native logical key value.
    Web(String),
}

/// Backend/platform interpretation of keyboard meaning.
///
/// Logical key meaning is distinct from physical held-state identity and from
/// committed text/IME.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LogicalKey {
    /// Named non-character key meaning.
    Named(String),
    /// Layout-derived character meaning; this is not committed text.
    Character(String),
    /// Preserved platform-native logical key meaning.
    Native(NativeLogicalKey),
    /// Dead-key meaning with an optional associated character.
    Dead(Option<char>),
}

/// One backend-neutral keyboard observation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeyboardInput {
    /// Layout-independent physical key identity.
    pub physical_key: PhysicalKeyIdentity,
    /// Backend/platform logical interpretation.
    pub logical_key: LogicalKey,
    /// Physical key location when established.
    pub location: KeyLocation,
    /// Observed pressed/released state.
    pub state: DigitalState,
    /// Whether this is repeat metadata on an already-held key lifetime.
    pub repeat: bool,
    /// Observation provenance, including reconciliation synthesis.
    pub origin: ObservationOrigin,
}
