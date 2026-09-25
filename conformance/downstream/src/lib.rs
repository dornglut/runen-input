#[cfg(test)]
mod tests {
    use runen_input::{
        AnalogMeasurement, CapabilityKnowledge, ContactId, ContactInput, ContactPhase,
        ContactPresence, ContinuityLoss, CoordinateSpace, DeliveryRole, DigitalState,
        EvidenceStatus, InputContext, InputDeviceId, InputError, InputObservation,
        InputObservationGroup, InputSourceId, InputState, InputToolKind, KeyLocation,
        KeyboardInput, LogicalKey, MeasurementDomain, NativeLogicalKey, ObservationOrigin,
        PhysicalKeyIdentity, PhysicalTabletControls, Point2, PointerButton, PointerButtonInput,
        RelativeMotionUnit, ScrollDelta, ScrollDomain, ScrollInput, SourceTime, SourceTimeUnit,
        TabletCapabilities,
        TabletObservation, ToolId, Vector2,
    };

    fn keyboard(
        key: PhysicalKeyIdentity,
        state: DigitalState,
        origin: ObservationOrigin,
    ) -> InputObservation {
        InputObservation::Keyboard(KeyboardInput {
            physical_key: key,
            logical_key: LogicalKey::Native(NativeLogicalKey::Unidentified),
            location: KeyLocation::Standard,
            state,
            repeat: false,
            origin,
        })
    }

    #[test]
    fn independent_consumer_uses_only_public_runen_input_contract() {
        let source = InputSourceId::new(1);
        let context_a = InputContext::new(source, Some(InputDeviceId::new(10)));
        let context_b = InputContext::new(source, Some(InputDeviceId::new(11)));
        let key = PhysicalKeyIdentity::code("KeyK");
        let mut state = InputState::default();

        for context in [context_a, context_b] {
            state
                .admit(&InputObservationGroup::single(
                    context,
                    keyboard(
                        key.clone(),
                        DigitalState::Pressed,
                        ObservationOrigin::SourceReport,
                    ),
                ))
                .expect("press should admit");
        }
        assert!(state.key_down_in(context_a, &key));
        assert!(state.key_down_in(context_b, &key));
        assert!(state.key_down_anywhere(&key));

        state
            .admit(&InputObservationGroup::single(
                context_a,
                keyboard(
                    key.clone(),
                    DigitalState::Released,
                    ObservationOrigin::SourceReport,
                ),
            ))
            .expect("first release should admit");
        assert!(!state.key_down_in(context_a, &key));
        assert!(state.key_down_anywhere(&key));

        state
            .admit(&InputObservationGroup::single(
                context_b,
                keyboard(
                    key.clone(),
                    DigitalState::Released,
                    ObservationOrigin::BackendSyntheticReconciliation,
                ),
            ))
            .expect("reconciliation release should admit");
        assert!(!state.key_down_anywhere(&key));

        state
            .admit(&InputObservationGroup::single(
                context_b,
                InputObservation::PointerButton(PointerButtonInput {
                    button: PointerButton::Right,
                    state: DigitalState::Pressed,
                }),
            ))
            .expect("button should admit");
        assert!(state.pointer_button_down_in(context_b, PointerButton::Right));

        let position = Point2::new(4.0, 8.0, CoordinateSpace::WindowPhysicalPixels);
        state
            .admit(&InputObservationGroup::single(
                context_a,
                InputObservation::Contact(ContactInput {
                    contact: ContactId::new(3),
                    phase: ContactPhase::Begin,
                    position,
                    pressure: None,
                    altitude_angle_radians: None,
                }),
            ))
            .expect("contact should admit");
        assert_eq!(
            state.contact_position_in(context_a, ContactId::new(3)),
            Some(position)
        );

        state
            .admit(&InputObservationGroup::single(
                context_a,
                InputObservation::Scroll(ScrollInput {
                    delta: ScrollDelta::vertical_only(1.0),
                    domain: ScrollDomain::Lines,
                    phase: None,
                }),
            ))
            .expect("scroll payload should admit without reconstruction");
    }

    #[test]
    fn independent_consumer_observes_truthful_tablet_capability_knowledge() {
        let context = InputContext::new(InputSourceId::new(22), Some(InputDeviceId::new(5)));
        let position = Point2::new(6.0, 9.0, CoordinateSpace::WindowPhysicalPixels);
        let observation = TabletObservation {
            contact: ContactId::new(8),
            tool: Some(ToolId::new(2)),
            tool_kind: InputToolKind::Pen,
            phase: ContactPhase::Begin,
            presence: ContactPresence::Contact,
            position,
            delta: Vector2::new(0.0, 0.0),
            pressure: Some(AnalogMeasurement::new(
                0.25,
                MeasurementDomain::NormalizedUnitInterval,
            )),
            tangential_pressure: None,
            tilt: None,
            twist: None,
            controls: PhysicalTabletControls::default(),
            capabilities: TabletCapabilities::default(),
            source_time: None,
            evidence: EvidenceStatus::ObservedConfirmed,
            delivery: DeliveryRole::OrdinaryCurrent,
            origin: ObservationOrigin::SourceReport,
        };

        assert_eq!(
            observation.capabilities.pressure,
            CapabilityKnowledge::Unknown
        );
        let mut state = InputState::default();
        state
            .admit(&InputObservationGroup::single(
                context,
                InputObservation::Tablet(observation.clone()),
            ))
            .expect("sample evidence remains usable while capability metadata is unknown");

        let mut unsupported = observation;
        unsupported.capabilities.pressure = CapabilityKnowledge::Unsupported;
        assert_eq!(
            InputState::default().admit(&InputObservationGroup::single(
                context,
                InputObservation::Tablet(unsupported),
            )),
            Err(InputError::UnsupportedTabletCapabilityEvidence)
        );
    }

    #[test]
    fn independent_consumer_can_invalidate_source_continuity() {
        let source = InputSourceId::new(31);
        let context = InputContext::new(source, Some(InputDeviceId::new(41)));
        let key = PhysicalKeyIdentity::code("KeyIndependentContinuity");
        let contact = ContactId::new(51);
        let position = Point2::new(3.0, 5.0, CoordinateSpace::WindowPhysicalPixels);
        let mut state = InputState::default();

        state
            .admit(&InputObservationGroup::new(
                context,
                vec![
                    keyboard(
                        key.clone(),
                        DigitalState::Pressed,
                        ObservationOrigin::SourceReport,
                    ),
                    InputObservation::AbsolutePointerPosition { position },
                    InputObservation::Contact(ContactInput {
                        contact,
                        phase: ContactPhase::Begin,
                        position,
                        pressure: None,
                        altitude_angle_radians: None,
                    }),
                ],
            ))
            .expect("confirmed source state should admit");

        state
            .admit(&InputObservationGroup::single(
                context,
                InputObservation::ContinuityLoss(ContinuityLoss::Source),
            ))
            .expect("source continuity loss should admit");

        assert!(!state.key_down_in(context, &key));
        assert_eq!(state.contact_position_in(context, contact), None);
        assert_eq!(state.absolute_pointer_position(source), None);
    }

    #[test]
    fn independent_consumer_distinguishes_source_time_context_and_unit_failures() {
        let context = InputContext::new(InputSourceId::new(61), Some(InputDeviceId::new(6)));
        let other_context = InputContext::new(InputSourceId::new(62), Some(InputDeviceId::new(7)));
        let position = Point2::new(7.0, 12.0, CoordinateSpace::WindowPhysicalPixels);
        let base = TabletObservation {
            contact: ContactId::new(13),
            tool: Some(ToolId::new(4)),
            tool_kind: InputToolKind::Pen,
            phase: ContactPhase::Begin,
            presence: ContactPresence::Contact,
            position,
            delta: Vector2::new(0.0, 0.0),
            pressure: None,
            tangential_pressure: None,
            tilt: None,
            twist: None,
            controls: PhysicalTabletControls::default(),
            capabilities: TabletCapabilities::default(),
            source_time: Some(SourceTime::new(
                context,
                9,
                SourceTimeUnit::NativeTicks {
                    ticks_per_second: 120,
                },
            )),
            evidence: EvidenceStatus::ObservedConfirmed,
            delivery: DeliveryRole::OrdinaryCurrent,
            origin: ObservationOrigin::SourceReport,
        };

        InputState::default()
            .admit(&InputObservationGroup::single(
                context,
                InputObservation::Tablet(base.clone()),
            ))
            .expect("valid native source time should admit");

        let mut wrong_context = base.clone();
        wrong_context.source_time = Some(SourceTime::new(
            other_context,
            9,
            SourceTimeUnit::Milliseconds,
        ));
        assert_eq!(
            InputState::default().admit(&InputObservationGroup::single(
                context,
                InputObservation::Tablet(wrong_context),
            )),
            Err(InputError::SourceTimeContextMismatch)
        );

        let mut invalid_unit = base;
        invalid_unit.source_time = Some(SourceTime::new(
            context,
            9,
            SourceTimeUnit::NativeTicks {
                ticks_per_second: 0,
            },
        ));
        assert_eq!(
            InputState::default().admit(&InputObservationGroup::single(
                context,
                InputObservation::Tablet(invalid_unit),
            )),
            Err(InputError::InvalidSourceTimeUnit)
        );
    }

    #[test]
    fn independent_consumer_keeps_relative_motion_distinct_from_absolute_position() {
        let source = InputSourceId::new(71);
        let context = InputContext::new(source, Some(InputDeviceId::new(8)));
        let absolute = Point2::new(20.0, 30.0, CoordinateSpace::WindowPhysicalPixels);
        let mut state = InputState::default();

        state
            .admit(&InputObservationGroup::single(
                context,
                InputObservation::AbsolutePointerPosition { position: absolute },
            ))
            .expect("absolute position should admit");
        state
            .admit(&InputObservationGroup::single(
                context,
                InputObservation::RelativeMotion {
                    delta: Vector2::new(4.0, -2.0),
                    unit: RelativeMotionUnit::BackendDeviceUnits,
                },
            ))
            .expect("relative motion should admit through the public contract");

        assert_eq!(state.absolute_pointer_position(source), Some(absolute));
    }

    #[test]
    fn independent_consumer_keeps_noncurrent_tablet_evidence_out_of_confirmed_state() {
        let context = InputContext::new(InputSourceId::new(72), Some(InputDeviceId::new(9)));
        let contact = ContactId::new(21);
        let current = Point2::new(10.0, 12.0, CoordinateSpace::WindowPhysicalPixels);
        let alternate = Point2::new(90.0, 120.0, CoordinateSpace::WindowPhysicalPixels);
        let mut observation = TabletObservation {
            contact,
            tool: Some(ToolId::new(7)),
            tool_kind: InputToolKind::Pen,
            phase: ContactPhase::Begin,
            presence: ContactPresence::Contact,
            position: current,
            delta: Vector2::new(0.0, 0.0),
            pressure: None,
            tangential_pressure: None,
            tilt: None,
            twist: None,
            controls: PhysicalTabletControls::default(),
            capabilities: TabletCapabilities::default(),
            source_time: None,
            evidence: EvidenceStatus::ObservedConfirmed,
            delivery: DeliveryRole::OrdinaryCurrent,
            origin: ObservationOrigin::SourceReport,
        };
        let mut state = InputState::default();

        state
            .admit(&InputObservationGroup::single(
                context,
                InputObservation::Tablet(observation.clone()),
            ))
            .expect("ordinary-current confirmed tablet evidence should admit");
        assert_eq!(state.contact_position_in(context, contact), Some(current));

        observation.phase = ContactPhase::Update;
        observation.position = alternate;
        observation.delivery = DeliveryRole::HistoricalCoalesced;
        state
            .admit(&InputObservationGroup::single(
                context,
                InputObservation::Tablet(observation.clone()),
            ))
            .expect("historical confirmed evidence should remain deliverable");
        assert_eq!(state.contact_position_in(context, contact), Some(current));

        observation.delivery = DeliveryRole::OrdinaryCurrent;
        observation.evidence = EvidenceStatus::PredictedProvisional;
        state
            .admit(&InputObservationGroup::single(
                context,
                InputObservation::Tablet(observation.clone()),
            ))
            .expect("predicted evidence should remain deliverable");
        assert_eq!(state.contact_position_in(context, contact), Some(current));

        observation.evidence = EvidenceStatus::EstimatedRevisable;
        state
            .admit(&InputObservationGroup::single(
                context,
                InputObservation::Tablet(observation),
            ))
            .expect("estimated evidence should remain deliverable");
        assert_eq!(state.contact_position_in(context, contact), Some(current));
    }
}
