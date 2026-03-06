use approval_service::ApprovalService;
use chrono::Utc;
use contracts::{
    AgentActionRequestV1, ApprovalDecisionV1, Classification, ImpactTier, ReleaseApprovalRequestV1,
};
use engineering_service::EngineeringService;
use evidence_service::EvidenceService;
use identity::{ActorRef, InstitutionalRole};
use orchestrator::WorkflowEngine;
use policy_service::PolicyService;
use release_approval::execute;
use security_service::SecurityService;

fn build_action(action_id: &str) -> AgentActionRequestV1 {
    AgentActionRequestV1 {
        action_id: action_id.to_owned(),
        actor_ref: ActorRef("human.release_operator".to_owned()),
        objective: "Approve governed production release".to_owned(),
        requested_workflow: "release_approval".to_owned(),
        impact_tier: ImpactTier::Tier3,
        classification: Classification::Restricted,
        required_approver_roles: vec![InstitutionalRole::Cto, InstitutionalRole::Ciso],
        policy_refs: vec!["engineering.release.approval.v1".to_owned()],
    }
}

fn build_request() -> ReleaseApprovalRequestV1 {
    ReleaseApprovalRequestV1 {
        release_id: "release-2026-03-06.1".to_owned(),
        build_ref: "ghcr.io/shortorigin/release@sha256:test".to_owned(),
        artifact_digest: "sha256:test".to_owned(),
        test_evidence_ref: "evidence/tests/release-2026-03-06.1".to_owned(),
        environment: "prod".to_owned(),
    }
}

#[test]
fn release_approval_rejects_missing_approval() {
    let policy_service = PolicyService::institutional_default();
    let approval_service = ApprovalService::default();
    let evidence_service = EvidenceService::default();
    let mut engine = WorkflowEngine::new(policy_service, approval_service, evidence_service);
    let mut engineering_service = EngineeringService::default();
    let security_service = SecurityService;
    let mut audit_service = EvidenceService::default();

    let result = execute(
        &mut engine,
        &mut engineering_service,
        &security_service,
        &mut audit_service,
        &build_action("action::release::missing"),
        build_request(),
    );

    assert!(matches!(
        result,
        Err(error_model::InstitutionalError::ApprovalMissing { .. })
    ));
    assert_eq!(engineering_service.release_approvals().len(), 0);
}

#[test]
fn release_approval_accepts_dual_approval_and_security_ready_release() {
    let policy_service = PolicyService::institutional_default();
    let mut approval_service = ApprovalService::default();
    let evidence_service = EvidenceService::default();
    let action = build_action("action::release::approved");
    let request = build_request();
    let decided_at = Utc::now();

    approval_service.record_decision(ApprovalDecisionV1 {
        action_id: action.action_id.clone(),
        approver: ActorRef("human.cto".to_owned()),
        approver_role: InstitutionalRole::Cto,
        approved: true,
        rationale: "Engineering signoff granted".to_owned(),
        decided_at,
    });
    approval_service.record_decision(ApprovalDecisionV1 {
        action_id: action.action_id.clone(),
        approver: ActorRef("human.ciso".to_owned()),
        approver_role: InstitutionalRole::Ciso,
        approved: true,
        rationale: "Security signoff granted".to_owned(),
        decided_at,
    });

    let mut engine = WorkflowEngine::new(policy_service, approval_service, evidence_service);
    let mut engineering_service = EngineeringService::default();
    let security_service = SecurityService;
    let mut audit_service = EvidenceService::default();

    let result = execute(
        &mut engine,
        &mut engineering_service,
        &security_service,
        &mut audit_service,
        &action,
        request,
    )
    .expect("release approval");

    assert_eq!(result.approved_by_roles.len(), 2);
    assert_eq!(result.audit_event_ids.len(), 2);
    assert_eq!(engineering_service.release_approvals().len(), 1);
    assert_eq!(engine.recorded_evidence().len(), 1);
    assert_eq!(audit_service.audit_events().len(), 2);
}
