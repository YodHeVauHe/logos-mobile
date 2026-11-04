use logos_mobile_core::lifecycle::{LifecycleEvent, NodeState, transition};
use logos_mobile_core::profile::{Confidence, Measurement, MeasurementStatus};

#[test]
fn public_measurement_contract_uses_explicit_clock_and_confidence() {
    let sample = Measurement::observed(0_u64, 40, Confidence::High);

    assert_eq!(
        sample.status(50, 20, Confidence::Medium),
        MeasurementStatus::Fresh { age_seconds: 10 }
    );
}

#[test]
fn public_lifecycle_contract_returns_new_state_without_runtime_side_effects() {
    assert_eq!(
        transition(NodeState::Stopped, LifecycleEvent::Start),
        Ok(NodeState::Starting)
    );
}
