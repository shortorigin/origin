use contracts::EvidenceManifestV1;
use enforcement::{ApprovedMutationContext, GuardedMutationRequest, MutationEnforcer};
use error_model::InstitutionalResult;
use evidence_sdk::EvidenceSink;
use policy_sdk::{ApprovalVerificationPort, PolicyDecisionPort};
use tokio::time::Duration;

pub struct WorkflowEngine<P, A, E> {
    policy_port: P,
    approval_port: A,
    evidence_sink: E,
}

impl<P, A, E> WorkflowEngine<P, A, E> {
    #[must_use]
    pub fn new(policy_port: P, approval_port: A, evidence_sink: E) -> Self {
        Self {
            policy_port,
            approval_port,
            evidence_sink,
        }
    }
}

impl<P, A, E> WorkflowEngine<P, A, E>
where
    P: PolicyDecisionPort,
    A: ApprovalVerificationPort,
    E: EvidenceSink,
{
    pub async fn execute_mutation<T, F>(
        &mut self,
        request: GuardedMutationRequest,
        action: F,
    ) -> InstitutionalResult<T>
    where
        F: FnOnce(&ApprovedMutationContext) -> InstitutionalResult<T>,
    {
        let mut enforcer =
            MutationEnforcer::new(&self.policy_port, &self.approval_port, &self.evidence_sink);
        let context = enforcer.authorize(&request).await?;
        action(&context)
    }

    pub async fn execute_mutation_with_timeout<T, F>(
        &mut self,
        request: GuardedMutationRequest,
        timeout_limit: Duration,
        action: F,
    ) -> InstitutionalResult<T>
    where
        F: FnOnce(&ApprovedMutationContext) -> InstitutionalResult<T>,
    {
        let mut enforcer =
            MutationEnforcer::new(&self.policy_port, &self.approval_port, &self.evidence_sink);
        let context = enforcer
            .authorize_with_timeout(&request, timeout_limit)
            .await?;
        action(&context)
    }

    pub async fn recorded_evidence(&self) -> InstitutionalResult<Vec<EvidenceManifestV1>> {
        self.evidence_sink.recorded().await
    }
}
