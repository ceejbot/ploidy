# Ploidy task runner. Run `just` to list recipes.

_default:
    @just --list

# Regenerate the golden Petstore v3 client, format it, fail if the emitted code
# drifted from what's checked in, then run its serde round-trip tests. A dirty
# `src` diff means codegen changed: review it and commit with `just conformance-accept`.
conformance:
    cargo run -q -p ploidy -- generate rust conformance/specs/petstore-v3.json -o conformance/petstore-v3-client --name petstore-v3-client
    cargo +nightly fmt --manifest-path conformance/petstore-v3-client/Cargo.toml --all
    git diff --exit-code -- conformance/petstore-v3-client/src
    cargo test --manifest-path conformance/petstore-v3-client/Cargo.toml

# Regenerate and format the golden client, leaving the diff staged for review.
# Use when a codegen change intentionally alters the emitted client.
conformance-accept:
    cargo run -q -p ploidy -- generate rust conformance/specs/petstore-v3.json -o conformance/petstore-v3-client --name petstore-v3-client
    cargo +nightly fmt --manifest-path conformance/petstore-v3-client/Cargo.toml --all

# Run the full end-to-end conformance tests: generate each vendored spec into a
# throwaway crate and compile it (v3), and assert the v31 limitation still holds.
# Heavier than `conformance`; builds the generated dependency tree.
conformance-full: conformance
    cargo test -p ploidy --test conformance -- --ignored
