use runen_input::{
    DigitalState, InputContext, InputError, InputObservation, InputObservationGroup, InputSourceId,
    InputState, KeyLocation, KeyboardInput, LogicalKey, NativeLogicalKey, ObservationOrigin,
    PhysicalKeyIdentity,
};

fn main() -> Result<(), InputError> {
    let context = InputContext::new(InputSourceId::new(1), None);
    let key = PhysicalKeyIdentity::code("KeyA");
    let observation = InputObservation::Keyboard(KeyboardInput {
        physical_key: key.clone(),
        logical_key: LogicalKey::Native(NativeLogicalKey::Unidentified),
        location: KeyLocation::Standard,
        state: DigitalState::Pressed,
        repeat: false,
        origin: ObservationOrigin::SourceReport,
    });

    let group = InputObservationGroup::single(context, observation);
    let mut state = InputState::default();
    state.admit(&group)?;

    assert_eq!(group.observations.len(), 1);
    assert!(state.key_down_in(context, &key));
    assert!(state.key_down_anywhere(&key));

    Ok(())
}
