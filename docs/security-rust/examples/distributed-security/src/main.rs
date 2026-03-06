use security_instrumentation::correlate_observation;

fn main() {
    let observation = correlate_observation("workflow-gateway", "medium", "decision-demo");

    println!(
        "correlated {} with evidence {}",
        observation.component, observation.evidence.evidence_id
    );
}
