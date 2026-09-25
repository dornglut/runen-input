# RunenInput testing and validation

## Canonical command

```text
cargo validate
```

The repository-local `xtask` owns merge-readiness semantics.

## Validation portfolio

The validator covers:

- required authority and provenance files;
- package/repository/license identity;
- recursive, fail-closed scans of the complete Rust product and independent conformance source closures;
- standalone source/dependency boundaries, including the std-only product semantic core;
- required canonical executable example presence;
- the public observation/state contract;
- all 15 transferred focused semantic/reducer laws;
- focused source/device continuity-loss laws;
- root public-contract integration tests;
- independent `conformance/downstream` public-API tests;
- root and downstream rustfmt;
- locked workspace and downstream tests;
- strict root and downstream Clippy;
- rustdoc with warnings denied and crate-level missing-public-docs enforcement;
- Rust 1.93.0 checks for root and downstream packages;
- Git whitespace checks;
- unchanged repository state.

## Transferred semantic laws

The focused reducer tests under `src/state/tests.rs` prove deterministic source/admission ordering,
down/up admission identity, keyboard and pointer correlation, source/device
separation, aggregate held state, repeat/reconciliation behavior, contact
identity, explicit absent scroll axes, omitted-vs-zero measurement semantics,
atomic invalid-group rejection, predicted/estimated tablet behavior,
historical/coalesced tablet non-mutation of current confirmed state, tri-state tablet
capability knowledge and unsupported-evidence rejection, source-time context/unit
validation, stale contact clearing, and invalid tablet measurement atomicity.

Additional focused continuity laws prove source-scoped invalidation, device-scoped
sibling preservation, source-pointer invalidation rules, repeated-loss state
idempotence, reconciliation after loss, and atomic rejection of device loss
without a device-bearing context.

## Independent downstream proof

`conformance/downstream` is a separate Cargo workspace with one dependency:
the public `runen-input` package through `path = "../.."`.

It proves multi-context key state, aggregate release behavior, reconciliation,
pointer-button state, direct canonical scroll/contact/tablet payload admission,
tri-state tablet capability knowledge, explicit unsupported-evidence rejection,
standard `InputError` diagnostics/error traits, source-time context/unit validation,
relative-motion/absolute-position separation,
historical/predicted/estimated tablet non-mutation, scoped continuity loss, and contact state
without Runenwerk, winit, RunenECS, RunenUI, or private-module access.

## CI

`.github/workflows/validation.yml` remains a thin immutable caller of the
accepted shared Rust validation workflow. Local execution is preparation;
acceptance requires repository-owned CI on the exact reviewed feature head and,
for the initial authority transfer, accepted-main validation after merge.
