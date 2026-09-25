use super::{DigitalTransition, InputState};
use crate::{
    AnalogMeasurement, CapabilityKnowledge, ContactId, ContactInput, ContactPhase, ContactPresence,
    ContinuityLoss, CoordinateSpace, DeliveryRole, DigitalState, EvidenceStatus, InputContext,
    InputDeviceId, InputError, InputObservation, InputObservationGroup, InputSourceId,
    InputToolKind, KeyLocation, KeyboardInput, LogicalKey, MeasurementDomain, NativeLogicalKey,
    ObservationOrigin, PhysicalKeyIdentity, PhysicalTabletControls, Point2, PointerButton,
    PointerButtonInput, RelativeMotionUnit, ScrollDelta, ScrollDomain, ScrollInput, SourceTime,
    SourceTimeUnit, TabletCapabilities, TabletObservation, ToolId, Vector2,
};

const SOURCE_A: InputSourceId = InputSourceId::new(1);
const SOURCE_B: InputSourceId = InputSourceId::new(2);
const CONTEXT_A: InputContext = InputContext::new(SOURCE_A, None);
const CONTEXT_B: InputContext = InputContext::new(SOURCE_B, None);

#[test]
fn source_and_admission_sequences_are_distinct_and_deterministic() {
    let mut authority = InputState::default();

    authority
        .admit(InputObservationGroup::single(
            CONTEXT_A,
            InputObservation::Keyboard(keyboard_input(
                PhysicalKeyIdentity::code("KeyControl"),
                DigitalState::Pressed,
                false,
                ObservationOrigin::SourceReport,
            )),
        ))
        .expect("first source-A observation should admit");
    assert_eq!(authority.source_sequence(SOURCE_A).unwrap().get(), 1);
    assert_eq!(authority.admission_sequence().get(), 1);

    authority
        .admit(InputObservationGroup::single(
            CONTEXT_B,
            InputObservation::RelativeMotion {
                delta: Vector2::new(1.0, -1.0),
                unit: RelativeMotionUnit::BackendDeviceUnits,
            },
        ))
        .expect("source-B observation should admit");
    assert_eq!(authority.source_sequence(SOURCE_B).unwrap().get(), 1);
    assert_eq!(authority.admission_sequence().get(), 2);

    authority
        .admit(InputObservationGroup::single(
            CONTEXT_A,
            InputObservation::Keyboard(keyboard_input(
                PhysicalKeyIdentity::code("KeyControl"),
                DigitalState::Released,
                false,
                ObservationOrigin::SourceReport,
            )),
        ))
        .expect("second source-A observation should admit");
    assert_eq!(authority.source_sequence(SOURCE_A).unwrap().get(), 2);
    assert_eq!(authority.admission_sequence().get(), 3);
}

#[test]
fn down_then_up_remain_two_admissions_even_when_final_state_matches_initial() {
    let mut authority = InputState::default();

    authority
        .admit(InputObservationGroup::single(
            CONTEXT_A,
            InputObservation::Keyboard(keyboard_input(
                PhysicalKeyIdentity::code("KeyControl"),
                DigitalState::Pressed,
                false,
                ObservationOrigin::SourceReport,
            )),
        ))
        .expect("down should admit");
    assert!(authority.key_down_in(CONTEXT_A, &PhysicalKeyIdentity::code("KeyControl")));
    assert_eq!(authority.admission_sequence().get(), 1);

    authority
        .admit(InputObservationGroup::single(
            CONTEXT_A,
            InputObservation::Keyboard(keyboard_input(
                PhysicalKeyIdentity::code("KeyControl"),
                DigitalState::Released,
                false,
                ObservationOrigin::SourceReport,
            )),
        ))
        .expect("up should admit");
    assert!(!authority.key_down_in(CONTEXT_A, &PhysicalKeyIdentity::code("KeyControl")));
    assert_eq!(authority.admission_sequence().get(), 2);
}

fn keyboard_input(
    physical_key: PhysicalKeyIdentity,
    state: DigitalState,
    repeat: bool,
    origin: ObservationOrigin,
) -> KeyboardInput {
    KeyboardInput {
        physical_key,
        logical_key: LogicalKey::Native(NativeLogicalKey::Unidentified),
        location: KeyLocation::Standard,
        state,
        repeat,
        origin,
    }
}

#[test]
fn keyboard_identity_and_aggregate_held_state_are_neutral_owned() {
    let mut authority = InputState::default();
    let key_a = PhysicalKeyIdentity::code("KeyA");
    let key_b = PhysicalKeyIdentity::code("KeyB");
    let context_a = InputContext::new(SOURCE_A, Some(InputDeviceId::new(1)));
    let context_b = InputContext::new(SOURCE_A, Some(InputDeviceId::new(2)));

    let first = authority
        .admit_keyboard(
            context_a,
            &keyboard_input(
                key_a.clone(),
                DigitalState::Pressed,
                false,
                ObservationOrigin::SourceReport,
            ),
        )
        .expect("first key press should admit");
    assert!(!first.was_down_anywhere);
    assert!(first.is_down_anywhere);
    assert!(authority.key_down_in(context_a, &key_a));
    assert!(!authority.key_down_in(context_b, &key_a));
    assert!(!authority.key_down_anywhere(&key_b));

    let second = authority
        .admit_keyboard(
            context_b,
            &keyboard_input(
                key_a.clone(),
                DigitalState::Pressed,
                false,
                ObservationOrigin::SourceReport,
            ),
        )
        .expect("second-device key press should admit");
    assert!(second.was_down_anywhere);
    assert!(second.is_down_anywhere);

    let release_a = authority
        .admit_keyboard(
            context_a,
            &keyboard_input(
                key_a.clone(),
                DigitalState::Released,
                false,
                ObservationOrigin::SourceReport,
            ),
        )
        .expect("first-device key release should admit");
    assert!(release_a.was_down_anywhere);
    assert!(release_a.is_down_anywhere);
    assert!(!authority.key_down_in(context_a, &key_a));
    assert!(authority.key_down_in(context_b, &key_a));

    let release_b = authority
        .admit_keyboard(
            context_b,
            &keyboard_input(
                key_a.clone(),
                DigitalState::Released,
                false,
                ObservationOrigin::SourceReport,
            ),
        )
        .expect("second-device key release should admit");
    assert!(release_b.was_down_anywhere);
    assert!(!release_b.is_down_anywhere);
    assert!(!authority.key_down_anywhere(&key_a));
}

#[test]
fn keyboard_repeat_and_reconciliation_preserve_neutral_transition_semantics() {
    let mut authority = InputState::default();
    let key = PhysicalKeyIdentity::code("KeyR");

    let first = authority
        .admit_keyboard(
            CONTEXT_A,
            &keyboard_input(
                key.clone(),
                DigitalState::Pressed,
                false,
                ObservationOrigin::SourceReport,
            ),
        )
        .expect("ordinary press should admit");
    assert_eq!(first.transition, DigitalTransition::Down);
    assert!(!first.was_down_anywhere);

    let repeat = authority
        .admit_keyboard(
            CONTEXT_A,
            &keyboard_input(
                key.clone(),
                DigitalState::Pressed,
                true,
                ObservationOrigin::SourceReport,
            ),
        )
        .expect("repeat should admit as repeated evidence");
    assert_eq!(repeat.transition, DigitalTransition::Down);
    assert!(repeat.was_down_anywhere);
    assert!(repeat.is_down_anywhere);

    let cancel = authority
        .admit_keyboard(
            CONTEXT_A,
            &keyboard_input(
                key.clone(),
                DigitalState::Released,
                false,
                ObservationOrigin::BackendSyntheticReconciliation,
            ),
        )
        .expect("synthetic release should reconcile");
    assert_eq!(cancel.transition, DigitalTransition::Cancel);
    assert!(!cancel.is_down_anywhere);

    let reconcile = authority
        .admit_keyboard(
            CONTEXT_A,
            &keyboard_input(
                key.clone(),
                DigitalState::Pressed,
                false,
                ObservationOrigin::BackendSyntheticReconciliation,
            ),
        )
        .expect("synthetic press should reconcile");
    assert_eq!(reconcile.transition, DigitalTransition::ReconcileDown);
    assert!(reconcile.is_down_anywhere);
}

#[test]
fn pointer_button_identity_and_aggregate_state_are_neutral_owned() {
    let mut authority = InputState::default();
    let context_a = InputContext::new(SOURCE_A, Some(InputDeviceId::new(1)));
    let context_b = InputContext::new(SOURCE_B, Some(InputDeviceId::new(2)));
    let pressed = PointerButtonInput {
        button: PointerButton::Left,
        state: DigitalState::Pressed,
    };
    let released = PointerButtonInput {
        button: PointerButton::Left,
        state: DigitalState::Released,
    };

    let first = authority
        .admit_pointer_button(context_a, pressed)
        .expect("first button press should admit");
    assert!(!first.was_down_anywhere);
    assert!(authority.pointer_button_down_in(context_a, PointerButton::Left));
    assert!(!authority.pointer_button_down_anywhere(PointerButton::Right));

    let second = authority
        .admit_pointer_button(context_b, pressed)
        .expect("second-source button press should admit");
    assert!(second.was_down_anywhere);

    let release_a = authority
        .admit_pointer_button(context_a, released)
        .expect("first-source release should admit");
    assert!(release_a.is_down_anywhere);
    assert!(!authority.pointer_button_down_in(context_a, PointerButton::Left));
    assert!(authority.pointer_button_down_in(context_b, PointerButton::Left));

    let release_b = authority
        .admit_pointer_button(context_b, released)
        .expect("second-source release should admit");
    assert!(!release_b.is_down_anywhere);
    assert!(!authority.pointer_button_down_anywhere(PointerButton::Left));
}

#[test]
fn same_control_on_distinct_devices_does_not_alias() {
    let mut authority = InputState::default();
    let device_a = InputDeviceId::new(1);
    let device_b = InputDeviceId::new(2);
    let context_a = InputContext::new(SOURCE_A, Some(device_a));
    let context_b = InputContext::new(SOURCE_A, Some(device_b));

    authority
        .admit(InputObservationGroup::single(
            context_a,
            InputObservation::Keyboard(keyboard_input(
                PhysicalKeyIdentity::code("KeyControl"),
                DigitalState::Pressed,
                false,
                ObservationOrigin::SourceReport,
            )),
        ))
        .expect("device-A control should admit");

    assert!(authority.key_down_in(context_a, &PhysicalKeyIdentity::code("KeyControl")));
    assert!(!authority.key_down_in(context_b, &PhysicalKeyIdentity::code("KeyControl")));
}

#[test]
fn same_contact_id_on_distinct_contexts_does_not_alias() {
    let mut authority = InputState::default();
    let contact = ContactId::new(9);
    let context_a = InputContext::new(SOURCE_A, Some(InputDeviceId::new(1)));
    let context_b = InputContext::new(SOURCE_B, Some(InputDeviceId::new(2)));
    let position = Point2::new(4.0, 5.0, CoordinateSpace::WindowPhysicalPixels);

    authority
        .admit(InputObservationGroup::single(
            context_a,
            InputObservation::Contact(ContactInput {
                contact,
                phase: ContactPhase::Begin,
                position,
                pressure: None,
                altitude_angle_radians: None,
            }),
        ))
        .expect("context-A contact should admit");
    authority
        .admit(InputObservationGroup::single(
            context_b,
            InputObservation::Contact(ContactInput {
                contact,
                phase: ContactPhase::Begin,
                position,
                pressure: None,
                altitude_angle_radians: None,
            }),
        ))
        .expect("context-B contact should admit");

    assert!(authority.contact_state_in(context_a, contact).is_some());
    assert!(authority.contact_state_in(context_b, contact).is_some());
}

#[test]
fn source_continuity_loss_invalidates_only_the_affected_source_without_fabricated_edges() {
    let mut authority = InputState::default();
    let context_a = InputContext::new(SOURCE_A, Some(InputDeviceId::new(1)));
    let context_b = InputContext::new(SOURCE_B, Some(InputDeviceId::new(2)));
    let key = PhysicalKeyIdentity::code("KeySourceLoss");
    let contact = ContactId::new(71);
    let position = Point2::new(7.0, 11.0, CoordinateSpace::WindowPhysicalPixels);

    authority
        .admit(InputObservationGroup::new(
            context_a,
            vec![
                InputObservation::Keyboard(keyboard_input(
                    key.clone(),
                    DigitalState::Pressed,
                    false,
                    ObservationOrigin::SourceReport,
                )),
                InputObservation::PointerButton(PointerButtonInput {
                    button: PointerButton::Left,
                    state: DigitalState::Pressed,
                }),
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
        .expect("source-A state should admit");
    authority
        .admit(InputObservationGroup::single(
            context_b,
            InputObservation::Keyboard(keyboard_input(
                key.clone(),
                DigitalState::Pressed,
                false,
                ObservationOrigin::SourceReport,
            )),
        ))
        .expect("source-B state should admit");

    assert_eq!(authority.admission_sequence().get(), 2);
    authority
        .admit(InputObservationGroup::single(
            context_a,
            InputObservation::ContinuityLoss(ContinuityLoss::Source),
        ))
        .expect("source continuity loss should admit");

    assert!(!authority.key_down_in(context_a, &key));
    assert!(authority.key_down_in(context_b, &key));
    assert!(!authority.pointer_button_down_in(context_a, PointerButton::Left));
    assert!(authority.contact_position_in(context_a, contact).is_none());
    assert_eq!(authority.absolute_pointer_position(SOURCE_A), None);
    assert_eq!(authority.admission_sequence().get(), 3);
}

#[test]
fn device_continuity_loss_preserves_sibling_device_and_source_pointer_state() {
    let mut authority = InputState::default();
    let context_a = InputContext::new(SOURCE_A, Some(InputDeviceId::new(10)));
    let context_b = InputContext::new(SOURCE_A, Some(InputDeviceId::new(11)));
    let key = PhysicalKeyIdentity::code("KeyDeviceLoss");
    let contact = ContactId::new(72);
    let position = Point2::new(17.0, 19.0, CoordinateSpace::WindowPhysicalPixels);

    for context in [context_a, context_b] {
        authority
            .admit(InputObservationGroup::new(
                context,
                vec![
                    InputObservation::Keyboard(keyboard_input(
                        key.clone(),
                        DigitalState::Pressed,
                        false,
                        ObservationOrigin::SourceReport,
                    )),
                    InputObservation::PointerButton(PointerButtonInput {
                        button: PointerButton::Right,
                        state: DigitalState::Pressed,
                    }),
                    InputObservation::Contact(ContactInput {
                        contact,
                        phase: ContactPhase::Begin,
                        position,
                        pressure: None,
                        altitude_angle_radians: None,
                    }),
                ],
            ))
            .expect("device-scoped state should admit");
    }
    authority
        .admit(InputObservationGroup::single(
            context_a,
            InputObservation::AbsolutePointerPosition { position },
        ))
        .expect("source pointer position should admit");

    authority
        .admit(InputObservationGroup::single(
            context_a,
            InputObservation::ContinuityLoss(ContinuityLoss::Device),
        ))
        .expect("device continuity loss should admit");

    assert!(!authority.key_down_in(context_a, &key));
    assert!(authority.key_down_in(context_b, &key));
    assert!(!authority.pointer_button_down_in(context_a, PointerButton::Right));
    assert!(authority.pointer_button_down_in(context_b, PointerButton::Right));
    assert!(authority.contact_position_in(context_a, contact).is_none());
    assert_eq!(
        authority.contact_position_in(context_b, contact),
        Some(position)
    );
    assert_eq!(
        authority.absolute_pointer_position(SOURCE_A),
        Some(position)
    );
}

#[test]
fn repeated_continuity_loss_is_state_idempotent_and_ordered() {
    let mut authority = InputState::default();
    let key = PhysicalKeyIdentity::code("KeyRepeatedLoss");

    authority
        .admit(InputObservationGroup::single(
            CONTEXT_A,
            InputObservation::Keyboard(keyboard_input(
                key.clone(),
                DigitalState::Pressed,
                false,
                ObservationOrigin::SourceReport,
            )),
        ))
        .expect("key state should admit");

    for expected_admission in [2, 3] {
        authority
            .admit(InputObservationGroup::single(
                CONTEXT_A,
                InputObservation::ContinuityLoss(ContinuityLoss::Source),
            ))
            .expect("repeated continuity loss should admit deterministically");
        assert!(!authority.key_down_in(CONTEXT_A, &key));
        assert_eq!(authority.admission_sequence().get(), expected_admission);
    }
}

#[test]
fn reconciliation_can_reestablish_confirmed_state_after_continuity_loss() {
    let mut authority = InputState::default();
    let key = PhysicalKeyIdentity::code("KeyReconcileAfterLoss");

    authority
        .admit(InputObservationGroup::single(
            CONTEXT_A,
            InputObservation::Keyboard(keyboard_input(
                key.clone(),
                DigitalState::Pressed,
                false,
                ObservationOrigin::SourceReport,
            )),
        ))
        .expect("ordinary key state should admit");
    authority
        .admit(InputObservationGroup::single(
            CONTEXT_A,
            InputObservation::ContinuityLoss(ContinuityLoss::Source),
        ))
        .expect("continuity loss should admit");

    let reconciliation = authority
        .admit_keyboard(
            CONTEXT_A,
            &keyboard_input(
                key.clone(),
                DigitalState::Pressed,
                false,
                ObservationOrigin::BackendSyntheticReconciliation,
            ),
        )
        .expect("reconciliation should reestablish confirmed state");

    assert_eq!(reconciliation.transition, DigitalTransition::ReconcileDown);
    assert!(!reconciliation.was_down_anywhere);
    assert!(reconciliation.is_down_anywhere);
    assert!(authority.key_down_in(CONTEXT_A, &key));
}

#[test]
fn device_continuity_loss_without_device_rejects_group_atomically() {
    let mut authority = InputState::default();
    let key = PhysicalKeyIdentity::code("KeyInvalidDeviceLoss");

    let result = authority.admit(InputObservationGroup::new(
        CONTEXT_A,
        vec![
            InputObservation::Keyboard(keyboard_input(
                key.clone(),
                DigitalState::Pressed,
                false,
                ObservationOrigin::SourceReport,
            )),
            InputObservation::ContinuityLoss(ContinuityLoss::Device),
        ],
    ));

    assert_eq!(result, Err(InputError::DeviceContinuityLossRequiresDevice));
    assert!(!authority.key_down_in(CONTEXT_A, &key));
    assert_eq!(authority.admission_sequence().get(), 0);
}

#[test]
fn reconciliation_changes_confirmed_state_without_an_ordinary_edge() {
    let mut authority = InputState::default();

    authority
        .admit(InputObservationGroup::single(
            CONTEXT_A,
            InputObservation::Keyboard(keyboard_input(
                PhysicalKeyIdentity::code("KeyControl"),
                DigitalState::Pressed,
                false,
                ObservationOrigin::BackendSyntheticReconciliation,
            )),
        ))
        .expect("reconciliation down should admit");
    assert!(authority.key_down_in(CONTEXT_A, &PhysicalKeyIdentity::code("KeyControl")));

    authority
        .admit(InputObservationGroup::single(
            CONTEXT_A,
            InputObservation::Keyboard(keyboard_input(
                PhysicalKeyIdentity::code("KeyControl"),
                DigitalState::Released,
                false,
                ObservationOrigin::BackendSyntheticReconciliation,
            )),
        ))
        .expect("cancel should admit");
    assert!(!authority.key_down_in(CONTEXT_A, &PhysicalKeyIdentity::code("KeyControl")));
}

#[test]
fn vertical_only_scroll_keeps_horizontal_absent_not_measured_zero() {
    let input = ScrollInput {
        delta: ScrollDelta::vertical_only(0.0),
        domain: ScrollDomain::Lines,
        phase: None,
    };
    let observation = InputObservation::Scroll(input);

    let InputObservation::Scroll(input) = observation else {
        panic!("scroll payload must remain canonical");
    };
    assert_eq!(input.delta.horizontal, None);
    assert_eq!(input.delta.vertical, Some(0.0));
    assert_ne!(
        input.delta,
        ScrollDelta {
            horizontal: Some(0.0),
            vertical: Some(0.0),
        }
    );
}

#[test]
fn omitted_pressure_remains_distinct_from_measured_zero() {
    let position = Point2::new(10.0, 12.0, CoordinateSpace::UnspecifiedTargetUnits);
    let omitted = InputObservation::Contact(ContactInput {
        contact: ContactId::new(9),
        phase: ContactPhase::Update,
        position,
        pressure: None,
        altitude_angle_radians: None,
    });
    let measured_zero = InputObservation::Contact(ContactInput {
        contact: ContactId::new(9),
        phase: ContactPhase::Update,
        position,
        pressure: Some(AnalogMeasurement::new(
            0.0,
            MeasurementDomain::UnspecifiedScalar,
        )),
        altitude_angle_radians: None,
    });

    assert_ne!(omitted, measured_zero);
}

#[test]
fn invalid_numeric_group_is_rejected_atomically() {
    let mut authority = InputState::default();

    let result = authority.admit(InputObservationGroup::new(
        CONTEXT_A,
        vec![
            InputObservation::Keyboard(keyboard_input(
                PhysicalKeyIdentity::code("KeyControl"),
                DigitalState::Pressed,
                false,
                ObservationOrigin::SourceReport,
            )),
            InputObservation::RelativeMotion {
                delta: Vector2::new(f32::NAN, 0.0),
                unit: RelativeMotionUnit::BackendDeviceUnits,
            },
        ],
    ));

    assert_eq!(result, Err(InputError::NonFiniteObservation));
    assert!(!authority.key_down_in(CONTEXT_A, &PhysicalKeyIdentity::code("KeyControl")));
    assert_eq!(authority.admission_sequence().get(), 0);
}

fn tablet_observation(
    phase: ContactPhase,
    evidence: EvidenceStatus,
    position: Point2,
    pressure: Option<AnalogMeasurement>,
) -> TabletObservation {
    TabletObservation {
        contact: ContactId::new(44),
        tool: Some(ToolId::new(8)),
        tool_kind: InputToolKind::Pen,
        phase,
        presence: ContactPresence::Contact,
        position,
        delta: Vector2::new(1.0, 2.0),
        pressure,
        tangential_pressure: None,
        tilt: None,
        twist: None,
        controls: PhysicalTabletControls::default(),
        capabilities: TabletCapabilities::default(),
        source_time: Some(SourceTime::new(
            CONTEXT_A,
            100,
            SourceTimeUnit::Microseconds,
        )),
        evidence,
        delivery: DeliveryRole::OrdinaryCurrent,
        origin: ObservationOrigin::SourceReport,
    }
}

fn historical_tablet_observation(phase: ContactPhase, position: Point2) -> TabletObservation {
    let mut observation =
        tablet_observation(phase, EvidenceStatus::ObservedConfirmed, position, None);
    observation.delivery = DeliveryRole::HistoricalCoalesced;
    observation
}

#[test]
fn historical_tablet_delivery_never_mutates_current_confirmed_contact_state() {
    let mut authority = InputState::default();
    let contact = ContactId::new(44);
    let historical_position = Point2::new(3.0, 5.0, CoordinateSpace::WindowPhysicalPixels);
    let current_position = Point2::new(10.0, 12.0, CoordinateSpace::WindowPhysicalPixels);

    authority
        .admit(InputObservationGroup::single(
            CONTEXT_A,
            InputObservation::Tablet(historical_tablet_observation(
                ContactPhase::Begin,
                historical_position,
            )),
        ))
        .expect("historical begin should remain deliverable");
    assert!(authority.contact_state_in(CONTEXT_A, contact).is_none());

    authority
        .admit(InputObservationGroup::single(
            CONTEXT_A,
            InputObservation::Tablet(tablet_observation(
                ContactPhase::Begin,
                EvidenceStatus::ObservedConfirmed,
                current_position,
                None,
            )),
        ))
        .expect("ordinary-current begin should establish current contact state");

    for phase in [
        ContactPhase::Begin,
        ContactPhase::Update,
        ContactPhase::End,
        ContactPhase::Cancel,
    ] {
        authority
            .admit(InputObservationGroup::single(
                CONTEXT_A,
                InputObservation::Tablet(historical_tablet_observation(phase, historical_position)),
            ))
            .expect("historical tablet evidence should remain deliverable");
        assert_eq!(
            authority.contact_position_in(CONTEXT_A, contact),
            Some(current_position)
        );
    }

    let mut historical_hover =
        historical_tablet_observation(ContactPhase::Update, historical_position);
    historical_hover.presence = ContactPresence::Hover;
    authority
        .admit(InputObservationGroup::single(
            CONTEXT_A,
            InputObservation::Tablet(historical_hover),
        ))
        .expect("historical hover should remain deliverable");
    assert_eq!(
        authority.contact_position_in(CONTEXT_A, contact),
        Some(current_position)
    );
}

#[test]
fn mixed_tablet_group_keeps_ordinary_current_sample_authoritative() {
    let mut authority = InputState::default();
    let contact = ContactId::new(44);
    let initial_position = Point2::new(10.0, 12.0, CoordinateSpace::WindowPhysicalPixels);
    let current_position = Point2::new(20.0, 24.0, CoordinateSpace::WindowPhysicalPixels);

    authority
        .admit(InputObservationGroup::single(
            CONTEXT_A,
            InputObservation::Tablet(tablet_observation(
                ContactPhase::Begin,
                EvidenceStatus::ObservedConfirmed,
                initial_position,
                None,
            )),
        ))
        .expect("ordinary-current begin should establish current contact state");

    authority
        .admit(InputObservationGroup::new(
            CONTEXT_A,
            vec![
                InputObservation::Tablet(historical_tablet_observation(
                    ContactPhase::Update,
                    Point2::new(2.0, 4.0, CoordinateSpace::WindowPhysicalPixels),
                )),
                InputObservation::Tablet(tablet_observation(
                    ContactPhase::Update,
                    EvidenceStatus::ObservedConfirmed,
                    current_position,
                    None,
                )),
                InputObservation::Tablet(historical_tablet_observation(
                    ContactPhase::Update,
                    Point2::new(3.0, 5.0, CoordinateSpace::WindowPhysicalPixels),
                )),
            ],
        ))
        .expect("mixed historical/current tablet group should admit");

    assert_eq!(
        authority.contact_position_in(CONTEXT_A, contact),
        Some(current_position)
    );
}

#[test]
fn ordinary_current_confirmed_tablet_terminal_phases_clear_contact_state() {
    for phase in [ContactPhase::End, ContactPhase::Cancel] {
        let mut authority = InputState::default();
        let contact = ContactId::new(44);
        let position = Point2::new(10.0, 12.0, CoordinateSpace::WindowPhysicalPixels);

        authority
            .admit(InputObservationGroup::single(
                CONTEXT_A,
                InputObservation::Tablet(tablet_observation(
                    ContactPhase::Begin,
                    EvidenceStatus::ObservedConfirmed,
                    position,
                    None,
                )),
            ))
            .expect("ordinary-current begin should establish current contact state");
        authority
            .admit(InputObservationGroup::single(
                CONTEXT_A,
                InputObservation::Tablet(tablet_observation(
                    phase,
                    EvidenceStatus::ObservedConfirmed,
                    position,
                    None,
                )),
            ))
            .expect("ordinary-current terminal phase should admit");

        assert!(authority.contact_state_in(CONTEXT_A, contact).is_none());
    }
}

#[test]
fn predicted_tablet_observation_does_not_mutate_confirmed_contact_state() {
    let mut authority = InputState::default();
    let position = Point2::new(10.0, 12.0, CoordinateSpace::WindowPhysicalPixels);
    let contact = ContactId::new(44);

    authority
        .admit(InputObservationGroup::single(
            CONTEXT_A,
            InputObservation::Tablet(tablet_observation(
                ContactPhase::Begin,
                EvidenceStatus::ObservedConfirmed,
                position,
                Some(AnalogMeasurement::new(
                    0.2,
                    MeasurementDomain::NormalizedUnitInterval,
                )),
            )),
        ))
        .expect("confirmed tablet observation should admit");
    authority
        .admit(InputObservationGroup::single(
            CONTEXT_A,
            InputObservation::Tablet(tablet_observation(
                ContactPhase::Update,
                EvidenceStatus::PredictedProvisional,
                Point2::new(99.0, 101.0, CoordinateSpace::WindowPhysicalPixels),
                Some(AnalogMeasurement::new(
                    0.9,
                    MeasurementDomain::NormalizedUnitInterval,
                )),
            )),
        ))
        .expect("predicted tablet observation should admit");

    assert_eq!(
        authority
            .contact_state_in(CONTEXT_A, contact)
            .unwrap()
            .position,
        position
    );
}

#[test]
fn estimated_tablet_observations_never_mutate_confirmed_contact_state() {
    let mut authority = InputState::default();
    let position = Point2::new(10.0, 12.0, CoordinateSpace::WindowPhysicalPixels);
    let contact = ContactId::new(44);

    authority
        .admit(InputObservationGroup::single(
            CONTEXT_A,
            InputObservation::Tablet(tablet_observation(
                ContactPhase::Begin,
                EvidenceStatus::EstimatedRevisable,
                position,
                None,
            )),
        ))
        .expect("estimated begin should remain deliverable");
    assert!(authority.contact_state_in(CONTEXT_A, contact).is_none());

    authority
        .admit(InputObservationGroup::single(
            CONTEXT_A,
            InputObservation::Tablet(tablet_observation(
                ContactPhase::Begin,
                EvidenceStatus::ObservedConfirmed,
                position,
                None,
            )),
        ))
        .expect("confirmed begin should admit");
    let revised_position = Point2::new(99.0, 101.0, CoordinateSpace::WindowPhysicalPixels);
    authority
        .admit(InputObservationGroup::single(
            CONTEXT_A,
            InputObservation::Tablet(tablet_observation(
                ContactPhase::Update,
                EvidenceStatus::EstimatedRevisable,
                revised_position,
                None,
            )),
        ))
        .expect("estimated update should remain deliverable");
    assert_eq!(
        authority
            .contact_state_in(CONTEXT_A, contact)
            .expect("confirmed contact should remain held")
            .position,
        position
    );

    authority
        .admit(InputObservationGroup::single(
            CONTEXT_A,
            InputObservation::Tablet(tablet_observation(
                ContactPhase::End,
                EvidenceStatus::EstimatedRevisable,
                revised_position,
                None,
            )),
        ))
        .expect("estimated end should remain deliverable");
    assert!(authority.contact_state_in(CONTEXT_A, contact).is_some());
}

#[test]
fn confirmed_hover_and_out_of_range_clear_stale_tablet_contact_state() {
    let mut authority = InputState::default();
    let contact = ContactId::new(44);
    let position = Point2::new(10.0, 12.0, CoordinateSpace::WindowPhysicalPixels);

    authority
        .admit(InputObservationGroup::single(
            CONTEXT_A,
            InputObservation::Tablet(tablet_observation(
                ContactPhase::Begin,
                EvidenceStatus::ObservedConfirmed,
                position,
                None,
            )),
        ))
        .expect("confirmed begin should admit");
    let mut hover = tablet_observation(
        ContactPhase::Update,
        EvidenceStatus::ObservedConfirmed,
        position,
        None,
    );
    hover.presence = ContactPresence::Hover;
    authority
        .admit(InputObservationGroup::single(
            CONTEXT_A,
            InputObservation::Tablet(hover),
        ))
        .expect("confirmed hover should admit");
    assert!(authority.contact_state_in(CONTEXT_A, contact).is_none());

    authority
        .admit(InputObservationGroup::single(
            CONTEXT_A,
            InputObservation::Tablet(tablet_observation(
                ContactPhase::Begin,
                EvidenceStatus::ObservedConfirmed,
                position,
                None,
            )),
        ))
        .expect("second confirmed begin should admit");
    let mut out_of_range = tablet_observation(
        ContactPhase::Update,
        EvidenceStatus::ObservedConfirmed,
        position,
        None,
    );
    out_of_range.presence = ContactPresence::OutOfRange;
    authority
        .admit(InputObservationGroup::single(
            CONTEXT_A,
            InputObservation::Tablet(out_of_range),
        ))
        .expect("confirmed out-of-range should admit");
    assert!(authority.contact_state_in(CONTEXT_A, contact).is_none());
}

#[test]
fn invalid_tablet_measurement_rejects_group_without_partial_state() {
    let mut authority = InputState::default();
    let result = authority.admit(InputObservationGroup::new(
        CONTEXT_A,
        vec![
            InputObservation::Tablet(tablet_observation(
                ContactPhase::Begin,
                EvidenceStatus::ObservedConfirmed,
                Point2::new(10.0, 12.0, CoordinateSpace::WindowPhysicalPixels),
                Some(AnalogMeasurement::new(
                    0.3,
                    MeasurementDomain::NormalizedUnitInterval,
                )),
            )),
            InputObservation::Tablet(tablet_observation(
                ContactPhase::Update,
                EvidenceStatus::ObservedConfirmed,
                Point2::new(11.0, 13.0, CoordinateSpace::WindowPhysicalPixels),
                Some(AnalogMeasurement::new(
                    1.5,
                    MeasurementDomain::NormalizedUnitInterval,
                )),
            )),
        ],
    ));

    assert_eq!(result, Err(InputError::InvalidMeasurement));
    assert_eq!(authority.active_contact_count(SOURCE_A), 0);
    assert_eq!(authority.admission_sequence().get(), 0);
}

#[test]
fn unknown_tablet_capability_knowledge_remains_distinct_from_unsupported() {
    let mut authority = InputState::default();
    let position = Point2::new(12.0, 14.0, CoordinateSpace::WindowPhysicalPixels);
    let observation = tablet_observation(
        ContactPhase::Begin,
        EvidenceStatus::ObservedConfirmed,
        position,
        Some(AnalogMeasurement::new(
            0.0,
            MeasurementDomain::NormalizedUnitInterval,
        )),
    );

    assert_eq!(
        observation.capabilities.pressure,
        CapabilityKnowledge::Unknown
    );
    authority
        .admit(InputObservationGroup::single(
            CONTEXT_A,
            InputObservation::Tablet(observation),
        ))
        .expect("unknown capability metadata must not erase concrete sample evidence");
    assert_eq!(
        authority.contact_position_in(CONTEXT_A, ContactId::new(44)),
        Some(position)
    );

    let mut supported_without_sample = tablet_observation(
        ContactPhase::Update,
        EvidenceStatus::ObservedConfirmed,
        position,
        None,
    );
    supported_without_sample.capabilities.pressure = CapabilityKnowledge::Supported;
    authority
        .admit(InputObservationGroup::single(
            CONTEXT_A,
            InputObservation::Tablet(supported_without_sample),
        ))
        .expect("supported capability does not require every sample to carry a value");
}

#[test]
fn explicit_unsupported_tablet_capabilities_reject_conflicting_evidence_atomically() {
    let position = Point2::new(20.0, 30.0, CoordinateSpace::WindowPhysicalPixels);
    let mut cases = Vec::new();

    let mut pressure = tablet_observation(
        ContactPhase::Update,
        EvidenceStatus::ObservedConfirmed,
        position,
        Some(AnalogMeasurement::new(
            0.5,
            MeasurementDomain::NormalizedUnitInterval,
        )),
    );
    pressure.capabilities.pressure = CapabilityKnowledge::Unsupported;
    cases.push(("pressure", pressure));

    let mut tilt = tablet_observation(
        ContactPhase::Update,
        EvidenceStatus::ObservedConfirmed,
        position,
        None,
    );
    tilt.tilt = Some(StylusTilt::new(10.0, -5.0));
    tilt.capabilities.tilt = CapabilityKnowledge::Unsupported;
    cases.push(("tilt", tilt));

    let mut twist = tablet_observation(
        ContactPhase::Update,
        EvidenceStatus::ObservedConfirmed,
        position,
        None,
    );
    twist.twist = Some(AnalogMeasurement::new(
        45.0,
        MeasurementDomain::Degrees {
            min: 0.0,
            max: 360.0,
        },
    ));
    twist.capabilities.twist = CapabilityKnowledge::Unsupported;
    cases.push(("twist", twist));

    let mut tangential = tablet_observation(
        ContactPhase::Update,
        EvidenceStatus::ObservedConfirmed,
        position,
        None,
    );
    tangential.tangential_pressure = Some(AnalogMeasurement::new(
        -0.25,
        MeasurementDomain::SignedNormalizedUnitInterval,
    ));
    tangential.capabilities.tangential_pressure = CapabilityKnowledge::Unsupported;
    cases.push(("tangential pressure", tangential));

    let mut hover = tablet_observation(
        ContactPhase::Update,
        EvidenceStatus::ObservedConfirmed,
        position,
        None,
    );
    hover.presence = ContactPresence::Hover;
    hover.capabilities.hover = CapabilityKnowledge::Unsupported;
    cases.push(("hover", hover));

    let mut eraser_tool = tablet_observation(
        ContactPhase::Update,
        EvidenceStatus::ObservedConfirmed,
        position,
        None,
    );
    eraser_tool.tool_kind = InputToolKind::Eraser;
    eraser_tool.capabilities.eraser = CapabilityKnowledge::Unsupported;
    cases.push(("eraser tool", eraser_tool));

    let mut eraser_control = tablet_observation(
        ContactPhase::Update,
        EvidenceStatus::ObservedConfirmed,
        position,
        None,
    );
    eraser_control.controls.eraser = true;
    eraser_control.capabilities.eraser = CapabilityKnowledge::Unsupported;
    cases.push(("eraser control", eraser_control));

    let mut barrel = tablet_observation(
        ContactPhase::Update,
        EvidenceStatus::ObservedConfirmed,
        position,
        None,
    );
    barrel.controls.barrel_primary = true;
    barrel.capabilities.barrel_controls = CapabilityKnowledge::Unsupported;
    cases.push(("barrel control", barrel));

    let mut historical = historical_tablet_observation(ContactPhase::Update, position);
    historical.capabilities.historical_samples = CapabilityKnowledge::Unsupported;
    cases.push(("historical delivery", historical));

    let mut predicted = tablet_observation(
        ContactPhase::Update,
        EvidenceStatus::PredictedProvisional,
        position,
        None,
    );
    predicted.capabilities.predicted_samples = CapabilityKnowledge::Unsupported;
    cases.push(("predicted evidence", predicted));

    for (label, observation) in cases {
        let mut authority = InputState::default();
        let key = PhysicalKeyIdentity::code(format!("KeyCapabilityConflict:{label}"));
        let result = authority.admit(InputObservationGroup::new(
            CONTEXT_A,
            vec![
                InputObservation::Keyboard(keyboard_input(
                    key.clone(),
                    DigitalState::Pressed,
                    false,
                    ObservationOrigin::SourceReport,
                )),
                InputObservation::Tablet(observation),
            ],
        ));

        assert_eq!(
            result,
            Err(InputError::UnsupportedTabletCapabilityEvidence),
            "{label}"
        );
        assert!(!authority.key_down_in(CONTEXT_A, &key), "{label}");
        assert_eq!(authority.admission_sequence().get(), 0, "{label}");
    }
}
