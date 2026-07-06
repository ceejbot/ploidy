//! End-to-end conformance tests: drive the real `ploidy` binary against the
//! vendored Swagger Petstore specs (`conformance/specs/`) and observe what it
//! generates and whether the output compiles.
//!
//! Both tests are `#[ignore]` because they shell out to `cargo` and build the
//! generated crate's full dependency tree (reqwest, tokio, …). Run them with:
//!
//! ```sh
//! cargo test -p ploidy --test conformance -- --ignored
//! ```
//!
//! The checked-in golden client (`conformance/petstore-v3-client/`) is the
//! reviewable snapshot of the v3 output; these tests instead prove the generator
//! still turns the *spec* into compiling code today, and pin the v31 limitation.

use std::fs;
use std::path::Path;
use std::process::Command;

/// The classic v3 spec generates a client that compiles against the in-tree
/// `ploidy-util`.
#[test]
#[ignore = "shells out to cargo and builds the generated crate"]
fn test_petstore_v3_generates_and_compiles() {
    let repo_root = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
    let spec = repo_root.join("conformance/specs/petstore-v3.json");
    let ploidy_util = repo_root.join("ploidy-util");
    let out = tempfile::tempdir().unwrap();

    let generated = Command::new(env!("CARGO_BIN_EXE_ploidy"))
        .args(["generate", "rust"])
        .arg(&spec)
        .arg("-o")
        .arg(out.path())
        .args(["--name", "petstore-v3-conformance"])
        .status()
        .unwrap();
    assert!(generated.success(), "ploidy failed to generate a v3 client");

    // Redirect the crates.io `ploidy-util` pin to this fork's in-tree crate so
    // the generated client builds against the runtime that produced it.
    let manifest = out.path().join("Cargo.toml");
    let mut contents = fs::read_to_string(&manifest).unwrap();
    contents.push_str(&format!(
        "\n[patch.crates-io]\nploidy-util = {{ path = \"{}\" }}\n",
        ploidy_util.display(),
    ));
    fs::write(&manifest, contents).unwrap();

    let checked = Command::new("cargo")
        .args(["check", "--all-targets"])
        .current_dir(out.path())
        .status()
        .unwrap();
    assert!(checked.success(), "generated v3 client failed to compile");
}

/// The 3.1 spec references `PetDetails` by an absolute-URI `$ref`, which ploidy
/// does not resolve. Generation fails with that specific diagnostic. When
/// external-reference support lands, flip this to assert success.
#[test]
#[ignore = "documents a current limitation; runs the ploidy binary"]
fn test_petstore_v31_external_ref_unsupported() {
    let repo_root = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
    let spec = repo_root.join("conformance/specs/petstore-v31.json");
    let out = tempfile::tempdir().unwrap();

    let output = Command::new(env!("CARGO_BIN_EXE_ploidy"))
        .args(["generate", "rust"])
        .arg(&spec)
        .arg("-o")
        .arg(out.path())
        .args(["--name", "petstore-v31-conformance"])
        .output()
        .unwrap();

    assert!(
        !output.status.success(),
        "expected v31 generation to fail on the external `$ref`; it succeeded",
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("external references aren't supported"),
        "expected the external-reference diagnostic; got:\n{stderr}",
    );
}
