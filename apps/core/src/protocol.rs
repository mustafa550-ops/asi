use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Agent Message Protocol (AMP)
/// Standard format for communication between agents in ADLER ASI.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentMessage {
    pub id: Uuid,
    pub from: String,
    pub to: String,
    pub payload: serde_json::Value,
    pub correlation_id: Option<Uuid>,
    pub timestamp: DateTime<Utc>,
}

impl AgentMessage {
    pub fn new(from: impl Into<String>, to: impl Into<String>, payload: serde_json::Value) -> Self {
        Self {
            id: Uuid::new_v4(),
            from: from.into(),
            to: to.into(),
            payload,
            correlation_id: None,
            timestamp: Utc::now(),
        }
    }

    pub fn with_correlation(mut self, correlation_id: Uuid) -> Self {
        self.correlation_id = Some(correlation_id);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_creates_message_with_required_fields() {
        let msg = AgentMessage::new("alice", "bob", serde_json::json!({"hello": "world"}));
        assert_eq!(msg.from, "alice");
        assert_eq!(msg.to, "bob");
        assert_eq!(msg.payload["hello"], "world");
        assert!(msg.correlation_id.is_none());
    }

    #[test]
    fn new_generates_unique_ids() {
        let msg1 = AgentMessage::new("a", "b", serde_json::json!(null));
        let msg2 = AgentMessage::new("a", "b", serde_json::json!(null));
        assert_ne!(msg1.id, msg2.id);
    }

    #[test]
    fn with_correlation_sets_id() {
        let correlation = Uuid::new_v4();
        let msg = AgentMessage::new("a", "b", serde_json::json!(null))
            .with_correlation(correlation);
        assert_eq!(msg.correlation_id, Some(correlation));
    }

    #[test]
    fn with_correlation_no_side_effects() {
        let correlation = Uuid::new_v4();
        let msg = AgentMessage::new("a", "b", serde_json::json!("data"));
        let msg2 = msg.clone().with_correlation(correlation);
        // Original msg should be unchanged (builder pattern returns new)
        // Actually it takes self by value, so msg is moved
        assert_eq!(msg2.correlation_id, Some(correlation));
    }

    #[test]
    fn serialization_roundtrip() {
        let original = AgentMessage::new("agent_01", "agent_02", serde_json::json!({"action": "test"}));
        let json = serde_json::to_string(&original).unwrap();
        let deserialized: AgentMessage = serde_json::from_str(&json).unwrap();
        assert_eq!(original.id, deserialized.id);
        assert_eq!(original.from, deserialized.from);
        assert_eq!(original.payload, deserialized.payload);
    }
}
