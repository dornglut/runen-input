use runen_input::{
    AnalogMeasurement, ContactId, ContactInput, ContactPhase, ContactPresence, ContinuityLoss,
    CoordinateSpace,
    DeliveryRole, DigitalState, EvidenceStatus, InputContext, InputDeviceId, InputError,
    InputObservation, InputObservationGroup, InputSourceId, InputState, InputToolKind, KeyLocation,
    KeyboardInput, LogicalKey, MeasurementDomain, NativeLogicalKey, NativePhysicalKeyCode,
    ObservationOrigin, PhysicalKeyIdentity, PhysicalTabletControls, Point2, PointerButton,
    PointerButtonInput, RelativeMotionUnit, ScrollDelta, ScrollDomain, ScrollInput, ScrollPhase,
    SourceTime, SourceTimeUnit, StylusTilt, TabletCapabilities, TabletObservation, ToolId, Vector2,
};

fn assert_public_type<T>() {}

#[test]
fn crate_root_exports_the_complete_accepted_public_surface() {
    assert_public_type::<AnalogMeasurement>();
    assert_public_type::<ContactId>();
    assert_public_type::<ContactInput>();
    assert_public_type::<ContactPhase>();
    assert_public_type::<ContactPresence>();
    assert_public_type::<ContinuityLoss>();
    assert_public_type::<CoordinateSpace>();
    assert_public_type::<DeliveryRole>();
    assert_public_type::<DigitalState>();
    assert_public_type::<EvidenceStatus>();
    assert_public_type::<InputContext>();
    assert_public_type::<InputDeviceId>();
    assert_public_type::<InputError>();
    assert_public_type::<InputObservation>();
    assert_public_type::<InputObservationGroup>();
    assert_public_type::<InputSourceId>();
    assert_public_type::<InputState>();
    assert_public_type::<InputToolKind>();
    assert_public_type::<KeyLocation>();
    assert_public_type::<KeyboardInput>();
    assert_public_type::<LogicalKey>();
    assert_public_type::<MeasurementDomain>();
    assert_public_type::<NativeLogicalKey>();
    assert_public_type::<NativePhysicalKeyCode>();
    assert_public_type::<ObservationOrigin>();
    assert_public_type::<PhysicalKeyIdentity>();
    assert_public_type::<PhysicalTabletControls>();
    assert_public_type::<Point2>();
    assert_public_type::<PointerButton>();
    assert_public_type::<PointerButtonInput>();
    assert_public_type::<RelativeMotionUnit>();
    assert_public_type::<ScrollDelta>();
    assert_public_type::<ScrollDomain>();
    assert_public_type::<ScrollInput>();
    assert_public_type::<ScrollPhase>();
    assert_public_type::<SourceTime>();
    assert_public_type::<SourceTimeUnit>();
    assert_public_type::<StylusTilt>();
    assert_public_type::<TabletCapabilities>();
    assert_public_type::<TabletObservation>();
    assert_public_type::<ToolId>();
    assert_public_type::<Vector2>();
}

fn keyboard(
    key: PhysicalKeyIdentity,
    state: DigitalState,
    origin: ObservationOrigin,
) -> KeyboardInput {
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
                InputObservation::Contact(ContactInput {
                    contact: ContactId::new(4),
                    phase: ContactPhase::Begin,
                    position,
                    pressure: None,
                    altitude_angle_radians: None,
                }),
            ],
        ))
        .expect("pointer/contact group should admit");
    assert_eq!(state.absolute_pointer_position(source), Some(position));
    assert_eq!(
        state.contact_position_in(context_a, ContactId::new(4)),
        Some(position)
    );

    let scroll = ScrollInput {
        delta: ScrollDelta::vertical_only(2.0),
        domain: ScrollDomain::Lines,
        phase: Some(ScrollPhase::Update),
    };
    assert_eq!(scroll.delta.horizontal, None);
    state
        .admit(InputObservationGroup::single(
            context_a,
            InputObservation::Scroll(scroll),
        ))
        .expect("semantic scroll payload should admit directly");

    assert_eq!(
        MeasurementDomain::calibrated_force(5.0).max_possible_force(),
        Some(5.0)
    );
}

#[test]
fn public_contract_scopes_continuity_loss_without_fabricating_releases() {
    let source = InputSourceId::new(21);
    let context_a = InputContext::new(source, Some(InputDeviceId::new(1)));
    let context_b = InputContext::new(source, Some(InputDeviceId::new(2)));
    let key = PhysicalKeyIdentity::code("KeyContinuity");
    let position = Point2::new(8.0, 13.0, CoordinateSpace::WindowPhysicalPixels);
    let mut state = InputState::default();

    for context in [context_a, context_b] {
        state
            .admit(InputObservationGroup::single(
                context,
                InputObservation::Keyboard(keyboard(
                    key.clone(),
                    DigitalState::Pressed,
                    ObservationOrigin::SourceReport,
                )),
            ))
            .expect("device key state should admit");
    }
    state
        .admit(InputObservationGroup::single(
            context_a,
            InputObservation::AbsolutePointerPosition { position },
        ))
        .expect("source pointer state should admit");

    state
        .admit(InputObservationGroup::single(
            context_a,
            InputObservation::ContinuityLost(ContinuityLoss::Device),
        ))
        .expect("device continuity loss should admit");

    assert!(!state.key_down_in(context_a, &key));
    assert!(state.key_down_in(context_b, &key));
    assert_eq!(state.absolute_pointer_position(source), Some(position));

    state
        .admit(InputObservationGroup::single(
            context_b,
            InputObservation::ContinuityLost(ContinuityLoss::Source),
        ))
        .expect("source continuity loss should admit");

    assert!(!state.key_down_in(context_b, &key));
    assert_eq!(state.absolute_pointer_position(source), None);
}
