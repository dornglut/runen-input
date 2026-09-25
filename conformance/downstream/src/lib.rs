#[cfg(test)]
mod tests {
    use runen_input::{
        ContactId, ContactInput, ContactPhase, CoordinateSpace, DigitalState, InputContext,
        InputDeviceId, InputObservation, InputObservationGroup, InputSourceId, InputState,
        KeyLocation, KeyboardInput, LogicalKey, NativeLogicalKey, ObservationOrigin,
        PhysicalKeyIdentity, Point2, PointerButton, PointerButtonInput, ScrollDelta, ScrollDomain,
        ScrollInput,
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
}
