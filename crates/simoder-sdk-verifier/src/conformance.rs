#![forbid(unsafe_code)]

//! Automated declaration-vs-implementation conformance verifier (`Simoder-SDK#2`).
//!
//! Executes the actual `Simoder-Base` `Facade.create(host)` and `TaskFacade.create(native, handle)`
//! in an embedded Luau 0.654 VM, introspects the exported runtime surface, and compares it
//! against `Luau/base.d.luau` and `contracts-matrix.json`. Detects any missing or surplus declaration.

use std::collections::BTreeSet;
use std::fs;
use std::path::Path;

use mlua::{Lua, MultiValue, Table, Value};
use serde::{Deserialize, Serialize};

use crate::matrix::{ContractStatus, ContractsMatrix, SdkVerifyError};

/// Structured report emitted by the declaration-vs-implementation conformance verifier.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConformanceReport {
    /// Verified contract profile (`base-luau-alpha-v1`).
    pub contract_name: String,
    /// SDK version (`0.2.0`).
    pub sdk_version: String,
    /// `Simoder-Base` version from `mod.toml` (`0.2.0-alpha`).
    pub base_version: String,
    /// Catalogue contract integer (`2`).
    pub catalogue_contract: u32,
    /// Sorted list of items verified across both declaration and implementation.
    pub verified_items: Vec<String>,
    /// Items exported by the implementation but missing from the SDK declaration.
    pub missing_declarations: Vec<String>,
    /// Items declared in the SDK as supported but absent from the implementation.
    pub surplus_declarations: Vec<String>,
}

/// Extract declared `base.*` and `BaseTask.*` symbols from `Luau/base.d.luau`.
pub fn parse_base_luau_declarations(base_d_luau_text: &str) -> BTreeSet<String> {
    let mut declared = BTreeSet::new();
    let mut current_block: Option<&str> = None;
    let mut nested_prefix: Option<&str> = None;

    for raw_line in base_d_luau_text.lines() {
        let line = raw_line.trim();
        if line.starts_with("--") || line.is_empty() {
            continue;
        }
        if line.starts_with("type BaseTask = {") {
            current_block = Some("BaseTask");
            nested_prefix = None;
            continue;
        }
        if line.starts_with("type BaseFacade = {") {
            current_block = Some("base");
            nested_prefix = None;
            continue;
        }
        if line == "}" {
            if nested_prefix.is_some() {
                nested_prefix = None;
            } else {
                current_block = None;
            }
            continue;
        }
        if line == "}," {
            nested_prefix = None;
            continue;
        }
        if let Some(block) = current_block {
            if line.ends_with(": {") {
                let sub = line.trim_end_matches(": {").trim();
                nested_prefix = match (block, sub) {
                    ("base", "permissions") => Some("base.permissions"),
                    ("base", "values") => Some("base.values"),
                    _ => None,
                };
                continue;
            }
            if let Some((field, _rest)) = line.split_once(':') {
                let field_name = field.trim();
                if field_name
                    .chars()
                    .all(|c| c.is_ascii_alphanumeric() || c == '_')
                {
                    if let Some(prefix) = nested_prefix {
                        declared.insert(format!("{prefix}.{field_name}"));
                    } else {
                        declared.insert(format!("{block}.{field_name}"));
                    }
                }
            }
        }
    }
    declared
}

/// Extract declared `base.*` and `BaseTask.*` symbols from `TypeScript/base.d.ts`.
pub fn parse_base_ts_declarations(base_d_ts_text: &str) -> BTreeSet<String> {
    let mut declared = BTreeSet::new();
    let mut current_block: Option<&str> = None;
    let mut nested_prefix: Option<&str> = None;

    for raw_line in base_d_ts_text.lines() {
        let line = raw_line.trim();
        if line.is_empty()
            || line.starts_with("//")
            || line.starts_with("/*")
            || line.starts_with('*')
        {
            continue;
        }
        if line.starts_with("interface BaseTask {") {
            current_block = Some("BaseTask");
            nested_prefix = None;
            continue;
        }
        if line.starts_with("interface BaseFacade {") {
            current_block = Some("base");
            nested_prefix = None;
            continue;
        }
        if line == "}" || line == "};" {
            if nested_prefix.is_some() {
                nested_prefix = None;
            } else {
                current_block = None;
            }
            continue;
        }
        if let Some(block) = current_block {
            if block == "base" && line == "readonly permissions: {" {
                nested_prefix = Some("base.permissions");
                continue;
            }
            if block == "base" && line == "readonly values: {" {
                nested_prefix = Some("base.values");
                continue;
            }
            if let Some((method_name, _rest)) = line.split_once('(') {
                let method_name = method_name.trim();
                if !method_name.is_empty()
                    && method_name
                        .chars()
                        .all(|c| c.is_ascii_alphanumeric() || c == '_')
                {
                    let target_prefix = nested_prefix.unwrap_or(block);
                    declared.insert(format!("{target_prefix}.{method_name}"));
                }
            }
        }
    }
    declared
}

/// Introspect the actual exported `base.*` and `BaseTask.*` callable surface by running
/// `Simoder-Base/scripts/lib/facade.luau` inside a real Luau 0.654 VM.
pub fn inspect_actual_base_implementation(base_root: &Path) -> Result<BTreeSet<String>, SdkVerifyError> {
    let lua = Lua::new();
    let scripts_lib = base_root.join("scripts/lib");
    let capability_src = fs::read_to_string(scripts_lib.join("capability.luau"))
        .map_err(|e| SdkVerifyError::Io(e.to_string()))?;
    let task_facade_src = fs::read_to_string(scripts_lib.join("task-facade.luau"))
        .map_err(|e| SdkVerifyError::Io(e.to_string()))?;
    let ui_kit_src = fs::read_to_string(scripts_lib.join("ui-kit.luau"))
        .unwrap_or_else(|_| "return {}".to_string());
    let facade_src = fs::read_to_string(scripts_lib.join("facade.luau"))
        .map_err(|e| SdkVerifyError::Io(e.to_string()))?;

    let req_fn = lua
        .create_function(move |lua_ctx, spec: String| match spec.as_str() {
            "./capability" | "../lib/capability" => lua_ctx
                .load(&capability_src)
                .set_name("capability.luau")
                .eval::<MultiValue>(),
            "./task-facade" | "../lib/task-facade" => lua_ctx
                .load(&task_facade_src)
                .set_name("task-facade.luau")
                .eval::<MultiValue>(),
            "./ui-kit" | "../lib/ui-kit" => lua_ctx
                .load(&ui_kit_src)
                .set_name("ui-kit.luau")
                .eval::<MultiValue>(),
            other => Err(mlua::Error::RuntimeError(format!(
                "unexpected require in facade introspection: {other}"
            ))),
        })
        .map_err(|e| SdkVerifyError::MatrixInvalid(e.to_string()))?;
    lua.globals()
        .set("require", req_fn)
        .map_err(|e| SdkVerifyError::MatrixInvalid(e.to_string()))?;

    let facade_mod: Table = lua
        .load(&facade_src)
        .set_name("facade.luau")
        .eval()
        .map_err(|e| SdkVerifyError::MatrixInvalid(e.to_string()))?;

    let probe_script = r#"
        local Facade = ...
        local dummyTask = {}
        local host = {
            values = { validate = function() return {} end, wire = function() return "" end },
            tasks = { poll = function() end, cancel = function() end, observe = function() end, await = function() end },
            lifecycle = { snapshot = function() return {} end, subscribe = function() return {} end },
            registry = { descriptors = function() return {}, 2 end },
            runtime = { request = function() return dummyTask end, run = function() return true end },
            permissions = { query = function() return { available = true, selected = true, granted = true, effective = false, restricted = true } end },
        }
        local instance = Facade.create(host)
        local taskInstance = instance.request("probe", {})
        return instance, taskInstance
    "#;

    let (facade_table, task_table): (Table, Table) = lua
        .load(probe_script)
        .call(facade_mod)
        .map_err(|e| SdkVerifyError::MatrixInvalid(e.to_string()))?;

    let mut exported = BTreeSet::new();
    for pair in facade_table.pairs::<String, Value>() {
        let (k, v) = pair.map_err(|e| SdkVerifyError::MatrixInvalid(e.to_string()))?;
        match v {
            Value::Function(_) => {
                exported.insert(format!("base.{k}"));
            }
            Value::Table(sub) => {
                for sub_pair in sub.pairs::<String, Value>() {
                    let (sk, sv) =
                        sub_pair.map_err(|e| SdkVerifyError::MatrixInvalid(e.to_string()))?;
                    if matches!(sv, Value::Function(_)) {
                        exported.insert(format!("base.{k}.{sk}"));
                    }
                }
            }
            _ => {}
        }
    }

    for pair in task_table.pairs::<String, Value>() {
        let (k, v) = pair.map_err(|e| SdkVerifyError::MatrixInvalid(e.to_string()))?;
        if matches!(v, Value::Function(_)) {
            exported.insert(format!("BaseTask.{k}"));
        }
    }

    Ok(exported)
}

/// Compare the declared `base-luau-alpha-v1` SDK surface (`Luau/base.d.luau` + `contracts-matrix.json`)
/// against an exported implementation surface, failing when any missing or surplus symbol is detected.
pub fn compare_declaration_to_implementation(
    contract_name: &str,
    matrix: &ContractsMatrix,
    declared_in_d_luau: &BTreeSet<String>,
    implemented_in_base: &BTreeSet<String>,
) -> Result<ConformanceReport, SdkVerifyError> {
    let matrix_supported: BTreeSet<String> = matrix
        .contracts
        .iter()
        .filter(|c| c.profile == contract_name && c.status == ContractStatus::Supported)
        .map(|c| {
            if contract_name == "base-ts-beta-v1" {
                c.id.strip_prefix("js.").unwrap_or(&c.id).to_string()
            } else {
                c.id.clone()
            }
        })
        .collect();

    let mut declared = declared_in_d_luau.clone();
    declared.extend(matrix_supported.iter().cloned());

    let missing_from_d_luau: Vec<String> = implemented_in_base
        .difference(declared_in_d_luau)
        .cloned()
        .collect();
    let missing_from_matrix: Vec<String> = implemented_in_base
        .difference(&matrix_supported)
        .cloned()
        .collect();
    let mut missing = BTreeSet::new();
    missing.extend(missing_from_d_luau);
    missing.extend(missing_from_matrix);

    let surplus_in_d_luau: Vec<String> = declared_in_d_luau
        .difference(implemented_in_base)
        .cloned()
        .collect();
    let surplus_in_matrix: Vec<String> = matrix_supported
        .difference(implemented_in_base)
        .cloned()
        .collect();
    let mut surplus = BTreeSet::new();
    surplus.extend(surplus_in_d_luau);
    surplus.extend(surplus_in_matrix);

    let missing_vec: Vec<String> = missing.into_iter().collect();
    let surplus_vec: Vec<String> = surplus.into_iter().collect();

    if !missing_vec.is_empty() || !surplus_vec.is_empty() {
        return Err(SdkVerifyError::ConformanceMismatch {
            contract: contract_name.to_string(),
            missing: missing_vec,
            surplus: surplus_vec,
        });
    }

    Ok(ConformanceReport {
        contract_name: contract_name.to_string(),
        sdk_version: matrix.sdk_version.clone(),
        base_version: matrix.pinned_sources.simoder_base.version.clone(),
        catalogue_contract: matrix.pinned_sources.simoder_base.catalogue_contract,
        verified_items: implemented_in_base.iter().cloned().collect(),
        missing_declarations: Vec::new(),
        surplus_declarations: Vec::new(),
    })
}

/// Run the full offline conformance check between `Simoder-SDK` (`Luau/base.d.luau`) and `Simoder-Base`.
pub fn verify_base_conformance(
    sdk_root: &Path,
    base_root: &Path,
) -> Result<ConformanceReport, SdkVerifyError> {
    let matrix = crate::matrix::load_and_validate_contracts_matrix(sdk_root)?;
    let d_luau = fs::read_to_string(sdk_root.join("Luau/base.d.luau"))
        .map_err(|e| SdkVerifyError::Io(e.to_string()))?;
    let declared = parse_base_luau_declarations(&d_luau);
    let implemented = inspect_actual_base_implementation(base_root)?;
    compare_declaration_to_implementation("base-luau-alpha-v1", &matrix, &declared, &implemented)
}

/// Run the full offline conformance check between `Simoder-SDK` (`TypeScript/base.d.ts`) and `Simoder-Base`.
pub fn verify_base_ts_conformance(
    sdk_root: &Path,
    base_root: &Path,
) -> Result<ConformanceReport, SdkVerifyError> {
    let matrix = crate::matrix::load_and_validate_contracts_matrix(sdk_root)?;
    let d_ts = fs::read_to_string(sdk_root.join("TypeScript/base.d.ts"))
        .map_err(|e| SdkVerifyError::Io(e.to_string()))?;
    let declared = parse_base_ts_declarations(&d_ts);
    let implemented = inspect_actual_base_implementation(base_root)?;
    compare_declaration_to_implementation("base-ts-beta-v1", &matrix, &declared, &implemented)
}
