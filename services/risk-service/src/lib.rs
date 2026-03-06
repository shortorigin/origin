use contracts::{RiskRecordV1, ServiceBoundaryV1};

#[derive(Debug, Default, Clone)]
pub struct RiskService {
    records: Vec<RiskRecordV1>,
}

impl RiskService {
    pub fn register(&mut self, record: RiskRecordV1) {
        self.records.push(record);
    }

    #[must_use]
    pub fn active_records(&self) -> &[RiskRecordV1] {
        &self.records
    }
}

#[must_use]
pub fn service_boundary() -> ServiceBoundaryV1 {
    contracts::service_boundary_named("risk-service").expect("generated risk-service boundary")
}
