use crate::{
    evidence::SourceTimeUnit,
    measurement::{AnalogMeasurement, Point2},
    observation::{InputError, InputObservation, InputObservationGroup},
};

pub(crate) fn validate_group(group: &InputObservationGroup) -> Result<(), InputError> {
    if group
        .observations
        .iter()
        .any(|observation| !is_finite(observation))
    {
        return Err(InputError::NonFiniteObservation);
    }
    if group
        .observations
        .iter()
        .any(|observation| !has_valid_measurements(observation))
    {
        return Err(InputError::InvalidMeasurement);
    }
    for observation in &group.observations {
        if let InputObservation::Tablet(tablet) = observation
            && tablet.source_time.is_some_and(|time| {
                time.context != group.context
                    || matches!(
                        time.unit,
                        SourceTimeUnit::NativeTicks {
                            ticks_per_second: 0
                        }
                    )
            })
        {
            return Err(InputError::SourceTimeContextMismatch);
        }
    }

    Ok(())
}

fn is_finite(observation: &InputObservation) -> bool {
    match observation {
        InputObservation::Keyboard(_) | InputObservation::PointerButton(_) => true,
        InputObservation::AbsolutePointerPosition { position } => point_is_finite(*position),
        InputObservation::RelativeMotion { delta, .. } => {
            delta.x.is_finite() && delta.y.is_finite()
        }
        InputObservation::Scroll { delta, .. } => {
            delta.horizontal.is_none_or(|value| value.is_finite())
                && delta.vertical.is_none_or(|value| value.is_finite())
        }
        InputObservation::Contact {
            position,
            pressure,
            altitude_angle_radians,
            ..
        } => {
            point_is_finite(*position)
                && pressure.is_none_or(measurement_is_finite)
                && altitude_angle_radians.is_none_or(|value| value.is_finite())
        }
        InputObservation::Tablet(observation) => {
            point_is_finite(observation.position)
                && observation.delta.x.is_finite()
                && observation.delta.y.is_finite()
                && observation.pressure.is_none_or(measurement_is_finite)
                && observation
                    .tangential_pressure
                    .is_none_or(measurement_is_finite)
                && observation.twist.is_none_or(measurement_is_finite)
                && observation.tilt.is_none_or(|tilt| {
                    tilt.x_degrees.is_finite()
                        && tilt.y_degrees.is_finite()
                        && (-90.0..=90.0).contains(&tilt.x_degrees)
                        && (-90.0..=90.0).contains(&tilt.y_degrees)
                })
        }
    }
}

fn measurement_is_finite(measurement: AnalogMeasurement) -> bool {
    measurement.value.is_finite()
        && measurement
            .domain
            .max_possible_force()
            .is_none_or(f32::is_finite)
}

fn has_valid_measurements(observation: &InputObservation) -> bool {
    match observation {
        InputObservation::Contact { pressure, .. } => {
            pressure.is_none_or(|measurement| measurement.domain.accepts(measurement.value))
        }
        InputObservation::Tablet(observation) => {
            observation
                .pressure
                .is_none_or(|measurement| measurement.domain.accepts(measurement.value))
                && observation
                    .tangential_pressure
                    .is_none_or(|measurement| measurement.domain.accepts(measurement.value))
                && observation
                    .twist
                    .is_none_or(|measurement| measurement.domain.accepts(measurement.value))
                && observation.tilt.is_none_or(|tilt| {
                    (-90.0..=90.0).contains(&tilt.x_degrees)
                        && (-90.0..=90.0).contains(&tilt.y_degrees)
                })
        }
        _ => true,
    }
}

fn point_is_finite(point: Point2) -> bool {
    point.x.is_finite() && point.y.is_finite()
}
