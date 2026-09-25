#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CoordinateSpace {
    UnspecifiedTargetUnits,
    WindowPhysicalPixels,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RelativeMotionUnit {
    BackendDeviceUnits,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MeasurementDomain {
    UnspecifiedScalar,
    NormalizedUnitInterval,
    SignedNormalizedUnitInterval,
    CalibratedForce { max_possible_force: f32 },
    Bounded { min: f32, max: f32 },
    Degrees { min: f32, max: f32 },
}

impl MeasurementDomain {
    pub fn calibrated_force(max_possible_force: f64) -> Self {
        Self::CalibratedForce {
            max_possible_force: max_possible_force as f32,
        }
    }

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

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point2 {
    pub x: f32,
    pub y: f32,
    pub space: CoordinateSpace,
}

impl Point2 {
    pub const fn new(x: f32, y: f32, space: CoordinateSpace) -> Self {
        Self { x, y, space }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

impl Vector2 {
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AnalogMeasurement {
    pub value: f32,
    pub domain: MeasurementDomain,
}

impl AnalogMeasurement {
    pub const fn new(value: f32, domain: MeasurementDomain) -> Self {
        Self { value, domain }
    }
}
