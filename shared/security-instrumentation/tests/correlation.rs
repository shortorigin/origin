use security_instrumentation::correlate_observation;

#[test]
fn observation_round_trips_trace_and_evidence() {
    let observation = correlate_observation("runtime-plane", "high", "decision-42");
    let round_trip = serde_json::from_str::<security_instrumentation::SecurityObservation>(
        &serde_json::to_string(&observation).expect("serialize"),
    )
    .expect("deserialize");

    assert_eq!(
        round_trip.trace.decision_ref.as_deref(),
        Some("decision-42")
    );
    assert_eq!(
        round_trip.evidence.related_decision_refs,
        vec!["decision-42"]
    );
}
