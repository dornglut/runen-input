# RunenInput architecture

## Dependency direction

```text
platform/backend adapters
        |
        v
    RunenInput
        |
        +--> Runenwerk integration/projections
        +--> other explicit consumers

RunenInput -X-> Runenwerk
RunenInput -X-> RunenUI
RunenInput -X-> RunenECS
```

Platform acquisition remains outside RunenInput. Consumers adapt admitted
device-level evidence upward into their own runtime/product semantics.

## Ownership

RunenInput owns backend-neutral device-input observation and deterministic
confirmed-state semantics: session-scoped identity, keyboard evidence, pointer
buttons and motion, scroll, contacts, demonstrated tablet/stylus measurements,
source time, evidence/history/origin, grouped admission, validation, and
confirmed-state reduction.

The public mutation boundary is one `InputState::admit(InputObservationGroup)`
path. Keyboard, pointer-button, scroll, and contact observations each carry one
canonical public semantic payload. Contact identity is represented by
`ContactId`, not a parallel raw scalar id. Reducer-internal `ControlId` and
digital-transition representations remain private.

RunenInput does not own App/Host/window lifecycle, native acquisition, backend
health/calibration acquisition, actions/bindings, UI routing/focus/text,
Draw/camera policy, ECS semantics, persisted replay/device profiles, network
input protocols, stable hardware identity, or a universal action framework.

## State semantics

Observation, confirmed state, product action, and UI interaction are separate
authorities.

- repeated or reconciled keyboard evidence can correct confirmed held state
  without creating product edges here;
- source/device contexts do not alias;
- aggregate key/button queries remain true while any admitted context holds;
- absolute pointer position is not reconstructed from relative motion;
- absent scroll axes remain absent rather than measured zero;
- measurement domains preserve uncertainty;
- predicted/estimated tablet samples are deliverable evidence but do not mutate
  confirmed contact state;
- invalid groups reject atomically.

Source/admission ordering remains reducer-owned implementation semantics. It is
not published merely because internal conformance inspects it.

## Repository boundary

RunenInput is one product package, `runen-input` / `runen_input`.
`xtask` is repository tooling. `conformance/downstream` is an independent
consumer proof, not a second product authority.

The semantic core is std-only. No compatibility forwarder, source include,
submodule, moving branch dependency, or private predecessor reach-through is
part of the architecture.

## Current authority

The ADR-0008 source-authority handoff and Runenwerk predecessor retirement are
complete. RunenInput `main` is the sole reusable semantic authority for this
boundary.

Runenwerk consumes an immutable accepted RunenInput revision and owns only its
backend acquisition, runtime integration, compatibility projections, and product
semantics. Future reusable input changes are accepted here first and adopted by
consumers through explicit immutable revisions.

Historical bootstrap and transfer provenance remains recorded in
[BOOTSTRAP.md](BOOTSTRAP.md).
