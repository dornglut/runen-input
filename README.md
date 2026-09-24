# RunenInput

RunenInput is the planned standalone Dornglut Rust framework for backend-neutral
device-input observations and deterministic confirmed-state semantics.

## Maturity

This repository is currently a **bootstrap-stage successor**. It contains the
RunenInput repository skeleton and validation authority, but no transferable
RunenInput implementation has moved here yet.

Until a later successor-transfer change is accepted on this repository's
`main` under Engineering ADR 0008, `dornglut/runenwerk` remains the sole
semantic source authority for the reusable neutral-input implementation.

## Boundary

RunenInput is intended to own reusable device-level facts and deterministic
confirmed state: source/device/tool/control/contact identity, keyboard evidence,
pointer and scroll observations, touch/contact lifetime, demonstrated
tablet/stylus observations, measurement/coordinate/time/provenance semantics,
observation admission/order, and confirmed-state reduction.

It does not own platform acquisition or winit/OS APIs; Runenwerk App/Host/window
lifecycle; product actions/bindings; RunenUI focus/routing/text semantics; Draw
stroke/tool behavior; camera policy; RunenECS scheduling; or speculative
persistence, replay, network, gesture, haptics, HID, or universal action
frameworks.

See [ARCHITECTURE.md](ARCHITECTURE.md).

## Package

```text
package: runen-input
crate: runen_input
version: 0.0.0
edition: 2024
bootstrap rust-version: 1.93.0
publish: false
```

The current Rust version is a bootstrap floor, not a stabilized long-term
framework MSRV. The successor-transfer work must validate the transferred source
before making a substantive compatibility commitment.

## Validation

`cargo validate` is the single repository-owned merge-readiness command.

It verifies required authority files, product identity and license consistency,
formatting, locked workspace tests, strict Clippy, rustdoc with warnings denied,
Git whitespace, and unchanged repository state. CI invokes the same command
through the accepted immutable Dornglut reusable workflow.

See [TESTING.md](TESTING.md).

## Authority and provenance

- [Architecture](ARCHITECTURE.md)
- [Testing](TESTING.md)
- [Bootstrap and provenance](BOOTSTRAP.md)
- [Executor contract](AGENTS.md)
- [Organization contribution guidance](https://github.com/dornglut/.github/blob/main/CONTRIBUTING.md)
- [Organization security policy](https://github.com/dornglut/.github/blob/main/SECURITY.md)
- [Public license](LICENSE)
- [Commercial-license guidance](LICENSING.md)

## Contribution

Tracked-content contributions are currently `owner-only`. Issues, discussion,
reviews, and reproducible reports may still be used through the repository's
public channels. This posture remains until an accepted inbound mechanism
preserves the rights required for commercial licensing.

## License

RunenInput is publicly represented under [GPL-3.0-only](LICENSE). A separately
governed commercial licensing path is described in
[LICENSING.md](LICENSING.md).
