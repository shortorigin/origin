use std::collections::BTreeMap;

use contracts::{
    ModelApprovalStatusV1, ModelApprovalV1, PromotionRecommendationV1, ServiceBoundaryV1,
};
use enforcement::ApprovedMutationContext;
use error_model::InstitutionalResult;
use identity::{ServiceId, WorkflowId};

const SERVICE_NAME: &str = "governance-service";
const DOMAIN_NAME: &str = "strategy_governance";
const APPROVED_WORKFLOWS: &[&str] = &[
    "strategy_review",
    "policy_exception",
    "quant_strategy_promotion",
];
const OWNED_AGGREGATES: &[&str] = &[
    "governance_decision",
    "institutional_invariant",
    "promotion_recommendation",
    "model_approval",
];

fn service_id() -> ServiceId {
    SERVICE_NAME.into()
}

fn quant_strategy_promotion_workflow_id() -> WorkflowId {
    "quant_strategy_promotion".into()
}

#[derive(Debug, Default, Clone)]
struct InMemoryGovernanceStore {
    approvals: BTreeMap<String, ModelApprovalV1>,
    recommendations: Vec<PromotionRecommendationV1>,
}

impl InMemoryGovernanceStore {
    fn submit_model(&mut self, model_id: &str, version: &str, notes: &str) {
        self.approvals.insert(
            format!("{model_id}:{version}"),
            ModelApprovalV1 {
                model_id: model_id.to_string(),
                version: version.to_string(),
                approved_by: None,
                status: ModelApprovalStatusV1::Pending,
                notes: notes.to_string(),
            },
        );
    }

    fn approve_model(&mut self, model_id: &str, version: &str, reviewer: &str) {
        if let Some(model) = self.approvals.get_mut(&format!("{model_id}:{version}")) {
            model.status = ModelApprovalStatusV1::Approved;
            model.approved_by = Some(reviewer.to_string());
        }
    }

    fn approved_models(&self) -> Vec<String> {
        self.approvals
            .values()
            .filter(|model| model.status == ModelApprovalStatusV1::Approved)
            .map(|model| format!("{}:{}", model.model_id, model.version))
            .collect()
    }

    fn record_recommendation(&mut self, recommendation: PromotionRecommendationV1) {
        self.recommendations.push(recommendation);
    }

    fn recommendations(&self) -> &[PromotionRecommendationV1] {
        &self.recommendations
    }
}

#[derive(Debug, Default, Clone)]
pub struct GovernanceService {
    store: InMemoryGovernanceStore,
}

impl GovernanceService {
    pub fn submit_model(&mut self, model_id: &str, version: &str, notes: &str) {
        self.store.submit_model(model_id, version, notes);
    }

    pub fn approve_model(&mut self, model_id: &str, version: &str, reviewer: &str) {
        self.store.approve_model(model_id, version, reviewer);
    }

    #[must_use]
    pub fn approved_models(&self) -> Vec<String> {
        self.store.approved_models()
    }

    pub fn record_recommendation(
        &mut self,
        context: &ApprovedMutationContext,
        recommendation: PromotionRecommendationV1,
    ) -> InstitutionalResult<PromotionRecommendationV1> {
        context.assert_workflow(&quant_strategy_promotion_workflow_id())?;
        context.assert_target_service(&service_id())?;
        self.store.record_recommendation(recommendation.clone());
        Ok(recommendation)
    }

    #[must_use]
    pub fn recommendations(&self) -> &[PromotionRecommendationV1] {
        self.store.recommendations()
    }
}

#[must_use]
pub fn service_boundary() -> ServiceBoundaryV1 {
    ServiceBoundaryV1 {
        service_name: SERVICE_NAME.to_owned(),
        domain: DOMAIN_NAME.to_owned(),
        approved_workflows: APPROVED_WORKFLOWS.iter().copied().map(Into::into).collect(),
        owned_aggregates: OWNED_AGGREGATES
            .iter()
            .copied()
            .map(str::to_owned)
            .collect(),
    }
}

#[cfg(test)]
mod tests {
    mod contract_parity {
        include!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../testing/contract_parity.rs"
        ));
    }

    use contract_parity::assert_service_boundary_matches_catalog;

    use super::{service_boundary, DOMAIN_NAME};

    #[test]
    fn service_boundary_matches_enterprise_catalog() {
        let source =
            include_str!("../../../enterprise/domains/strategy_governance/service_boundaries.toml");
        let boundary = service_boundary();

        assert_service_boundary_matches_catalog(&boundary, DOMAIN_NAME, source);
    }
}
