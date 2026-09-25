use std::collections::{HashMap, HashSet};

use crate::{
    contact::ContactPhase,
    continuity::ContinuityLoss,
    digital::DigitalState,
    evidence::{EvidenceStatus, ObservationOrigin},
    identity::{ContactId, InputContext, InputDeviceId, InputSourceId},
    keyboard::PhysicalKeyIdentity,
    measurement::Point2,
    observation::{InputError, InputObservation, InputObservationGroup},
    pointer::PointerButton,
    tablet::ContactPresence,
    validation::validate_group,
};

#[cfg(test)]
use crate::{keyboard::KeyboardInput, pointer::PointerButtonInput};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct ControlId(u64);

impl ControlId {
    const fn new(raw: u64) -> Self {
        Self(raw)
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) struct SourceSequence(u64);

impl SourceSequence {
    fn next(self) -> Self {
        Self(
            self.0
                .checked_add(1)
                .expect("input source sequence exhausted"),
        )
    }

    #[cfg(test)]
    pub(crate) const fn get(self) -> u64 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) struct AdmissionSequence(u64);

impl AdmissionSequence {
    fn next(self) -> Self {
        Self(
            self.0
                .checked_add(1)
                .expect("input admission sequence exhausted"),
        )
    }

    #[cfg(test)]
    pub(crate) const fn get(self) -> u64 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DigitalTransition {
    Down,
    ReconcileDown,
    Up,
    Cancel,
}

#[derive(Debug, Default)]
struct ControlInterner {
    next_control: u64,
    keys: HashMap<PhysicalKeyIdentity, ControlId>,
    buttons: HashMap<PointerButton, ControlId>,
}

impl ControlInterner {
    fn intern_key(&mut self, key: &PhysicalKeyIdentity) -> ControlId {
        if let Some(control) = self.keys.get(key).copied() {
            return control;
        }
        let control = self.next_control();
        self.keys.insert(key.clone(), control);
        control
    }

    fn key(&self, key: &PhysicalKeyIdentity) -> Option<ControlId> {
        self.keys.get(key).copied()
    }

    fn intern_button(&mut self, button: PointerButton) -> ControlId {
        if let Some(control) = self.buttons.get(&button).copied() {
            return control;
        }
        let control = self.next_control();
        self.buttons.insert(button, control);
        control
    }

    fn button(&self, button: PointerButton) -> Option<ControlId> {
        self.buttons.get(&button).copied()
    }

    fn next_control(&mut self) -> ControlId {
        self.next_control = self
            .next_control
            .checked_add(1)
            .expect("neutral input control identity exhausted");
        ControlId::new(self.next_control)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct ContactState {
    position: Point2,
}

#[derive(Debug, Default)]
struct ConfirmedState {
    held_controls: HashSet<(InputSourceId, Option<InputDeviceId>, ControlId)>,
    contacts: HashMap<(InputSourceId, Option<InputDeviceId>, ContactId), ContactState>,
    absolute_pointer_positions: HashMap<InputSourceId, Point2>,
}

#[cfg(test)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct DigitalAdmission {
    transition: DigitalTransition,
    was_down_anywhere: bool,
    is_down_anywhere: bool,
}

#[derive(Debug, Default)]
pub struct InputState {
    state: ConfirmedState,
    controls: ControlInterner,
    source_sequences: HashMap<InputSourceId, SourceSequence>,
    admission_sequence: AdmissionSequence,
}

impl InputState {
    pub fn admit(&mut self, group: InputObservationGroup) -> Result<(), InputError> {
        validate_group(&group)?;

        let source_sequence = self
            .source_sequences
            .entry(group.context.source)
            .or_default();
        *source_sequence = source_sequence.next();
        self.admission_sequence = self.admission_sequence.next();

        for observation in group.observations {
            self.apply(group.context, observation);
        }

        Ok(())
    }

    pub fn key_down_in(&self, context: InputContext, key: &PhysicalKeyIdentity) -> bool {
        self.controls
            .key(key)
            .is_some_and(|control| self.control_down_in(context, control))
    }

    pub fn key_down_anywhere(&self, key: &PhysicalKeyIdentity) -> bool {
        self.controls
            .key(key)
            .is_some_and(|control| self.control_down_anywhere(control))
    }

    pub fn pointer_button_down_in(&self, context: InputContext, button: PointerButton) -> bool {
        self.controls
            .button(button)
            .is_some_and(|control| self.control_down_in(context, control))
    }

    pub fn pointer_button_down_anywhere(&self, button: PointerButton) -> bool {
        self.controls
            .button(button)
            .is_some_and(|control| self.control_down_anywhere(control))
    }

    pub fn absolute_pointer_position(&self, source: InputSourceId) -> Option<Point2> {
        self.state.absolute_pointer_positions.get(&source).copied()
    }

    pub fn contact_position_in(&self, context: InputContext, contact: ContactId) -> Option<Point2> {
        self.state
            .contacts
            .get(&(context.source, context.device, contact))
            .map(|state| state.position)
    }

    fn control_down_in(&self, context: InputContext, control: ControlId) -> bool {
        self.state
            .held_controls
            .contains(&(context.source, context.device, control))
    }

    fn control_down_anywhere(&self, control: ControlId) -> bool {
        self.state
            .held_controls
            .iter()
            .any(|(_, _, candidate)| *candidate == control)
    }

    fn apply_digital_control(
        &mut self,
        context: InputContext,
        control: ControlId,
        transition: DigitalTransition,
    ) {
        match transition {
            DigitalTransition::Down | DigitalTransition::ReconcileDown => {
                self.state
                    .held_controls
                    .insert((context.source, context.device, control));
            }
            DigitalTransition::Up | DigitalTransition::Cancel => {
                self.state
                    .held_controls
                    .remove(&(context.source, context.device, control));
            }
        }
    }

    fn apply_continuity_loss(&mut self, context: InputContext, loss: ContinuityLoss) {
        match loss {
            ContinuityLoss::Source => {
                self.state
                    .held_controls
                    .retain(|(source, _, _)| *source != context.source);
                self.state
                    .contacts
                    .retain(|(source, _, _), _| *source != context.source);
                self.state
                    .absolute_pointer_positions
                    .remove(&context.source);
            }
            ContinuityLoss::Device => {
                let device = context
                    .device
                    .expect("validated device continuity loss requires a device");
                self.state.held_controls.retain(|(source, candidate, _)| {
                    *source != context.source || *candidate != Some(device)
                });
                self.state.contacts.retain(|(source, candidate, _), _| {
                    *source != context.source || *candidate != Some(device)
                });
            }
        }
    }

    fn apply(&mut self, context: InputContext, observation: InputObservation) {
        match observation {
            InputObservation::Keyboard(input) => {
                let control = self.controls.intern_key(&input.physical_key);
                let transition = match (input.origin, input.state) {
                    (ObservationOrigin::SourceReport, DigitalState::Pressed) => {
                        DigitalTransition::Down
                    }
                    (ObservationOrigin::SourceReport, DigitalState::Released) => {
                        DigitalTransition::Up
                    }
                    (ObservationOrigin::BackendSyntheticReconciliation, DigitalState::Pressed) => {
                        DigitalTransition::ReconcileDown
                    }
                    (ObservationOrigin::BackendSyntheticReconciliation, DigitalState::Released) => {
                        DigitalTransition::Cancel
                    }
                };
                self.apply_digital_control(context, control, transition);
            }
            InputObservation::PointerButton(input) => {
                let control = self.controls.intern_button(input.button);
                let transition = match input.state {
                    DigitalState::Pressed => DigitalTransition::Down,
                    DigitalState::Released => DigitalTransition::Up,
                };
                self.apply_digital_control(context, control, transition);
            }
            InputObservation::AbsolutePointerPosition { position } => {
                self.state
                    .absolute_pointer_positions
                    .insert(context.source, position);
            }
            InputObservation::RelativeMotion { .. } | InputObservation::Scroll(_) => {}
            InputObservation::Contact(input) => match input.phase {
                ContactPhase::Begin | ContactPhase::Update => {
                    self.state.contacts.insert(
                        (context.source, context.device, input.contact),
                        ContactState {
                            position: input.position,
                        },
                    );
                }
                ContactPhase::End | ContactPhase::Cancel => {
                    self.state
                        .contacts
                        .remove(&(context.source, context.device, input.contact));
                }
            },
            InputObservation::Tablet(observation) => {
                if observation.evidence != EvidenceStatus::ObservedConfirmed {
                    return;
                }
                match observation.phase {
                    ContactPhase::Begin | ContactPhase::Update
                        if observation.presence == ContactPresence::Contact =>
                    {
                        self.state.contacts.insert(
                            (context.source, context.device, observation.contact),
                            ContactState {
                                position: observation.position,
                            },
                        );
                    }
                    ContactPhase::End
                    | ContactPhase::Cancel
                    | ContactPhase::Begin
                    | ContactPhase::Update => {
                        self.state.contacts.remove(&(
                            context.source,
                            context.device,
                            observation.contact,
                        ));
                    }
                }
            }
            InputObservation::ContinuityLoss(loss) => {
                self.apply_continuity_loss(context, loss);
            }
        }
    }

    #[cfg(test)]
    fn admit_keyboard(
        &mut self,
        context: InputContext,
        input: &KeyboardInput,
    ) -> Result<DigitalAdmission, InputError> {
        let was_down_anywhere = self.key_down_anywhere(&input.physical_key);
        let transition = match (input.origin, input.state) {
            (ObservationOrigin::SourceReport, DigitalState::Pressed) => DigitalTransition::Down,
            (ObservationOrigin::SourceReport, DigitalState::Released) => DigitalTransition::Up,
            (ObservationOrigin::BackendSyntheticReconciliation, DigitalState::Pressed) => {
                DigitalTransition::ReconcileDown
            }
            (ObservationOrigin::BackendSyntheticReconciliation, DigitalState::Released) => {
                DigitalTransition::Cancel
            }
        };
        self.admit(InputObservationGroup::single(
            context,
            InputObservation::Keyboard(input.clone()),
        ))?;
        Ok(DigitalAdmission {
            transition,
            was_down_anywhere,
            is_down_anywhere: self.key_down_anywhere(&input.physical_key),
        })
    }

    #[cfg(test)]
    fn admit_pointer_button(
        &mut self,
        context: InputContext,
        input: PointerButtonInput,
    ) -> Result<DigitalAdmission, InputError> {
        let was_down_anywhere = self.pointer_button_down_anywhere(input.button);
        let transition = match input.state {
            DigitalState::Pressed => DigitalTransition::Down,
            DigitalState::Released => DigitalTransition::Up,
        };
        self.admit(InputObservationGroup::single(
            context,
            InputObservation::PointerButton(input),
        ))?;
        Ok(DigitalAdmission {
            transition,
            was_down_anywhere,
            is_down_anywhere: self.pointer_button_down_anywhere(input.button),
        })
    }

    #[cfg(test)]
    fn contact_state_in(&self, context: InputContext, contact: ContactId) -> Option<ContactState> {
        self.state
            .contacts
            .get(&(context.source, context.device, contact))
            .copied()
    }

    #[cfg(test)]
    fn active_contact_count(&self, source: InputSourceId) -> usize {
        self.state
            .contacts
            .keys()
            .filter(|(candidate, _, _)| *candidate == source)
            .count()
    }

    #[cfg(test)]
    fn source_sequence(&self, source: InputSourceId) -> Option<SourceSequence> {
        self.source_sequences.get(&source).copied()
    }

    #[cfg(test)]
    fn admission_sequence(&self) -> AdmissionSequence {
        self.admission_sequence
    }
}

#[cfg(test)]
mod tests;
