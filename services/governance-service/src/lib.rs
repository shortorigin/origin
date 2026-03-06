use std::collections::BTreeMap;

use contracts::{
    ModelApprovalStatusV1, ModelApprovalV1, MutationAuthorizationV1, PromotionRecommendationV1,
    ServiceBoundaryV1,
};
use error_model::{InstitutionalError, InstitutionalResult};

#[derive(Debug, Default, Clone)]
pub struct GovernanceService {
    approvals: BTreeMap<String, ModelApprovalV1>,
    recommendations: Vec<PromotionRecommendationV1>,
}

impl GovernanceService {
    pub fn submit_model(&mut self, model_id: &str, version: &str, notes: &str) {
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

    pub fn approve_model(&mut self, model_id: &str, version: &str, reviewer: &str) {
        if let Some(model) = self.approvals.get_mut(&format!("{model_id}:{version}")) {
            model.status = ModelApprovalStatusV1::Approved;
            model.approved_by = Some(reviewer.to_string());
        }
    }

    #[must_use]
    pub fn approved_models(&self) -> Vec<String> {
        self.approvals
            .values()
            .filter(|model| model.status == ModelApprovalStatusV1::Approved)
            .map(|model| format!("{}:{}", model.model_id, model.version))
            .collect()
    }

    pub fn record_recommendation(
        &mut self,
        authorization: &MutationAuthorizationV1,
        recommendation: PromotionRecommendationV1,
    ) -> InstitutionalResult<PromotionRecommendationV1> {
        authorization
            .assert_workflow("quant_strategy_promotion")
            .map_err(|invariant| InstitutionalError::InvariantViolation { invariant })?;
        authorization
            .assert_target_service("governance-service")
            .map_err(|invariant| InstitutionalError::InvariantViolation { invariant })?;
        self.recommendations.push(recommendation.clone());
        Ok(recommendation)
    }

    #[must_use]
    pub fn recommendations(&self) -> &[PromotionRecommendationV1] {
        &self.recommendations
    }
}

#[must_use]
pub fn service_boundary() -> ServiceBoundaryV1 {
    contracts::service_boundary_named("governance-service")
        .expect("generated governance-service boundary")
}
