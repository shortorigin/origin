pub mod component;

use contracts::{ReleaseApprovalRequestV1, SecurityReleaseAssessmentV1, ServiceBoundaryV1};
use error_model::InstitutionalResult;

#[derive(Debug, Default, Clone)]
pub struct SecurityService;

impl SecurityService {
    pub fn assess_release(
        &self,
        request: &ReleaseApprovalRequestV1,
    ) -> InstitutionalResult<SecurityReleaseAssessmentV1> {
        let digest_ok = request.artifact_digest.starts_with("sha256:");
        let evidence_ok = !request.test_evidence_ref.trim().is_empty();
        let ready = digest_ok && evidence_ok;
        let control_refs = vec![
            format!(
                "security.release/{}/artifact-integrity",
                request.environment
            ),
            format!("security.release/{}/test-evidence", request.environment),
        ];
        let summary = if ready {
            format!(
                "security release controls satisfied for `{}`",
                request.release_id
            )
        } else if !digest_ok {
            "artifact digest must be a sha256 reference".to_string()
        } else {
            "test evidence reference must not be empty".to_string()
        };

        Ok(SecurityReleaseAssessmentV1 {
            release_id: request.release_id.clone(),
            environment: request.environment.clone(),
            ready,
            control_refs,
            summary,
        })
    }
}

#[must_use]
pub fn service_boundary() -> ServiceBoundaryV1 {
    contracts::service_boundary_named("security-service")
        .expect("generated security-service boundary")
}
