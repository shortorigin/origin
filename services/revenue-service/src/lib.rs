use contracts::ServiceBoundaryV1;

#[must_use]
pub fn service_boundary() -> ServiceBoundaryV1 {
    contracts::service_boundary_named("revenue-service")
        .expect("generated revenue-service boundary")
}
