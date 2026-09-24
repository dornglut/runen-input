# RunenInput bootstrap and transfer provenance

This record contains stable repository-bootstrap and source-transfer provenance.
It is not a branch, pull-request, workflow-run, or current-head ledger.

## Generated repository provenance

```text
repository:                     dornglut/runen-input
repository ID:                  1385996662
template repository:            dornglut/rust-framework-template
accepted template commit:       500461d51fe155febc806e288e5bc013e413a785
accepted template tree:         1e1ae24713cd48b5ea2c3fe1da87cf8dd8f8358a
generated initial commit:       c72df4492681eb96489e92dba9617134b229206e
generated initial tree:         1e1ae24713cd48b5ea2c3fe1da87cf8dd8f8358a
```

The generated initial tree exactly matched the accepted template tree. Historical
template-origin material was granted under Apache-2.0; those historical rights
remain historical and the template is not an ongoing authority.

## Accepted bootstrap

The accepted product skeleton established:

```text
package:      runen-input
crate:        runen_input
edition:      2024
rust-version: 1.93.0
publish:      false
profile:      rust-framework
lifecycle:    active
contribution: owner-only
visibility:   public
license:      GPL-3.0-only
```

The bootstrap accepted no reusable input implementation.

## Transferred predecessor provenance

The substantive implementation originates from:

```text
predecessor:                    dornglut/runenwerk
accepted pre-transfer revision: 7ad601ea931582bd9bf6305611641f7b7349129e
fresh transfer census revision: 785e94af583c0a2f9c2050f0a23f1c38258d7ebc
source path:                    engine/src/plugins/input/neutral.rs
source blob:                    36c5ff401d2c8ce48a03cfece28764949c772556
source lineage:                 #629, #639, #673, #675, #777
```

The source blob was unchanged between accepted pre-transfer correction and the
fresh successor-transfer census.

The transferred semantic nucleus includes observation/value semantics,
deterministic grouped admission, confirmed state, control correlation, validation,
and the 15 focused semantic tests. Runenwerk `InputState`, ActionState/bindings,
App/ECS hosting, winit/native acquisition, UI/Draw adaptation, and camera policy
are not transferred.

## Successor contract adaptation

The predecessor source was self-contained but its external Rust visibility was
not a standalone contract. The successor therefore performs one bounded
contract adaptation:

- public `InputObservation::Keyboard(KeyboardInput)` and
  `InputObservation::PointerButton(PointerButtonInput)` replace exposure of an
  interned digital-control observation;
- public `InputState::admit(InputObservationGroup)` is the canonical mutation
  path;
- reducer-internal `ControlId`, `DigitalTransition`, and interning stay private;
- only demonstrated confirmed-state queries and adapter constructors are exposed.

This changes the external seam, not the accepted device-input semantic laws.

## ADR-0008 authority handoff

An unmerged successor branch is staging only. Acceptance of the initial transfer
on RunenInput `main` is the authority-switch event: the accepted successor
revision becomes sole semantic source authority and the Runenwerk predecessor
copy freezes.

The Runenwerk cutover then pins that exact accepted successor revision and
deletes the predecessor reusable implementation in the same bounded downstream
cutover.

No source mirror, forwarding module, source include, submodule, moving branch
dependency, or dual writable authority is permitted.
