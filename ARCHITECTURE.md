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
path. Keyboard and pointer-button evidence are public observation variants.
Reducer-internal `ControlId` and digital-transition representations remain
private.

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

## ADR-0008 transfer

The initial successor candidate is non-authoritative while unmerged. Acceptance
on RunenInput `main` switches semantic source authority to the accepted
successor revision immediately. The Runenwerk predecessor then freezes until its
exact-revision consumer migration and predecessor deletion are accepted.

A cutover-blocking reusable defect discovered after successor acceptance is fixed
here and accepted here; the frozen predecessor is not patched as an alternate
implementation.
