use contracts::ServiceBoundaryV1;

#[must_use]
pub fn service_boundary() -> ServiceBoundaryV1 {
    contracts::service_boundary_named("resilience-service")
        .expect("generated resilience-service boundary")
}
