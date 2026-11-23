#![forbid(unsafe_code)]

//! Manifest (`sdk-manifest.json`) and Native ABI identity (`Native/identity.json`) verifier.

use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::matrix::SdkVerifyError;

/// Profile metadata in `sdk-manifest.json`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SdkProfileDescriptor {
    /// Target language (`luau`, `typescript`, `c`).
    pub language: String,
    /// Relative declaration path.
    pub declaration: String,
    /// Relative documentation guide path.
    pub guide: String,
    /// Host availability requirement.
    pub availability: String,
    /// Stability tier (`alpha`, `beta`).
    pub compatibility: String,
}

/// Parsed `sdk-manifest.json`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SdkManifest {
    /// SDK version (`0.2.0`).
    pub version: String,
    /// Upstream origin repository.
    pub upstream_repository: String,
    /// Upstream revision hash.
    pub upstream_revision: String,
    /// Map of relative file path -> SHA-256 hex digest.
    pub files: BTreeMap<String, String>,
    /// Canonical native ABI v2 bundle digest.
    pub native_contract: String,
    /// Published SDK profiles.
    pub profiles: BTreeMap<String, SdkProfileDescriptor>,
}

/// Parsed `Native/identity.json`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NativeIdentity {
    /// Digest algorithm description.
    pub algorithm: String,
    /// Map of normative input filename -> SHA-256 hex digest.
    pub input_hashes: BTreeMap<String, String>,
    /// Canonical SHA-256 digest of the compact sorted JSON `input_hashes` map.
    pub digest: String,
}

/// Verify `sdk-manifest.json` and `Native/identity.json` against the files on disk.
pub fn verify_sdk_manifest_and_native_identity(sdk_root: &Path) -> Result<SdkManifest, SdkVerifyError> {
    let manifest_raw = fs::read_to_string(sdk_root.join("sdk-manifest.json"))
        .map_err(|e| SdkVerifyError::Io(e.to_string()))?;
    let manifest: SdkManifest = serde_json::from_str(&manifest_raw)
        .map_err(|e| SdkVerifyError::ManifestDrift(e.to_string()))?;

    if manifest.version != "0.2.0" {
        return Err(SdkVerifyError::ManifestDrift(format!(
            "unexpected SDK version: {}",
            manifest.version
        )));
    }

    for (rel_path, expected_sha) in &manifest.files {
        let full = sdk_root.join(rel_path);
        let bytes = fs::read(&full).map_err(|e| {
            SdkVerifyError::ManifestDrift(format!("missing file {rel_path}: {e}"))
        })?;
        let actual_sha = hex::encode(Sha256::digest(&bytes));
        if !actual_sha.eq_ignore_ascii_case(expected_sha) {
            return Err(SdkVerifyError::ManifestDrift(format!(
                "hash drift for {rel_path}: expected {expected_sha}, got {actual_sha}"
            )));
        }
    }

    let identity_raw = fs::read_to_string(sdk_root.join("Native/identity.json"))
        .map_err(|e| SdkVerifyError::Io(e.to_string()))?;
    let identity: NativeIdentity = serde_json::from_str(&identity_raw)
        .map_err(|e| SdkVerifyError::ManifestDrift(e.to_string()))?;

    let native_dir = sdk_root.join("Native");
    for (name, expected_sha) in &identity.input_hashes {
        let bytes = fs::read(native_dir.join(name)).map_err(|e| {
            SdkVerifyError::ManifestDrift(format!("missing Native input {name}: {e}"))
        })?;
        let actual_sha = hex::encode(Sha256::digest(&bytes));
        if !actual_sha.eq_ignore_ascii_case(expected_sha) {
            return Err(SdkVerifyError::ManifestDrift(format!(
                "Native input drift for {name}: expected {expected_sha}, got {actual_sha}"
            )));
        }
    }

    let canonical_json =
        serde_json::to_string(&identity.input_hashes).map_err(|e| SdkVerifyError::Io(e.to_string()))?;
    let computed_digest = hex::encode(Sha256::digest(canonical_json.as_bytes()));
    if !computed_digest.eq_ignore_ascii_case(&identity.digest)
        || !computed_digest.eq_ignore_ascii_case(&manifest.native_contract)
    {
        return Err(SdkVerifyError::ManifestDrift(format!(
            "native contract identity digest mismatch: computed {computed_digest}, identity={}, manifest={}",
            identity.digest, manifest.native_contract
        )));
    }

    Ok(manifest)
}
