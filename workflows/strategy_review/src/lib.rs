use contracts::WorkflowBoundaryV1;

#[must_use]
pub fn workflow_boundary() -> WorkflowBoundaryV1 {
    contracts::workflow_boundary_named("strategy_review")
        .expect("generated strategy_review boundary")
}
