#![forbid(unsafe_code)]

//! Integration tests for `Simoder-SDK` covering:
//! - `#1`: Canonical Beta contracts matrix (`contracts-matrix.json` & `CONTRACTS.md`)
//! - `#2`: Declaration-vs-implementation conformance verifier (detecting missing and surplus declarations)

use std::fs;
use std::path::{Path, PathBuf};

use sha2::{Digest, Sha256};
use simoder_sdk_verifier::{
    SdkManifest, SdkVerifyError, compare_declaration_to_implementation,
    inspect_actual_base_implementation, load_and_validate_contracts_matrix,
    parse_base_luau_declarations, parse_base_ts_declarations, verify_base_conformance,
    verify_base_ts_conformance, verify_sdk_manifest_and_native_identity,
};

fn sdk_repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("Simoder-SDK root")
        .to_path_buf()
}

fn base_repo_root() -> PathBuf {
    sdk_repo_root()
        .parent()
        .expect("workspace parent")
        .join("Simoder-Base")
}

#[test]
fn sdk_manifest_and_native_identity_hashes_match_canonical_files() {
    let sdk_root = sdk_repo_root();
    let manifest =
        verify_sdk_manifest_and_native_identity(&sdk_root).expect("SDK manifest and Native identity");
    assert_eq!(manifest.version, "0.2.0");
    assert_eq!(
        manifest.native_contract,
        "00c98e36f669c6c4f16f7ffd9073130ae9798543b3d0759d7c1937f443e6e236"
    );
}

#[test]
fn beta_contracts_matrix_covers_luau_base_native_and_js_ts_with_owners_and_versions() {
    let sdk_root = sdk_repo_root();
    let matrix = load_and_validate_contracts_matrix(&sdk_root).expect("valid contracts-matrix.json");
    assert_eq!(matrix.sdk_version, "0.2.0");
    assert_eq!(matrix.pinned_sources.simoder_base.catalogue_contract, 2);
    assert!(matrix.contracts.len() >= 30);

    // Ensure every declared file in contracts-matrix.json exists and has non-empty version & owner
    for item in &matrix.contracts {
        assert!(!item.id.is_empty());
        assert!(!item.version.is_empty());
        assert!(!item.owner.is_empty());
        assert!(sdk_root.join(&item.declaration).is_file());
    }
}

#[test]
fn declaration_conformance_matches_actual_simoder_base_implementation_and_detects_missing_or_surplus() {
    let sdk_root = sdk_repo_root();
    let base_root = base_repo_root();

    let report =
        verify_base_conformance(&sdk_root, &base_root).expect("conformance between SDK and Base");
    assert_eq!(report.contract_name, "base-luau-alpha-v1");
    assert_eq!(report.sdk_version, "0.2.0");
    assert_eq!(report.base_version, "0.2.0-alpha");
    assert_eq!(report.catalogue_contract, 2);
    assert!(report.verified_items.contains(&"base.permissions.query".to_string()));
    assert!(report.verified_items.contains(&"base.values.validate".to_string()));
    assert!(report.verified_items.contains(&"base.request".to_string()));
    assert!(report.verified_items.contains(&"BaseTask.await".to_string()));

    // Negative check 1: Missing declaration in base.d.luau is detected
    let matrix = load_and_validate_contracts_matrix(&sdk_root).unwrap();
    let d_luau = fs::read_to_string(sdk_root.join("Luau/base.d.luau")).unwrap();
    let mut declared_missing = parse_base_luau_declarations(&d_luau);
    let implemented = inspect_actual_base_implementation(&base_root).unwrap();
    declared_missing.remove("base.permissions.query");

    let err_missing = compare_declaration_to_implementation(
        "base-luau-alpha-v1",
        &matrix,
        &declared_missing,
        &implemented,
    )
    .expect_err("missing declaration must fail conformance");
    assert_eq!(
        err_missing,
        SdkVerifyError::ConformanceMismatch {
            contract: "base-luau-alpha-v1".to_string(),
            missing: vec!["base.permissions.query".to_string()],
            surplus: Vec::new(),
        }
    );

    // Negative check 2: Surplus/unimplemented declaration in base.d.luau is detected
    let mut declared_surplus = parse_base_luau_declarations(&d_luau);
    declared_surplus.insert("base.unimplementedTeleport".to_string());
    let err_surplus = compare_declaration_to_implementation(
        "base-luau-alpha-v1",
        &matrix,
        &declared_surplus,
        &implemented,
    )
    .expect_err("surplus declaration must fail conformance");
    assert_eq!(
        err_surplus,
        SdkVerifyError::ConformanceMismatch {
            contract: "base-luau-alpha-v1".to_string(),
            missing: Vec::new(),
            surplus: vec!["base.unimplementedTeleport".to_string()],
        }
    );

    // Verify sdk-manifest.json has the updated hashes for base.d.luau, BASE_API.md, and TypeScript/base.d.ts
    let manifest_raw = fs::read_to_string(sdk_root.join("sdk-manifest.json")).unwrap();
    let manifest: SdkManifest = serde_json::from_str(&manifest_raw).unwrap();
    for path in ["Luau/base.d.luau", "Luau/BASE_API.md", "TypeScript/base.d.ts"] {
        let actual = hex::encode(Sha256::digest(fs::read(sdk_root.join(path)).unwrap()));
        assert_eq!(manifest.files.get(path).unwrap(), &actual);
    }

    // Verify TypeScript/base.d.ts ALSO matches actual Simoder-Base facade with zero missing/surplus symbols
    let ts_report = verify_base_ts_conformance(&sdk_root, &base_root)
        .expect("TypeScript/base.d.ts conformance with Simoder-Base");
    assert_eq!(ts_report.contract_name, "base-ts-beta-v1");
    assert!(ts_report.verified_items.contains(&"base.permissions.query".to_string()));
    assert!(ts_report.verified_items.contains(&"base.snapshot".to_string()));
    assert!(ts_report.verified_items.contains(&"BaseTask.await".to_string()));

    let ts_decl = parse_base_ts_declarations(
        &fs::read_to_string(sdk_root.join("TypeScript/base.d.ts")).unwrap(),
    );
    assert_eq!(ts_decl, implemented);
}
