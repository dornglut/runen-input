# RunenInput

RunenInput is a standalone Rust framework for backend-neutral device-input
observations and deterministic confirmed-state semantics.

## Authority

RunenInput's accepted default branch is the semantic source authority for the
standalone implementation present there. Under Engineering ADR 0008, an
unmerged transfer candidate is staging only; the initial authority switch occurs
only when the successor transfer is accepted on `main`.

After that switch, the Runenwerk predecessor copy is frozen and deletion-bound
until the exact-revision consumer cutover completes.

## Boundary

RunenInput owns reusable device-level facts and confirmed state:

- source/device/tool/contact identity;
- physical and logical keyboard evidence, key location, repeat, and origin;
- pointer buttons, absolute pointer position, relative motion, and two-axis scroll;
- touch/contact lifetime and cancellation;
- demonstrated tablet/stylus observations;
- coordinate and measurement domains, source time, delivery/history role,
  evidence certainty, and origin;
- deterministic grouped admission and confirmed-state reduction.

It does not own platform acquisition, winit/OS APIs, App/Host/window lifecycle,
product actions/bindings, RunenUI routing/focus/text semantics, Draw behavior,
camera policy, RunenECS scheduling, or speculative replay/network/device-family
contracts.

## Public contract

`InputObservationGroup` is the canonical admission unit and
`InputState::admit` is the single public mutation path.

Keyboard and pointer-button evidence are semantic observations:

```text
InputObservation::Keyboard(KeyboardInput)
InputObservation::PointerButton(PointerButtonInput)
```

Reducer-internal control ids and digital transition forms are not public API.

`InputState` exposes confirmed-state queries for physical keys, pointer buttons,
absolute pointer position, and contacts. Predicted or estimated tablet evidence
does not mutate confirmed contact state.

## Package

```text
package: runen-input
crate: runen_input
version: 0.1.0
edition: 2024
rust-version: 1.93.0
publish: false
```

## Validation

`cargo validate` is the repository-owned merge-readiness command. It covers
the transferred semantic laws, public-contract integration tests, independent
downstream conformance, source/dependency boundary guards, formatting, locked
tests, strict Clippy, rustdoc, the declared Rust floor, Git whitespace, and
unchanged repository state.

See [TESTING.md](TESTING.md).

## Authority and provenance

- [Architecture](ARCHITECTURE.md)
- [Testing](TESTING.md)
- [Bootstrap and transfer provenance](BOOTSTRAP.md)
- [Executor contract](AGENTS.md)
- [Organization contribution guidance](https://github.com/dornglut/.github/blob/main/CONTRIBUTING.md)
- [Organization security policy](https://github.com/dornglut/.github/blob/main/SECURITY.md)
- [Public license](LICENSE)
- [Commercial-license guidance](LICENSING.md)

## Contribution

Tracked-content contributions are currently `owner-only`. Issues, discussion,
reviews, and reproducible reports may still be used through the repository's
public channels.

## License

RunenInput is publicly represented under [GPL-3.0-only](LICENSE). A separately
governed commercial licensing path is described in
[LICENSING.md](LICENSING.md).
