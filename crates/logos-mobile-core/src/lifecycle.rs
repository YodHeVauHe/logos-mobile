//! Pure node lifecycle transition table; it performs no process operations.

/// Current logical node state.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NodeState {
    Stopped,
    Starting,
    Running,
    Degraded,
    Failed,
    Stopping,
}

/// An event reported by a future process or platform adapter.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LifecycleEvent {
    Start,
    StartupSucceeded,
    StartupFailed,
    HealthDegraded,
    HealthRecovered,
    RuntimeFailed,
    Stop,
    Stopped,
    Restart,
}

/// An event that is invalid in the current state.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TransitionError {
    pub state: NodeState,
    pub event: LifecycleEvent,
}

impl std::fmt::Display for TransitionError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "event {:?} is invalid while node is {:?}",
            self.event, self.state
        )
    }
}

impl std::error::Error for TransitionError {}

/// Apply one event to a logical lifecycle state without side effects.
///
/// # Errors
///
/// Returns the original state and event when no transition is defined.
pub const fn transition(
    state: NodeState,
    event: LifecycleEvent,
) -> Result<NodeState, TransitionError> {
    let next = match (state, event) {
        (NodeState::Stopped, LifecycleEvent::Start)
        | (NodeState::Failed, LifecycleEvent::Restart) => NodeState::Starting,
        (NodeState::Starting, LifecycleEvent::StartupSucceeded)
        | (NodeState::Degraded, LifecycleEvent::HealthRecovered) => NodeState::Running,
        (NodeState::Starting, LifecycleEvent::StartupFailed)
        | (NodeState::Running | NodeState::Degraded, LifecycleEvent::RuntimeFailed) => {
            NodeState::Failed
        }
        (
            NodeState::Starting | NodeState::Running | NodeState::Degraded | NodeState::Failed,
            LifecycleEvent::Stop,
        ) => NodeState::Stopping,
        (NodeState::Running, LifecycleEvent::HealthDegraded) => NodeState::Degraded,
        (NodeState::Stopping, LifecycleEvent::Stopped) => NodeState::Stopped,
        _ => return Err(TransitionError { state, event }),
    };
    Ok(next)
}

#[cfg(test)]
mod tests {
    use super::{LifecycleEvent, NodeState, TransitionError, transition};

    #[test]
    fn start_and_successful_startup_transition_to_running() {
        assert_eq!(
            transition(NodeState::Stopped, LifecycleEvent::Start),
            Ok(NodeState::Starting)
        );
        assert_eq!(
            transition(NodeState::Starting, LifecycleEvent::StartupSucceeded),
            Ok(NodeState::Running)
        );
    }

    #[test]
    fn health_can_degrade_and_recover() {
        assert_eq!(
            transition(NodeState::Running, LifecycleEvent::HealthDegraded),
            Ok(NodeState::Degraded)
        );
        assert_eq!(
            transition(NodeState::Degraded, LifecycleEvent::HealthRecovered),
            Ok(NodeState::Running)
        );
    }

    #[test]
    fn startup_and_runtime_failures_are_distinct_and_restartable() {
        assert_eq!(
            transition(NodeState::Starting, LifecycleEvent::StartupFailed),
            Ok(NodeState::Failed)
        );
        assert_eq!(
            transition(NodeState::Failed, LifecycleEvent::Restart),
            Ok(NodeState::Starting)
        );
        assert_eq!(
            transition(NodeState::Running, LifecycleEvent::RuntimeFailed),
            Ok(NodeState::Failed)
        );
    }

    #[test]
    fn stop_requires_explicit_stop_completion() {
        assert_eq!(
            transition(NodeState::Degraded, LifecycleEvent::Stop),
            Ok(NodeState::Stopping)
        );
        assert_eq!(
            transition(NodeState::Stopping, LifecycleEvent::Stopped),
            Ok(NodeState::Stopped)
        );
    }

    #[test]
    fn invalid_transition_returns_error_without_changing_copied_state() {
        let current = NodeState::Stopped;
        assert_eq!(
            transition(current, LifecycleEvent::StartupSucceeded),
            Err(TransitionError {
                state: current,
                event: LifecycleEvent::StartupSucceeded
            })
        );
        assert_eq!(current, NodeState::Stopped);
    }

    #[test]
    fn every_state_rejects_events_outside_its_transition_table() {
        let invalid = [
            (NodeState::Stopped, LifecycleEvent::HealthDegraded),
            (NodeState::Starting, LifecycleEvent::HealthRecovered),
            (NodeState::Running, LifecycleEvent::StartupSucceeded),
            (NodeState::Degraded, LifecycleEvent::StartupFailed),
            (NodeState::Failed, LifecycleEvent::HealthRecovered),
            (NodeState::Stopping, LifecycleEvent::Start),
        ];
        for (state, event) in invalid {
            assert_eq!(
                transition(state, event),
                Err(TransitionError { state, event })
            );
        }
    }
}
