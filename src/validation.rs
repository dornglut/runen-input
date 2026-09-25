use crate::{
    continuity::ContinuityLoss,
    evidence::{DeliveryRole, EvidenceStatus, SourceTimeUnit},
    measurement::{AnalogMeasurement, Point2},
    observation::{InputError, InputObservation, InputObservationGroup},
    tablet::{CapabilityKnowledge, ContactPresence, InputToolKind},
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
    if group
        .observations
        .iter()
        .any(|observation| !has_consistent_capability_evidence(observation))
    {
        return Err(InputError::UnsupportedTabletCapabilityEvidence);
    }
    for observation in &group.observations {
        if matches!(
            observation,
            InputObservation::ContinuityLoss(ContinuityLoss::Device)
        ) && group.context.device.is_none()
        {
            return Err(InputError::DeviceContinuityLossRequiresDevice);
        }
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
        InputObservation::Keyboard(_)
        | InputObservation::PointerButton(_)
        | InputObservation::ContinuityLoss(_) => true,
        InputObservation::AbsolutePointerPosition { position } => point_is_finite(*position),
        InputObservation::RelativeMotion { delta, .. } => {
            delta.x.is_finite() && delta.y.is_finite()
        }
        InputObservation::Scroll(input) => {
            input.delta.horizontal.is_none_or(|value| value.is_finite())
                && input.delta.vertical.is_none_or(|value| value.is_finite())
        }
        InputObservation::Contact(input) => {
            point_is_finite(input.position)
                && input.pressure.is_none_or(measurement_is_finite)
                && input
                    .altitude_angle_radians
                    .is_none_or(|value| value.is_finite())
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
        InputObservation::Contact(input) => input
            .pressure
            .is_none_or(|measurement| measurement.domain.accepts(measurement.value)),
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

fn has_consistent_capability_evidence(observation: &InputObservation) -> bool {
    let InputObservation::Tablet(observation) = observation else {
        return true;
    };

    capability_allows_evidence(
        observation.capabilities.pressure,
        observation.pressure.is_some(),
    ) && capability_allows_evidence(observation.capabilities.tilt, observation.tilt.is_some())
        && capability_allows_evidence(observation.capabilities.twist, observation.twist.is_some())
        && capability_allows_evidence(
            observation.capabilities.tangential_pressure,
            observation.tangential_pressure.is_some(),
        )
        && capability_allows_evidence(
            observation.capabilities.hover,
            observation.presence == ContactPresence::Hover,
        )
        && capability_allows_evidence(
            observation.capabilities.eraser,
            observation.controls.eraser || observation.tool_kind == InputToolKind::Eraser,
        )
        && capability_allows_evidence(
            observation.capabilities.barrel_controls,
            observation.controls.barrel_primary || observation.controls.barrel_secondary,
        )
        && capability_allows_evidence(
            observation.capabilities.historical_samples,
            observation.delivery == DeliveryRole::HistoricalCoalesced,
        )
        && capability_allows_evidence(
            observation.capabilities.predicted_samples,
            observation.evidence == EvidenceStatus::PredictedProvisional,
        )
}

fn capability_allows_evidence(capability: CapabilityKnowledge, evidence_present: bool) -> bool {
    !evidence_present || capability != CapabilityKnowledge::Unsupported
}

fn point_is_finite(point: Point2) -> bool {
    point.x.is_finite() && point.y.is_finite()
}
