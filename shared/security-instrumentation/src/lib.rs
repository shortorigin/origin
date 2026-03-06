use contracts::{Classification, EvidenceManifestV1};
use serde::{Deserialize, Serialize};
use telemetry::TraceContext;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SecurityObservation {
    pub component: String,
    pub severity: String,
    pub trace: TraceContext,
    pub evidence: EvidenceManifestV1,
}

#[must_use]
pub fn correlate_observation(
    component: impl Into<String>,
    severity: impl Into<String>,
    decision_ref: impl Into<String>,
) -> SecurityObservation {
    let decision_ref = decision_ref.into();
    let trace = TraceContext::new().with_decision_ref(decision_ref.clone());
    let evidence = EvidenceManifestV1 {
        evidence_id: format!("evidence::{decision_ref}"),
        producer: "shared/security-instrumentation".to_string(),
        artifact_hash: format!("sha256::{decision_ref}"),
        storage_ref: format!("memory://security-observation/{decision_ref}"),
        retention_class: "institutional_record".to_string(),
        classification: Classification::Internal,
        related_decision_refs: vec![decision_ref],
    };

    SecurityObservation {
        component: component.into(),
        severity: severity.into(),
        trace,
        evidence,
    }
}
