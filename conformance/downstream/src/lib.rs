#[cfg(test)]
mod tests {
    use runen_input::{
        AnalogMeasurement, CapabilityKnowledge, ContactId, ContactInput, ContactPhase,
        ContactPresence, ContinuityLoss, CoordinateSpace, DeliveryRole, DigitalState,
        EvidenceStatus, InputContext, InputDeviceId, InputError, InputObservation,
        InputObservationGroup, InputSourceId, InputState, InputToolKind, KeyLocation,
        KeyboardInput, LogicalKey, MeasurementDomain, NativeLogicalKey, ObservationOrigin,
        PhysicalKeyIdentity, PhysicalTabletControls, Point2, PointerButton, PointerButtonInput,
        ScrollDelta, ScrollDomain, ScrollInput, TabletCapabilities, TabletObservation, ToolId,
        Vector2,
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
                .admit(InputObservationGroup::single(
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
            .admit(InputObservationGroup::single(
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
            .admit(InputObservationGroup::single(
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
            .admit(InputObservationGroup::single(
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
            .admit(InputObservationGroup::single(
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
            .admit(InputObservationGroup::single(
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
            .admit(InputObservationGroup::single(
                context,
                InputObservation::Tablet(observation.clone()),
            ))
            .expect("sample evidence remains usable while capability metadata is unknown");

        let mut unsupported = observation;
        unsupported.capabilities.pressure = CapabilityKnowledge::Unsupported;
        assert_eq!(
            InputState::default().admit(InputObservationGroup::single(
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
            .admit(InputObservationGroup::new(
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
            .admit(InputObservationGroup::single(
                context,
                InputObservation::ContinuityLoss(ContinuityLoss::Source),
            ))
            .expect("source continuity loss should admit");

        assert!(!state.key_down_in(context, &key));
        assert_eq!(state.contact_position_in(context, contact), None);
        assert_eq!(state.absolute_pointer_position(source), None);
    }
}
