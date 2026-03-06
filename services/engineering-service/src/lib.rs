pub mod component;

use contracts::{
    MutationAuthorizationV1, ReleaseApprovalRecordV1, ReleaseApprovalRequestV1,
    SecurityReleaseAssessmentV1, ServiceBoundaryV1,
};
use error_model::{InstitutionalError, InstitutionalResult};

#[derive(Debug, Default, Clone)]
pub struct EngineeringService {
    release_approvals: Vec<ReleaseApprovalRecordV1>,
}

impl EngineeringService {
    pub fn approve_release(
        &mut self,
        authorization: &MutationAuthorizationV1,
        request: ReleaseApprovalRequestV1,
        security: &SecurityReleaseAssessmentV1,
        evidence_refs: Vec<String>,
        audit_event_ids: Vec<String>,
    ) -> InstitutionalResult<ReleaseApprovalRecordV1> {
        authorization
            .assert_workflow("release_approval")
            .map_err(|invariant| InstitutionalError::InvariantViolation { invariant })?;
        authorization
            .assert_target_service("engineering-service")
            .map_err(|invariant| InstitutionalError::InvariantViolation { invariant })?;
        if !security.ready {
            return Err(InstitutionalError::PolicyDenied {
                reason: security.summary.clone(),
            });
        }

        let record = ReleaseApprovalRecordV1::new(
            &request,
            authorization.approved_by_roles.clone(),
            evidence_refs,
            audit_event_ids,
        );
        self.release_approvals.push(record.clone());
        Ok(record)
    }

    pub fn append_audit_event(
        &mut self,
        release_id: &str,
        audit_event_id: String,
    ) -> InstitutionalResult<ReleaseApprovalRecordV1> {
        let record = self
            .release_approvals
            .iter_mut()
            .find(|record| record.release_id == release_id)
            .ok_or_else(|| InstitutionalError::NotFound {
                resource: release_id.to_string(),
            })?;
        record.audit_event_ids.push(audit_event_id);
        Ok(record.clone())
    }

    #[must_use]
    pub fn release_approvals(&self) -> &[ReleaseApprovalRecordV1] {
        &self.release_approvals
    }
}

#[must_use]
pub fn service_boundary() -> ServiceBoundaryV1 {
    contracts::service_boundary_named("engineering-service")
        .expect("generated engineering-service boundary")
}
