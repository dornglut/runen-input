# RunenInput testing and validation

## Canonical command

```text
cargo validate
```

This command is implemented by the repository-local `xtask` and is the
merge-readiness baseline for the RunenInput repository.

## Bootstrap baseline checks

Validation fails closed when:

- a required repository authority file is missing;
- package/repository identity is not `runen-input` / `dornglut/runen-input`;
- Cargo public license metadata is not `GPL-3.0-only`;
- the current license files do not represent GPL-3.0-only plus the separate
  commercial-license guidance;
- active crate/workflow identity still claims to be the generic framework
  template;
- Rust formatting is not clean;
- locked workspace tests fail;
- Clippy emits warnings;
- rustdoc emits warnings;
- Git whitespace checks fail;
- validation changes repository state.

The validator starts from a clean repository and verifies that the repository
remains unchanged after the checks.

## Current proof boundary

Bootstrap intentionally has no transferred input implementation and therefore no
device-input conformance portfolio yet. The root crate is semantic-empty.

The repository-local successor-transfer issue must establish focused standalone
conformance for the transferred observation/reducer semantics before successor
acceptance. Bootstrap CI must not be misrepresented as proof of the future input
framework implementation.

## CI

The workflow in `.github/workflows/validation.yml` is intentionally thin. It
pins the accepted `dornglut/github-workflows` reusable Rust validation workflow
to an immutable commit and delegates validation meaning to
`cargo +stable validate`.

The reusable workflow proves the exact caller revision before validation and
provisions stable plus Cargo-declared Rust versions required by the checked-out
repository.

Local validation is preparation. Pull-request acceptance requires independent
repository-owned CI against the exact reviewed feature head. Accepted-main
validation is required when the owning bootstrap or transfer authority calls for
it.
