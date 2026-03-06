use contracts::WorkflowBoundaryV1;

#[must_use]
pub fn workflow_boundary() -> WorkflowBoundaryV1 {
    contracts::workflow_boundary_named("vendor_onboarding")
        .expect("generated vendor_onboarding boundary")
}
