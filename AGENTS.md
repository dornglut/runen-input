# RunenInput executor contract

Start with `README.md`, `ARCHITECTURE.md`, `TESTING.md`, `BOOTSTRAP.md`, and
the current owning issue before changing repository content.

## Current authority state

This repository is the bootstrap-stage successor for RunenInput. It does not yet
own the reusable device-input implementation.

Until a later successor-transfer pull request is accepted on `main` under
Engineering ADR 0008, `dornglut/runenwerk` remains the sole semantic source
authority for the transferable neutral-input implementation.

## Durable constraints

- Keep RunenInput host- and backend-neutral.
- Keep native acquisition, winit/OS integration, Runenwerk App/Host lifecycle,
  RunenECS scheduling, product actions/bindings, RunenUI routing/focus/text,
  Draw behavior, and product camera policy outside this repository.
- Preserve one semantic authority per concern and one-way dependencies.
- Do not add compatibility aliases, forwarding modules/packages, source includes,
  submodules, mirrors, moving-branch dependencies, or duplicate reducers.
- Do not move predecessor source without a repository-local successor-transfer
  issue and exact accepted provenance.
- Do not stabilize speculative device families, persistence/replay formats,
  network protocols, or a universal action framework during extraction.
- Keep tracked-content contributions `owner-only` until an accepted inbound
  contribution mechanism preserves the rights required for commercial licensing.
- Keep `cargo validate` as the canonical repository-owned validation command.
- Keep CI a thin, read-only caller of repository-owned validation.

## Delivery workflow

1. Resolve current `main`, current authority, open writers, and the owning issue.
2. Audit the complete dependency and provenance closure for the proposed change.
3. Construct one complete candidate from the exact accepted base.
4. Validate the exact candidate through repository-owned CI.
5. Reconcile current `main`, review state, settings, and complete diff before
   guarded squash merge.
6. Verify accepted `main` when the owning acceptance contract requires it.

The historical framework template is one-time bootstrap provenance, not an
ongoing synchronization or architecture authority.
