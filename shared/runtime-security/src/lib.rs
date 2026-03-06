use std::path::{Path, PathBuf};

use strategy_sandbox::demo_strategy_wat;
use thiserror::Error;
use wasmcloud_bindings::{InterfaceBindingV1, WasmComponentBindingV1, WorkloadKindV1};

pub const RUNTIME_LAB_ROOT: &str = "testing/security-labs/runtime-security";

#[derive(Debug, Error, PartialEq, Eq)]
pub enum DescriptorError {
    #[error("component binding is not a service workload")]
    WrongWorkloadKind,
    #[error("component binding must expose the service world")]
    MissingServiceWorld,
    #[error("component binding must declare at least one config schema reference")]
    MissingSchemaReference,
}

#[must_use]
pub fn runtime_lab_path() -> PathBuf {
    workspace_root().join(RUNTIME_LAB_ROOT)
}

#[must_use]
pub fn runtime_lab_fixture_path(name: &str) -> PathBuf {
    runtime_lab_path().join("fixtures").join(name)
}

pub fn load_fixture(name: &str) -> Result<String, std::io::Error> {
    std::fs::read_to_string(runtime_lab_fixture_path(name))
}

pub fn oversized_state_wat() -> Result<String, std::io::Error> {
    let strategy_id = load_fixture("oversized-state-id.txt")?;
    Ok(demo_strategy_wat(strategy_id.trim()))
}

pub fn validate_service_binding(binding: &WasmComponentBindingV1) -> Result<(), DescriptorError> {
    if binding.workload_kind != WorkloadKindV1::Service {
        return Err(DescriptorError::WrongWorkloadKind);
    }
    if !binding
        .interfaces
        .iter()
        .any(|interface| interface == &InterfaceBindingV1::service_world())
    {
        return Err(DescriptorError::MissingServiceWorld);
    }
    if binding.config_schema_refs.is_empty() {
        return Err(DescriptorError::MissingSchemaReference);
    }
    Ok(())
}

fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("runtime-security workspace root")
        .to_path_buf()
}
