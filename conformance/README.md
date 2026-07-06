# Conformance suite

Drives ploidy against real-world OpenAPI specs to find, proactively, what it
doesn't yet handle — and to keep a reviewable snapshot of the code it emits.

| Path | What it is |
|------|------------|
| `specs/` | Vendored Swagger Petstore specs (v3 and v31) with provenance. |
| `petstore-v3-client/` | The v3 client **as ploidy currently emits it**, checked in. Its `tests/roundtrip.rs` are hermetic serde round-trip tests. Excluded from the root workspace so its `reqwest`/`tokio` tree stays out of the main build. |
| `GAPS.md` | What generates, compiles, and fails — a prioritized list of missing features. |

The dynamic generate-and-compile tests live in `../ploidy/tests/conformance.rs`.

## Commands (see the `justfile`)

- `just conformance` — regenerate the golden client, format it, fail if the
  emitted code drifted from what's checked in, then run its serde tests. A dirty
  `src` diff means codegen changed: review it, then `just conformance-accept` to
  take the new output.
- `just conformance-full` — the above plus the end-to-end tests that generate
  each spec into a throwaway crate and compile it (v3) / assert the v31
  limitation still holds.

## Why a golden crate

Ploidy emits ASTs, not templates, so the code it produces is the artifact worth
reviewing. Checking in the generated v3 client turns any codegen change into a
readable `git diff`, and gives serde round-trip tests a real crate to exercise
without a network round-trip. It compiles against this fork's in-tree
`ploidy-util` via a `path` dependency that regeneration preserves, and the
emitted code is clippy-clean. The crate is kept out of the root workspace only to
keep its `reqwest`/`tokio` tree out of the main build and MSRV matrix, not
because of lint noise.
