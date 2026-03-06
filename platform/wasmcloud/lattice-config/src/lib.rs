use serde::{Deserialize, Serialize};
use wasmcloud_bindings::{CapabilityBindingV1, SignedComponentRefV1, WasmComponentBindingV1};

const FINANCE_SERVICE_NAME: &str = "finance-service";
const FINANCE_COMPONENT_REPOSITORY: &str = "ghcr.io/shortorigin/finance-service";
const FINANCE_DEFAULT_COMPONENT_TAG: &str = "wasm";
const FINANCE_DEFAULT_COMPONENT_DIGEST: &str = "sha256:finance-service";

const TREASURY_DISBURSEMENT_NAME: &str = "treasury-disbursement";
const TREASURY_COMPONENT_REPOSITORY: &str = "ghcr.io/shortorigin/treasury-disbursement";
const TREASURY_DEFAULT_COMPONENT_TAG: &str = "wasm";
const TREASURY_DEFAULT_COMPONENT_DIGEST: &str = "sha256:treasury-disbursement";

const PLATFORM_WIT_PATH: &str = "schemas/wit/v1/platform.wit";
const TREASURY_CONTRACT_PATH: &str = "schemas/contracts/v1/treasury-disbursement-v1.json";
const SURREALDB_PROVIDER_ID: &str = "wasmcloud:keyvalue";
const SURREALDB_CONTRACT_ID: &str = "keyvalue";
const SURREALDB_LINK_NAME: &str = "surrealdb";
const HTTP_PROVIDER_ID: &str = "wasmcloud:httpserver";
const HTTP_CONTRACT_ID: &str = "http";
const HTTP_LINK_NAME: &str = "default";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RolloutTargetV1 {
    pub environment: String,
    pub namespace: String,
    pub policy_group: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LatticeConfigV1 {
    pub lattice_name: String,
    pub rollout: RolloutTargetV1,
    pub components: Vec<WasmComponentBindingV1>,
}

impl LatticeConfigV1 {
    #[must_use]
    pub fn component_refs(&self) -> Vec<&SignedComponentRefV1> {
        self.components
            .iter()
            .map(|component| &component.component)
            .collect()
    }
}

#[must_use]
pub fn finance_service_component_binding() -> WasmComponentBindingV1 {
    finance_service_component_binding_with_artifact(
        format!("{FINANCE_COMPONENT_REPOSITORY}:{FINANCE_DEFAULT_COMPONENT_TAG}"),
        FINANCE_DEFAULT_COMPONENT_DIGEST,
        "prod",
    )
}

#[must_use]
pub fn finance_service_component_binding_with_artifact(
    component_ref: impl Into<String>,
    digest: impl Into<String>,
    rollout_environment: impl Into<String>,
) -> WasmComponentBindingV1 {
    WasmComponentBindingV1::service(
        FINANCE_SERVICE_NAME,
        signed_component_ref(component_ref, digest),
        rollout_environment,
        platform_contract_paths(),
        vec![surrealdb_capability_binding()],
    )
}

#[must_use]
pub fn treasury_disbursement_component_binding() -> WasmComponentBindingV1 {
    treasury_disbursement_component_binding_with_artifact(
        format!("{TREASURY_COMPONENT_REPOSITORY}:{TREASURY_DEFAULT_COMPONENT_TAG}"),
        TREASURY_DEFAULT_COMPONENT_DIGEST,
        "prod",
    )
}

#[must_use]
pub fn treasury_disbursement_component_binding_with_artifact(
    component_ref: impl Into<String>,
    digest: impl Into<String>,
    rollout_environment: impl Into<String>,
) -> WasmComponentBindingV1 {
    WasmComponentBindingV1::workflow(
        TREASURY_DISBURSEMENT_NAME,
        signed_component_ref(component_ref, digest),
        rollout_environment,
        platform_contract_paths(),
        vec![surrealdb_capability_binding(), http_capability_binding()],
    )
}

#[must_use]
pub fn rollout_target_for_environment(environment: &str) -> RolloutTargetV1 {
    RolloutTargetV1 {
        environment: environment.to_string(),
        namespace: environment.to_string(),
        policy_group: format!("origin-{environment}"),
    }
}

fn signed_component_ref(
    component_ref: impl Into<String>,
    digest: impl Into<String>,
) -> SignedComponentRefV1 {
    SignedComponentRefV1 {
        component_ref: component_ref.into(),
        digest: digest.into(),
        signature_ref: None,
    }
}

fn platform_contract_paths() -> Vec<String> {
    vec![
        PLATFORM_WIT_PATH.to_string(),
        TREASURY_CONTRACT_PATH.to_string(),
    ]
}

fn surrealdb_capability_binding() -> CapabilityBindingV1 {
    CapabilityBindingV1 {
        provider_id: SURREALDB_PROVIDER_ID.to_string(),
        contract_id: SURREALDB_CONTRACT_ID.to_string(),
        link_name: SURREALDB_LINK_NAME.to_string(),
    }
}

fn http_capability_binding() -> CapabilityBindingV1 {
    CapabilityBindingV1 {
        provider_id: HTTP_PROVIDER_ID.to_string(),
        contract_id: HTTP_CONTRACT_ID.to_string(),
        link_name: HTTP_LINK_NAME.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::{
        finance_service_component_binding, rollout_target_for_environment,
        treasury_disbursement_component_binding,
    };

    #[test]
    fn finance_service_component_binding_stays_in_platform_delivery_layer() {
        let binding = finance_service_component_binding();
        assert_eq!(
            binding.component.component_ref,
            "ghcr.io/shortorigin/finance-service:wasm"
        );
        assert_eq!(binding.component.digest, "sha256:finance-service");
        assert_eq!(binding.required_capabilities.len(), 1);
    }

    #[test]
    fn treasury_workflow_component_binding_declares_expected_capabilities() {
        let binding = treasury_disbursement_component_binding();
        assert_eq!(
            binding.component.component_ref,
            "ghcr.io/shortorigin/treasury-disbursement:wasm"
        );
        assert_eq!(binding.required_capabilities.len(), 2);
    }

    #[test]
    fn rollout_target_tracks_environment_name() {
        let target = rollout_target_for_environment("stage");
        assert_eq!(target.environment, "stage");
        assert_eq!(target.namespace, "stage");
        assert_eq!(target.policy_group, "origin-stage");
    }
}
