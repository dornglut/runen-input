use runen_input::{
    ContactId, ContactPhase, CoordinateSpace, DigitalState, InputContext, InputDeviceId,
    InputObservation, InputObservationGroup, InputSourceId, InputState, KeyLocation,
    KeyboardInput, LogicalKey, MeasurementDomain, NativeLogicalKey, ObservationOrigin,
    PhysicalKeyIdentity, Point2, PointerButton, PointerButtonInput, ScrollDelta,
};

fn keyboard(key: PhysicalKeyIdentity, state: DigitalState, origin: ObservationOrigin) -> KeyboardInput {
    KeyboardInput {
        physical_key: key,
        logical_key: LogicalKey::Native(NativeLogicalKey::Unidentified),
        location: KeyLocation::Standard,
        state,
        repeat: false,
        origin,
    }
}

#[test]
fn public_contract_admits_semantic_observations_and_queries_confirmed_state() {
    let source = InputSourceId::new(7);
    let context_a = InputContext::new(source, Some(InputDeviceId::new(1)));
    let context_b = InputContext::new(source, Some(InputDeviceId::new(2)));
    let key = PhysicalKeyIdentity::code("KeyA");
    let mut state = InputState::default();

    state
        .admit(InputObservationGroup::single(
            context_a,
            InputObservation::Keyboard(keyboard(
                key.clone(),
                DigitalState::Pressed,
                ObservationOrigin::SourceReport,
            )),
        ))
        .expect("semantic keyboard observation should admit");
    assert!(state.key_down_in(context_a, &key));
    assert!(state.key_down_anywhere(&key));
    assert!(!state.key_down_in(context_b, &key));

    state
        .admit(InputObservationGroup::single(
            context_b,
            InputObservation::PointerButton(PointerButtonInput {
                button: PointerButton::Left,
                state: DigitalState::Pressed,
            }),
        ))
        .expect("semantic pointer-button observation should admit");
    assert!(state.pointer_button_down_in(context_b, PointerButton::Left));
    assert!(state.pointer_button_down_anywhere(PointerButton::Left));

    let position = Point2::new(12.0, 18.0, CoordinateSpace::WindowPhysicalPixels);
    state
        .admit(InputObservationGroup::new(
            context_a,
            vec![
                InputObservation::AbsolutePointerPosition { position },
                InputObservation::Contact {
                    contact: ContactId::new(4),
                    phase: ContactPhase::Begin,
                    position,
                    pressure: None,
                    altitude_angle_radians: None,
                },
            ],
        ))
        .expect("pointer/contact group should admit");
    assert_eq!(state.absolute_pointer_position(source), Some(position));
    assert_eq!(
        state.contact_position_in(context_a, ContactId::new(4)),
        Some(position)
    );

    assert_eq!(ScrollDelta::vertical_only(2.0).horizontal, None);
    assert_eq!(
        MeasurementDomain::calibrated_force(5.0).max_possible_force(),
        Some(5.0)
    );
}
