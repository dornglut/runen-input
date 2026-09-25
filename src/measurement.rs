/// Coordinate space attached to an absolute position.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CoordinateSpace {
    /// Target-local units whose precise scale is owned by the adapter/consumer.
    UnspecifiedTargetUnits,
    /// Target/window physical pixel coordinates.
    WindowPhysicalPixels,
}

/// Unit domain for relative motion.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RelativeMotionUnit {
    /// Backend/device-native relative-motion units.
    BackendDeviceUnits,
}

/// Domain and range semantics for one analog measurement.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MeasurementDomain {
    /// Finite scalar without a stronger normalized range contract.
    UnspecifiedScalar,
    /// Normalized inclusive range from 0 to 1.
    NormalizedUnitInterval,
    /// Signed normalized inclusive range from -1 to 1.
    SignedNormalizedUnitInterval,
    /// Calibrated force from zero through the supplied maximum.
    CalibratedForce {
        /// Inclusive maximum possible force.
        max_possible_force: f32,
    },
    /// Explicit inclusive scalar bounds.
    Bounded {
        /// Inclusive minimum.
        min: f32,
        /// Inclusive maximum.
        max: f32,
    },
    /// Explicit inclusive degree bounds.
    Degrees {
        /// Inclusive minimum angle in degrees.
        min: f32,
        /// Inclusive maximum angle in degrees.
        max: f32,
    },
}

impl MeasurementDomain {
    /// Creates a calibrated-force domain with the supplied maximum.
    pub fn calibrated_force(max_possible_force: f64) -> Self {
        Self::CalibratedForce {
            max_possible_force: max_possible_force as f32,
        }
    }

    /// Returns the calibrated-force maximum when this is a calibrated-force domain.
    pub fn max_possible_force(self) -> Option<f32> {
        match self {
            Self::CalibratedForce { max_possible_force } => Some(max_possible_force),
            Self::UnspecifiedScalar
            | Self::NormalizedUnitInterval
            | Self::SignedNormalizedUnitInterval
            | Self::Bounded { .. }
            | Self::Degrees { .. } => None,
        }
    }

    pub(crate) fn accepts(self, value: f32) -> bool {
        if !value.is_finite() {
            return false;
        }
        match self {
            Self::UnspecifiedScalar => true,
            Self::NormalizedUnitInterval => (0.0..=1.0).contains(&value),
            Self::SignedNormalizedUnitInterval => (-1.0..=1.0).contains(&value),
            Self::CalibratedForce { max_possible_force } => {
                max_possible_force.is_finite()
                    && max_possible_force >= 0.0
                    && (0.0..=max_possible_force).contains(&value)
            }
            Self::Bounded { min, max } | Self::Degrees { min, max } => {
                min.is_finite() && max.is_finite() && min <= max && (min..=max).contains(&value)
            }
        }
    }
}

/// Two-dimensional absolute point with an explicit coordinate space.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point2 {
    /// Horizontal coordinate.
    pub x: f32,
    /// Vertical coordinate.
    pub y: f32,
    /// Coordinate space for both components.
    pub space: CoordinateSpace,
}

impl Point2 {
    /// Creates a point in the supplied coordinate space.
    pub const fn new(x: f32, y: f32, space: CoordinateSpace) -> Self {
        Self { x, y, space }
    }
}

/// Two-dimensional vector without an implied absolute origin.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector2 {
    /// Horizontal component.
    pub x: f32,
    /// Vertical component.
    pub y: f32,
}

impl Vector2 {
    /// Creates a two-dimensional vector.
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

/// Analog sample paired with its measurement domain.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AnalogMeasurement {
    /// Measured scalar value.
    pub value: f32,
    /// Domain that defines valid values and interpretation.
    pub domain: MeasurementDomain,
}

impl AnalogMeasurement {
    /// Creates an analog measurement in the supplied domain.
    pub const fn new(value: f32, domain: MeasurementDomain) -> Self {
        Self { value, domain }
    }
}
