pub mod component;

use contracts::{
    AgentActionRequestV1, TreasuryDisbursementRecordedV1, TreasuryDisbursementRequestV1,
    WorkflowBoundaryV1,
};
use enforcement::GuardedMutationRequest;
use error_model::InstitutionalResult;
use evidence_sdk::EvidenceSink;
use finance_service::FinanceService;
use orchestrator::WorkflowEngine;
use policy_sdk::{ApprovalVerificationPort, PolicyDecisionPort};

#[must_use]
pub fn workflow_boundary() -> WorkflowBoundaryV1 {
    contracts::workflow_boundary_named("treasury_disbursement")
        .expect("generated treasury_disbursement boundary")
}

pub fn execute<P, A, E>(
    engine: &mut WorkflowEngine<P, A, E>,
    finance_service: &mut FinanceService,
    action: &AgentActionRequestV1,
    request: TreasuryDisbursementRequestV1,
) -> InstitutionalResult<TreasuryDisbursementRecordedV1>
where
    P: PolicyDecisionPort,
    A: ApprovalVerificationPort,
    E: EvidenceSink,
{
    let guarded_request = GuardedMutationRequest {
        action_id: action.action_id.clone(),
        workflow_name: "treasury_disbursement".to_owned(),
        target_service: "finance-service".to_owned(),
        target_aggregate: "treasury_ledger".to_owned(),
        actor_ref: action.actor_ref.clone(),
        impact_tier: action.impact_tier,
        classification: action.classification,
        policy_refs: action.policy_refs.clone(),
        required_approver_roles: action.required_approver_roles.clone(),
        environment: "prod".to_owned(),
        cross_domain: true,
    };

    engine.execute_mutation(guarded_request, |context| {
        finance_service.record_disbursement(&context.authorization(), request.clone())
    })
}
