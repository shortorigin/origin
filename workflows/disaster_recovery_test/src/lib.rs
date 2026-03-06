use contracts::WorkflowBoundaryV1;

#[must_use]
pub fn workflow_boundary() -> WorkflowBoundaryV1 {
    contracts::workflow_boundary_named("disaster_recovery_test")
        .expect("generated disaster_recovery_test boundary")
}
