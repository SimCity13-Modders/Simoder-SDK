#![forbid(unsafe_code)]

//! Canonical Beta API/ABI contracts matrix model and validator (`Simoder-SDK#1`).

use std::collections::BTreeSet;
use std::fs;
use std::path::Path;

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Errors returned while validating `contracts-matrix.json` or SDK conformance.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum SdkVerifyError {
    /// Matrix JSON or manifest JSON failed schema parsing or validation.
    #[error("SDK matrix validation error: {0}")]
    MatrixInvalid(String),
    /// Manifest hash or native identity digest drifted from source files.
    #[error("SDK manifest integrity error: {0}")]
    ManifestDrift(String),
    /// Public SDK declaration diverged from the actual `Simoder-Base` implementation.
    #[error("SDK conformance mismatch in contract '{contract}': missing={missing:?}, surplus={surplus:?}")]
    ConformanceMismatch {
        /// Contract profile identifier (`base-luau-alpha-v1`, etc.).
        contract: String,
        /// Items exported by the implementation but missing from SDK declarations.
        missing: Vec<String>,
        /// Items declared as supported in the SDK but absent from the implementation.
        surplus: Vec<String>,
    },
    /// Filesystem I/O failure.
    #[error("I/O error: {0}")]
    Io(String),
}

/// Availability status of a contract item in the Beta matrix.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ContractStatus {
    /// Implemented, tested, and supported in the pinned Beta profile.
    Supported,
    /// Reserved in layout/types or restricted to internal host lanes; unavailable to mods.
    Unavailable,
}

/// One entry in `contracts-matrix.json`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ContractEntry {
    /// Dotted contract item identifier (e.g., `base.permissions.query`, `SimoderModule_QueryV2`).
    pub id: String,
    /// Domain family (`luau`, `base-luau`, `native-abi`, `js-ts`).
    pub domain: String,
    /// Profile identifier (`simoder-luau-v1`, `base-luau-alpha-v1`, `native-abi-v2`, `js-ts-beta-v1`).
    pub profile: String,
    /// Semantic version of the contract item.
    pub version: String,
    /// Authoritative owning repository (`Simoder`, `Simoder-Base`).
    pub owner: String,
    /// Supported or unavailable status.
    pub status: ContractStatus,
    /// Relative path to the declaration file in `Simoder-SDK`.
    pub declaration: String,
}

/// Pinned `Simoder` runtime metadata inside `contracts-matrix.json`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PinnedSimoderSource {
    /// Repository slug (`SimCity13-Modders/Simoder`).
    pub repository: String,
    /// Branch or commit reference.
    pub branch: String,
    /// Exact 64-hex SHA-256 digest of the native ABI v2 bundle.
    #[serde(rename = "nativeContractSha256")]
    pub native_contract_sha256: String,
}

/// Pinned `Simoder-Base` metadata inside `contracts-matrix.json`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PinnedBaseSource {
    /// Repository slug (`SimCity13-Modders/Simoder-Base`).
    pub repository: String,
    /// Base version (`0.2.0-alpha`).
    pub version: String,
    /// Required catalogue contract integer (`2`).
    #[serde(rename = "catalogueContract")]
    pub catalogue_contract: u32,
}

/// Container for pinned upstream repositories in `contracts-matrix.json`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PinnedSources {
    /// Pinned `Simoder` host information.
    pub simoder: PinnedSimoderSource,
    /// Pinned `Simoder-Base` semantic package information.
    #[serde(rename = "simoderBase")]
    pub simoder_base: PinnedBaseSource,
}

/// Top-level `contracts-matrix.json` document.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ContractsMatrix {
    /// Schema version (`1`).
    #[serde(rename = "schemaVersion")]
    pub schema_version: u32,
    /// SDK version (`0.2.0`).
    #[serde(rename = "sdkVersion")]
    pub sdk_version: String,
    /// Pinned upstream source metadata.
    #[serde(rename = "pinnedSources")]
    pub pinned_sources: PinnedSources,
    /// Complete list of API/ABI contract entries.
    pub contracts: Vec<ContractEntry>,
}

/// Load and validate `contracts-matrix.json` from the `Simoder-SDK` root.
pub fn load_and_validate_contracts_matrix(sdk_root: &Path) -> Result<ContractsMatrix, SdkVerifyError> {
    let raw = fs::read_to_string(sdk_root.join("contracts-matrix.json"))
        .map_err(|e| SdkVerifyError::Io(e.to_string()))?;
    let matrix: ContractsMatrix =
        serde_json::from_str(&raw).map_err(|e| SdkVerifyError::MatrixInvalid(e.to_string()))?;

    if matrix.schema_version != 1 || matrix.sdk_version != "0.2.0" {
        return Err(SdkVerifyError::MatrixInvalid(
            "unexpected schemaVersion or sdkVersion".to_string(),
        ));
    }
    if matrix.pinned_sources.simoder_base.catalogue_contract != 2 {
        return Err(SdkVerifyError::MatrixInvalid(
            "pinned Simoder-Base catalogueContract must be 2".to_string(),
        ));
    }

    let mut seen_ids = BTreeSet::new();
    let mut domains = BTreeSet::new();
    for entry in &matrix.contracts {
        if entry.id.is_empty() || entry.version.is_empty() || entry.owner.is_empty() {
            return Err(SdkVerifyError::MatrixInvalid(
                "contract entry has empty id, version, or owner".to_string(),
            ));
        }
        if !matches!(
            entry.domain.as_str(),
            "luau" | "base-luau" | "native-abi" | "js-ts"
        ) {
            return Err(SdkVerifyError::MatrixInvalid(format!(
                "unknown contract domain: {}",
                entry.domain
            )));
        }
        if !seen_ids.insert(entry.id.clone()) {
            return Err(SdkVerifyError::MatrixInvalid(format!(
                "duplicate contract entry id: {}",
                entry.id
            )));
        }
        if !sdk_root.join(&entry.declaration).is_file() {
            return Err(SdkVerifyError::MatrixInvalid(format!(
                "missing declaration file for {}: {}",
                entry.id, entry.declaration
            )));
        }
        domains.insert(entry.domain.clone());
    }

    for required_domain in ["luau", "base-luau", "native-abi", "js-ts"] {
        if !domains.contains(required_domain) {
            return Err(SdkVerifyError::MatrixInvalid(format!(
                "contracts matrix is missing required domain: {required_domain}"
            )));
        }
    }

    Ok(matrix)
}
