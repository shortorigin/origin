use contracts::WorkflowBoundaryV1;

#[must_use]
pub fn workflow_boundary() -> WorkflowBoundaryV1 {
    contracts::workflow_boundary_named("continuity_activation")
        .expect("generated continuity_activation boundary")
}
