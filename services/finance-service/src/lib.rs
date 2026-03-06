pub mod component;

use contracts::{
    MutationAuthorizationV1, ServiceBoundaryV1, TreasuryDisbursementRecordedV1,
    TreasuryDisbursementRequestV1,
};
use error_model::{InstitutionalError, InstitutionalResult};

#[derive(Debug, Default, Clone)]
pub struct FinanceService {
    disbursements: Vec<TreasuryDisbursementRecordedV1>,
}

impl FinanceService {
    pub fn record_disbursement(
        &mut self,
        authorization: &MutationAuthorizationV1,
        request: TreasuryDisbursementRequestV1,
    ) -> InstitutionalResult<TreasuryDisbursementRecordedV1> {
        authorization
            .assert_workflow("treasury_disbursement")
            .map_err(|invariant| InstitutionalError::InvariantViolation { invariant })?;
        authorization
            .assert_target_service("finance-service")
            .map_err(|invariant| InstitutionalError::InvariantViolation { invariant })?;
        let record = TreasuryDisbursementRecordedV1::new(
            authorization.correlation_id.clone(),
            &request,
            authorization.approved_by_roles.clone(),
        );
        self.disbursements.push(record.clone());
        Ok(record)
    }

    #[must_use]
    pub fn disbursements(&self) -> &[TreasuryDisbursementRecordedV1] {
        &self.disbursements
    }
}

#[must_use]
pub fn service_boundary() -> ServiceBoundaryV1 {
    contracts::service_boundary_named("finance-service")
        .expect("generated finance-service boundary")
}
