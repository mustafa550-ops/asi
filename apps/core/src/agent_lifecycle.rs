use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AgentState {
    Created,
    Idle,
    Planning,
    Executing,
    Validating,
    Reporting,
    Paused,
    Completed,
    Error,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AgentAclLevel {
    Observer,
    SemiAutonomous,
    FullAuthority,
}

impl AgentState {
    pub fn can_transition_to(&self, new_state: AgentState) -> bool {
        match (self, new_state) {
            // Standard flow
            (AgentState::Created, AgentState::Idle) => true,
            (AgentState::Idle, AgentState::Planning) => true,
            (AgentState::Planning, AgentState::Executing) => true,
            (AgentState::Executing, AgentState::Validating) => true,
            (AgentState::Validating, AgentState::Reporting) => true,
            (AgentState::Reporting, AgentState::Completed) => true,
            (AgentState::Reporting, AgentState::Idle) => true,
            
            // Pausing
            (_, AgentState::Paused) => true,
            (AgentState::Paused, AgentState::Idle) => true,
            
            // Errors can happen anytime
            (_, AgentState::Error) => true,
            
            // Recovery
            (AgentState::Error, AgentState::Idle) => true,
            
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn standard_flow_transitions() {
        let flow = vec![
            (AgentState::Created, AgentState::Idle, true),
            (AgentState::Idle, AgentState::Planning, true),
            (AgentState::Planning, AgentState::Executing, true),
            (AgentState::Executing, AgentState::Validating, true),
            (AgentState::Validating, AgentState::Reporting, true),
            (AgentState::Reporting, AgentState::Completed, true),
        ];
        for (from, to, expected) in flow {
            assert_eq!(from.can_transition_to(to), expected,
                "Expected {:?} -> {:?} = {}", from, to, expected);
        }
    }

    #[test]
    fn can_pause_from_any_state() {
        let states = vec![
            AgentState::Created,
            AgentState::Idle,
            AgentState::Planning,
            AgentState::Executing,
            AgentState::Validating,
            AgentState::Reporting,
            AgentState::Completed,
            AgentState::Error,
        ];
        for state in states {
            assert!(state.can_transition_to(AgentState::Paused),
                "Should be able to pause from {:?}", state);
        }
    }

    #[test]
    fn can_error_from_any_state() {
        let states = vec![
            AgentState::Created,
            AgentState::Idle,
            AgentState::Planning,
            AgentState::Executing,
            AgentState::Validating,
            AgentState::Reporting,
            AgentState::Paused,
            AgentState::Completed,
        ];
        for state in states {
            assert!(state.can_transition_to(AgentState::Error),
                "Should be able to error from {:?}", state);
        }
    }

    #[test]
    fn recovery_from_error_to_idle() {
        assert!(AgentState::Error.can_transition_to(AgentState::Idle));
    }

    #[test]
    fn resume_from_paused_to_idle() {
        assert!(AgentState::Paused.can_transition_to(AgentState::Idle));
    }

    #[test]
    fn reporting_can_loop_back_to_idle() {
        assert!(AgentState::Reporting.can_transition_to(AgentState::Idle));
    }

    #[test]
    fn illegal_transitions() {
        assert!(!AgentState::Created.can_transition_to(AgentState::Completed));
        assert!(!AgentState::Idle.can_transition_to(AgentState::Completed));
        assert!(!AgentState::Planning.can_transition_to(AgentState::Completed));
        assert!(!AgentState::Paused.can_transition_to(AgentState::Executing));
        assert!(!AgentState::Error.can_transition_to(AgentState::Completed));
    }

    #[test]
    fn acl_level_debug_clone_partial_eq() {
        let levels = vec![
            AgentAclLevel::Observer,
            AgentAclLevel::SemiAutonomous,
            AgentAclLevel::FullAuthority,
        ];
        for level in &levels {
            let cloned = *level;
            assert_eq!(*level, cloned);
            let _ = format!("{:?}", level);
        }
        assert_ne!(AgentAclLevel::Observer, AgentAclLevel::FullAuthority);
    }

    #[test]
    fn agent_state_serialization() {
        let states = vec![
            AgentState::Created,
            AgentState::Idle,
            AgentState::Planning,
            AgentState::Executing,
            AgentState::Validating,
            AgentState::Reporting,
            AgentState::Paused,
            AgentState::Completed,
            AgentState::Error,
        ];
        for state in states {
            let json = serde_json::to_string(&state).unwrap();
            let deserialized: AgentState = serde_json::from_str(&json).unwrap();
            assert_eq!(state, deserialized);
        }
    }

    #[test]
    fn acl_level_serialization() {
        let json = serde_json::to_string(&AgentAclLevel::FullAuthority).unwrap();
        assert_eq!(json, "\"FullAuthority\"");
        let deserialized: AgentAclLevel = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, AgentAclLevel::FullAuthority);
    }
}
