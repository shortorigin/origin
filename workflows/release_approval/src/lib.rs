pub mod component;

use contracts::{
    AgentActionRequestV1, ReleaseApprovalRecordV1, ReleaseApprovalRequestV1, WorkflowBoundaryV1,
};
use enforcement::GuardedMutationRequest;
use engineering_service::EngineeringService;
use error_model::{InstitutionalError, InstitutionalResult};
use evidence_sdk::EvidenceSink;
use evidence_service::EvidenceService;
use orchestrator::WorkflowEngine;
use policy_sdk::{ApprovalVerificationPort, PolicyDecisionPort};
use security_service::SecurityService;

#[must_use]
pub fn workflow_boundary() -> WorkflowBoundaryV1 {
    contracts::workflow_boundary_named("release_approval")
        .expect("generated release_approval boundary")
}

pub fn execute<P, A, E>(
    engine: &mut WorkflowEngine<P, A, E>,
    engineering_service: &mut EngineeringService,
    security_service: &SecurityService,
    audit_service: &mut EvidenceService,
    action: &AgentActionRequestV1,
    request: ReleaseApprovalRequestV1,
) -> InstitutionalResult<ReleaseApprovalRecordV1>
where
    P: PolicyDecisionPort,
    A: ApprovalVerificationPort,
    E: EvidenceSink,
{
    let guarded_request = GuardedMutationRequest {
        action_id: action.action_id.clone(),
        workflow_name: "release_approval".to_owned(),
        target_service: "engineering-service".to_owned(),
        target_aggregate: "release_window".to_owned(),
        actor_ref: action.actor_ref.clone(),
        impact_tier: action.impact_tier,
        classification: action.classification,
        policy_refs: action.policy_refs.clone(),
        required_approver_roles: action.required_approver_roles.clone(),
        environment: request.environment.clone(),
        cross_domain: true,
    };

    let request_audit = audit_service.append_audit_event(serde_json::json!({
        "event_type": "release_approval_requested",
        "action_id": action.action_id,
        "release_id": request.release_id,
        "environment": request.environment,
    }))?;

    engine.execute_mutation(guarded_request, |context| {
        let security_assessment = security_service.assess_release(&request)?;
        if !security_assessment.ready {
            audit_service.append_audit_event(serde_json::json!({
                "event_type": "release_approval_denied",
                "action_id": action.action_id,
                "release_id": request.release_id,
                "reason": security_assessment.summary,
            }))?;
            return Err(InstitutionalError::PolicyDenied {
                reason: security_assessment.summary,
            });
        }

        let record = engineering_service.approve_release(
            &context.authorization(),
            request.clone(),
            &security_assessment,
            context.decision().evidence_refs.clone(),
            vec![request_audit.event_id.clone()],
        )?;

        let approved_audit = audit_service.append_audit_event(serde_json::json!({
            "event_type": "release_approval_approved",
            "action_id": action.action_id,
            "release_id": record.release_id,
            "release_window_ref": record.release_window_ref,
            "approved_by_roles": record
                .approved_by_roles
                .iter()
                .map(|role| format!("{role:?}"))
                .collect::<Vec<_>>(),
        }))?;

        engineering_service.append_audit_event(&record.release_id, approved_audit.event_id)
    })
}
