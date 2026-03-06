pub mod finance_service {
    use wasmcloud_bindings::{CapabilityBindingV1, SignedComponentRefV1, WasmComponentBindingV1};

    pub const COMPONENT_REPOSITORY: &str = "ghcr.io/shortorigin/finance-service";
    pub const DEFAULT_COMPONENT_TAG: &str = "wasm";
    pub const DEFAULT_COMPONENT_DIGEST: &str = "sha256:finance-service";

    #[must_use]
    pub fn component_binding() -> WasmComponentBindingV1 {
        component_binding_with_artifact(
            format!("{COMPONENT_REPOSITORY}:{DEFAULT_COMPONENT_TAG}"),
            DEFAULT_COMPONENT_DIGEST,
            "prod",
        )
    }

    #[must_use]
    pub fn component_binding_with_artifact(
        component_ref: impl Into<String>,
        digest: impl Into<String>,
        rollout_environment: impl Into<String>,
    ) -> WasmComponentBindingV1 {
        WasmComponentBindingV1::service(
            "finance-service",
            SignedComponentRefV1 {
                component_ref: component_ref.into(),
                digest: digest.into(),
                signature_ref: None,
            },
            rollout_environment,
            vec![
                "schemas/wit/v1/platform.wit".to_string(),
                "schemas/contracts/v1/treasury-disbursement-v1.json".to_string(),
            ],
            vec![CapabilityBindingV1 {
                provider_id: "wasmcloud:keyvalue".to_string(),
                contract_id: "keyvalue".to_string(),
                link_name: "surrealdb".to_string(),
            }],
        )
    }
}

pub mod treasury_disbursement {
    use wasmcloud_bindings::{CapabilityBindingV1, SignedComponentRefV1, WasmComponentBindingV1};

    pub const COMPONENT_REPOSITORY: &str = "ghcr.io/shortorigin/treasury-disbursement";
    pub const DEFAULT_COMPONENT_TAG: &str = "wasm";
    pub const DEFAULT_COMPONENT_DIGEST: &str = "sha256:treasury-disbursement";

    #[must_use]
    pub fn component_binding() -> WasmComponentBindingV1 {
        component_binding_with_artifact(
            format!("{COMPONENT_REPOSITORY}:{DEFAULT_COMPONENT_TAG}"),
            DEFAULT_COMPONENT_DIGEST,
            "prod",
        )
    }

    #[must_use]
    pub fn component_binding_with_artifact(
        component_ref: impl Into<String>,
        digest: impl Into<String>,
        rollout_environment: impl Into<String>,
    ) -> WasmComponentBindingV1 {
        WasmComponentBindingV1::workflow(
            "treasury-disbursement",
            SignedComponentRefV1 {
                component_ref: component_ref.into(),
                digest: digest.into(),
                signature_ref: None,
            },
            rollout_environment,
            vec![
                "schemas/wit/v1/platform.wit".to_string(),
                "schemas/contracts/v1/treasury-disbursement-v1.json".to_string(),
            ],
            vec![
                CapabilityBindingV1 {
                    provider_id: "wasmcloud:keyvalue".to_string(),
                    contract_id: "keyvalue".to_string(),
                    link_name: "surrealdb".to_string(),
                },
                CapabilityBindingV1 {
                    provider_id: "wasmcloud:httpserver".to_string(),
                    contract_id: "http".to_string(),
                    link_name: "default".to_string(),
                },
            ],
        )
    }
}

pub mod engineering_service {
    use wasmcloud_bindings::{CapabilityBindingV1, SignedComponentRefV1, WasmComponentBindingV1};

    pub const COMPONENT_REPOSITORY: &str = "ghcr.io/shortorigin/engineering-service";
    pub const DEFAULT_COMPONENT_TAG: &str = "wasm";
    pub const DEFAULT_COMPONENT_DIGEST: &str = "sha256:engineering-service";

    #[must_use]
    pub fn component_binding() -> WasmComponentBindingV1 {
        component_binding_with_artifact(
            format!("{COMPONENT_REPOSITORY}:{DEFAULT_COMPONENT_TAG}"),
            DEFAULT_COMPONENT_DIGEST,
            "prod",
        )
    }

    #[must_use]
    pub fn component_binding_with_artifact(
        component_ref: impl Into<String>,
        digest: impl Into<String>,
        rollout_environment: impl Into<String>,
    ) -> WasmComponentBindingV1 {
        WasmComponentBindingV1::service(
            "engineering-service",
            SignedComponentRefV1 {
                component_ref: component_ref.into(),
                digest: digest.into(),
                signature_ref: None,
            },
            rollout_environment,
            vec![
                "schemas/wit/v1/platform.wit".to_string(),
                "schemas/contracts/v1/release-approval-v1.json".to_string(),
            ],
            vec![CapabilityBindingV1 {
                provider_id: "wasmcloud:keyvalue".to_string(),
                contract_id: "keyvalue".to_string(),
                link_name: "surrealdb".to_string(),
            }],
        )
    }
}

pub mod security_service {
    use wasmcloud_bindings::{CapabilityBindingV1, SignedComponentRefV1, WasmComponentBindingV1};

    pub const COMPONENT_REPOSITORY: &str = "ghcr.io/shortorigin/security-service";
    pub const DEFAULT_COMPONENT_TAG: &str = "wasm";
    pub const DEFAULT_COMPONENT_DIGEST: &str = "sha256:security-service";

    #[must_use]
    pub fn component_binding() -> WasmComponentBindingV1 {
        component_binding_with_artifact(
            format!("{COMPONENT_REPOSITORY}:{DEFAULT_COMPONENT_TAG}"),
            DEFAULT_COMPONENT_DIGEST,
            "prod",
        )
    }

    #[must_use]
    pub fn component_binding_with_artifact(
        component_ref: impl Into<String>,
        digest: impl Into<String>,
        rollout_environment: impl Into<String>,
    ) -> WasmComponentBindingV1 {
        WasmComponentBindingV1::service(
            "security-service",
            SignedComponentRefV1 {
                component_ref: component_ref.into(),
                digest: digest.into(),
                signature_ref: None,
            },
            rollout_environment,
            vec![
                "schemas/wit/v1/platform.wit".to_string(),
                "schemas/contracts/v1/release-approval-v1.json".to_string(),
            ],
            vec![CapabilityBindingV1 {
                provider_id: "wasmcloud:keyvalue".to_string(),
                contract_id: "keyvalue".to_string(),
                link_name: "surrealdb".to_string(),
            }],
        )
    }
}

pub mod release_approval {
    use wasmcloud_bindings::{CapabilityBindingV1, SignedComponentRefV1, WasmComponentBindingV1};

    pub const COMPONENT_REPOSITORY: &str = "ghcr.io/shortorigin/release-approval";
    pub const DEFAULT_COMPONENT_TAG: &str = "wasm";
    pub const DEFAULT_COMPONENT_DIGEST: &str = "sha256:release-approval";

    #[must_use]
    pub fn component_binding() -> WasmComponentBindingV1 {
        component_binding_with_artifact(
            format!("{COMPONENT_REPOSITORY}:{DEFAULT_COMPONENT_TAG}"),
            DEFAULT_COMPONENT_DIGEST,
            "prod",
        )
    }

    #[must_use]
    pub fn component_binding_with_artifact(
        component_ref: impl Into<String>,
        digest: impl Into<String>,
        rollout_environment: impl Into<String>,
    ) -> WasmComponentBindingV1 {
        WasmComponentBindingV1::workflow(
            "release-approval",
            SignedComponentRefV1 {
                component_ref: component_ref.into(),
                digest: digest.into(),
                signature_ref: None,
            },
            rollout_environment,
            vec![
                "schemas/wit/v1/platform.wit".to_string(),
                "schemas/contracts/v1/release-approval-v1.json".to_string(),
            ],
            vec![
                CapabilityBindingV1 {
                    provider_id: "wasmcloud:keyvalue".to_string(),
                    contract_id: "keyvalue".to_string(),
                    link_name: "surrealdb".to_string(),
                },
                CapabilityBindingV1 {
                    provider_id: "wasmcloud:httpserver".to_string(),
                    contract_id: "http".to_string(),
                    link_name: "default".to_string(),
                },
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        engineering_service, finance_service, release_approval, security_service,
        treasury_disbursement,
    };

    #[test]
    fn finance_descriptor_matches_expected_defaults() {
        let binding = finance_service::component_binding();

        assert_eq!(binding.workload_name, "finance-service");
        assert_eq!(
            binding.component.component_ref,
            "ghcr.io/shortorigin/finance-service:wasm"
        );
        assert_eq!(binding.component.digest, "sha256:finance-service");
        assert_eq!(binding.rollout_environment, "prod");
        assert_eq!(binding.required_capabilities.len(), 1);
        assert_eq!(binding.required_capabilities[0].link_name, "surrealdb");
    }

    #[test]
    fn treasury_descriptor_preserves_capabilities_and_refs() {
        let binding = treasury_disbursement::component_binding_with_artifact(
            "ghcr.io/shortorigin/treasury-disbursement:v1",
            "sha256:test",
            "stage",
        );

        assert_eq!(binding.workload_name, "treasury-disbursement");
        assert_eq!(
            binding.component.component_ref,
            "ghcr.io/shortorigin/treasury-disbursement:v1"
        );
        assert_eq!(binding.component.digest, "sha256:test");
        assert_eq!(binding.rollout_environment, "stage");
        assert_eq!(binding.required_capabilities.len(), 2);
        assert!(binding
            .config_schema_refs
            .iter()
            .any(|path| path == "schemas/wit/v1/platform.wit"));
    }

    #[test]
    fn release_approval_descriptor_matches_defaults() {
        let binding = release_approval::component_binding();
        assert_eq!(binding.workload_name, "release-approval");
        assert_eq!(
            binding.component.component_ref,
            "ghcr.io/shortorigin/release-approval:wasm"
        );
        assert_eq!(binding.required_capabilities.len(), 2);
    }

    #[test]
    fn engineering_and_security_descriptors_reference_release_contract() {
        let engineering = engineering_service::component_binding();
        let security = security_service::component_binding();
        assert!(engineering
            .config_schema_refs
            .iter()
            .any(|path| path == "schemas/contracts/v1/release-approval-v1.json"));
        assert!(security
            .config_schema_refs
            .iter()
            .any(|path| path == "schemas/contracts/v1/release-approval-v1.json"));
    }
}
