use contracts::ServiceBoundaryV1;

#[must_use]
pub fn service_boundary() -> ServiceBoundaryV1 {
    contracts::service_boundary_named("legal-service").expect("generated legal-service boundary")
}
