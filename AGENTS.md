# RunenInput executor contract

Begin with `README.md`, `ARCHITECTURE.md`, `TESTING.md`,
`BOOTSTRAP.md`, and the current owning issue. Re-resolve current `main`,
source authority, open writers, and applicable Engineering decisions before
editing.

## Durable constraints

- Keep one semantic authority per concern and preserve one-way dependencies.
- Keep public RunenInput contracts backend- and host-neutral.
- Keep one product package unless independent authority proves another release
  boundary. `xtask` is tooling only.
- Keep reducer-internal control ids and transition representations private.
- Keep `InputObservationGroup` + `InputState::admit` as the canonical public
  mutation boundary unless a later accepted contract changes it.
- Do not move App/Host, winit/native acquisition, RunenECS, RunenUI, text/IME,
  product actions/bindings, Draw, camera, replay/network, or speculative device
  semantics into this crate by consumer convenience.
- Do not add compatibility aliases, forwarding modules/packages, source includes,
  submodules, moving-branch dependencies, mirrors, or duplicate reducer authority.
- Keep tracked-content contributions `owner-only` until an accepted inbound
  mechanism preserves commercial-relicensing rights.

## ADR-0008 transfer rule

Before the initial successor transfer is accepted, Runenwerk remains semantic
source authority and the RunenInput branch is staging only.

Once the successor transfer is accepted on `main`, RunenInput becomes sole
semantic source authority. The predecessor copy is frozen and may change only
for downstream integration, exact-revision migration, evidence, and deletion.

Cutover-blocking reusable defects after the switch are fixed in RunenInput, not
in the frozen predecessor.

## Validation

Run from a clean checkout:

```text
cargo validate
```

Exact-head repository CI is acceptance authority. Reconcile current `main`,
review state, predecessor source blob/open writers, and repository settings
before guarded squash merge. Never claim local, CI, platform, or downstream
evidence that was not observed.
