# RunenInput architecture

## Current state

`dornglut/runen-input` is a bootstrap-stage standalone Rust-framework
successor. No transferable RunenInput implementation has moved into this
repository yet.

Until accepted successor publication under Engineering ADR 0008,
`dornglut/runenwerk` remains the sole semantic source authority for the
transferable neutral-input implementation.

## Intended ownership boundary

RunenInput is intended to own reusable backend-neutral device-input observation
and deterministic confirmed-state semantics, including:

- session/source/device/tool/control/contact identity whose invariants belong to
  neutral input;
- physical and logical keyboard evidence, key location, repeat, and
  reconciliation provenance as distinct facts;
- pointer buttons, absolute pointer position, and relative motion;
- two-dimensional scroll, domain, phase, and source provenance;
- touch/contact lifetime and cancellation;
- demonstrated tablet/stylus physical observations;
- coordinate and measurement domains, source time, delivery/history role,
  evidence certainty, and observation origin;
- deterministic observation admission/order and confirmed-state reduction;
- framework-local conformance for those semantics.

RunenInput does not own:

- Runenwerk App, Host, window, event-loop, or frame lifecycle;
- winit/native OS APIs, acquisition, backend health, or calibration acquisition;
- Runenwerk product actions, bindings, defaults, rebinding, or camera behavior;
- RunenUI focus, capture, routing, widgets, text editing, or accessibility;
- committed text or IME composition;
- Draw stroke/tool behavior;
- RunenECS scheduling/resource/component semantics;
- replay/persistence/network formats or stable hardware identity without a
  separate accepted authority;
- a generic event bus, RunenCore, universal action framework, or speculative
  device families.

## Dependency direction

The intended dependency direction is one-way:

```text
platform/backend adapters
        |
        v
    RunenInput
        |
        +--> Runenwerk integration/projections
        +--> explicit independent consumers

RunenInput -X-> Runenwerk
RunenInput -X-> RunenUI
RunenInput -X-> RunenECS
```

Platform acquisition remains outside RunenInput. Consumers adapt RunenInput
contracts upward into their own product/runtime semantics.

## Repository boundary

The bootstrap repository contains one non-published `runen-input` package plus
the repository-local `xtask`. The root package is intentionally semantic-empty
until the repository-local successor-transfer issue authorizes the accepted
source/provenance transfer.

No public API stability, release compatibility, serialization format, or
additional package topology is established by bootstrap.

## Authority transfer

The accepted transfer sequence is:

```text
Runenwerk accepted neutral-input implementation
    -> sole semantic source authority
unmerged runen-input successor candidate
    -> staging only
accepted runen-input successor on default branch
    -> semantic authority switches under ADR 0008
Runenwerk predecessor implementation
    -> frozen and deletion-bound
Runenwerk exact-revision migration + predecessor deletion
    -> completed handoff
```

Bootstrap is before the successor-candidate step and does not itself switch
semantic authority.
