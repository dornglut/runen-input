# RunenInput bootstrap and provenance

This record captures stable repository-bootstrap and predecessor provenance. It
is not a branch, pull-request, workflow-run, or current-head ledger.

## Generated repository provenance

```text
repository:                     dornglut/runen-input
repository ID:                  1385996662
template repository:            dornglut/rust-framework-template
accepted template commit:       500461d51fe155febc806e288e5bc013e413a785
accepted template tree:         1e1ae24713cd48b5ea2c3fe1da87cf8dd8f8358a
generated initial commit:       c72df4492681eb96489e92dba9617134b229206e
generated initial tree:         1e1ae24713cd48b5ea2c3fe1da87cf8dd8f8358a
generated initial validation:   36031854632 — PASS
```

The generated initial tree exactly matched the accepted template tree. The
historical template material originated under Apache-2.0. Rights already granted
on that historical template-origin revision are not revoked or reinterpreted.
The template is not an ongoing synchronization or architecture authority.

## Bootstrap product decisions

```text
repository:   dornglut/runen-input
package:      runen-input
crate:        runen_input
version:      0.0.0
edition:      2024
rust-version: 1.93.0
publish:      false
profile:      rust-framework
lifecycle:    active
contribution: owner-only
visibility:   public
license:      GPL-3.0-only
```

Rust 1.93.0 is the current bootstrap package/tooling floor inherited from the
accepted template baseline. Because no RunenInput implementation has transferred
yet, bootstrap does not claim that 1.93.0 is a stabilized long-term framework
MSRV. The successor-transfer issue must validate the transferred source before
changing or presenting an MSRV as a substantive framework compatibility
commitment.

## Predecessor provenance

The accepted predecessor boundary at bootstrap is:

```text
predecessor repository: dornglut/runenwerk
accepted predecessor:   7ad601ea931582bd9bf6305611641f7b7349129e
transferable source:    engine/src/plugins/input/neutral.rs
pre-transfer issue:     dornglut/runenwerk#774
accepted delivery:      dornglut/runenwerk#777
```

The accepted pre-transfer correction made the neutral owner self-contained:
production dependencies are standard-library-only, physical keyboard/pointer
identity correlation and confirmed-state reduction are neutral-owned, and
Runenwerk-specific legacy semantic names were removed from the transferable
owner.

`InputState`, product actions/bindings, plugin/ECS hosting, winit/native
acquisition, RunenUI adaptation, Draw behavior, and product camera behavior stay
outside the transferable owner.

No predecessor implementation source has moved into this repository during
bootstrap. Runenwerk remains semantic source authority until later accepted
successor publication under Engineering ADR 0008.

## Accepted predecessor lineage

Stable source-provenance milestones for the transferable owner are:

- Runenwerk #629 — establish the internal neutral-input authority seam;
- Runenwerk #639 — normalize winit input at the platform edge;
- Runenwerk #673 — converge native-tablet observations;
- Runenwerk #675 — correct tablet assurance semantics;
- Runenwerk #777 — make the neutral owner self-contained for transfer.

These references are provenance, not a synchronization mechanism.

## Intentional deviations from the template

1. RunenInput repository/package/crate identity.
2. GPL-3.0-only current product representation plus `LICENSING.md`.
3. RunenInput-specific README, architecture, testing, agent, and provenance
   documentation.
4. RunenInput workflow identity.
5. Product identity and license validation guards.
6. Explicit staging-authority rules for the ADR-0008 transfer.
7. Retention of the template Rust 1.93.0 bootstrap floor until transferred-source
   evidence justifies a different substantive MSRV.

The crate remains semantic-empty and version `0.0.0`; substantive API,
implementation, conformance, and release decisions belong to the successor
transfer authority.
