use tokio::sync::broadcast;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SystemEvent {
    AgentReady(String),
    LlmResponse { request_id: String, text: String },
    HardwareTrigger { pin: u8, state: bool },
    SystemHeartbeat,
    ShutdownInitiated,
    AgentMessage(crate::protocol::AgentMessage),
    AgentStateChanged { agent_id: String, old_state: crate::agent_lifecycle::AgentState, new_state: crate::agent_lifecycle::AgentState },
    Error(String),
    HardwareAction { device_id: String, action: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventEnvelope {
    pub id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub payload: SystemEvent,
}

impl EventEnvelope {
    pub fn new(payload: SystemEvent) -> Self {
        Self {
            id: Uuid::new_v4(),
            timestamp: Utc::now(),
            payload,
        }
    }
}

#[derive(Clone)]
pub struct EventBus {
    sender: broadcast::Sender<EventEnvelope>,
}

impl EventBus {
    pub fn new(capacity: usize) -> Self {
        let (sender, _) = broadcast::channel(capacity);
        Self { sender }
    }

    pub fn subscribe(&self) -> broadcast::Receiver<EventEnvelope> {
        self.sender.subscribe()
    }

    pub fn publish(&self, event: SystemEvent) -> Result<usize, broadcast::error::SendError<EventEnvelope>> {
        let envelope = EventEnvelope::new(event);
        self.sender.send(envelope)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn event_envelope_has_unique_ids() {
        let e1 = EventEnvelope::new(SystemEvent::SystemHeartbeat);
        let e2 = EventEnvelope::new(SystemEvent::SystemHeartbeat);
        assert_ne!(e1.id, e2.id);
    }

    #[test]
    fn event_envelope_timestamps() {
        let e = EventEnvelope::new(SystemEvent::SystemHeartbeat);
        let now = Utc::now();
        let diff = now - e.timestamp;
        assert!(diff.num_seconds() < 5);
    }

    #[test]
    fn event_bus_publish_subscribe() {
        let bus = EventBus::new(16);
        let mut rx = bus.subscribe();
        let n = bus.publish(SystemEvent::SystemHeartbeat).unwrap();
        assert_eq!(n, 1);
        let received = rx.try_recv().unwrap();
        assert!(matches!(received.payload, SystemEvent::SystemHeartbeat));
    }

    #[test]
    fn event_bus_multiple_subscribers() {
        let bus = EventBus::new(16);
        let mut rx1 = bus.subscribe();
        let mut rx2 = bus.subscribe();
        bus.publish(SystemEvent::SystemHeartbeat).unwrap();
        assert!(rx1.try_recv().is_ok());
        assert!(rx2.try_recv().is_ok());
    }

    #[test]
    fn system_event_variants_serialize() {
        let events = vec![
            SystemEvent::AgentReady("test_agent".into()),
            SystemEvent::LlmResponse { request_id: "req-1".into(), text: "response".into() },
            SystemEvent::HardwareTrigger { pin: 17, state: true },
            SystemEvent::SystemHeartbeat,
            SystemEvent::ShutdownInitiated,
        ];
        for event in events {
            let json = serde_json::to_string(&event).unwrap();
            let deserialized: SystemEvent = serde_json::from_str(&json).unwrap();
            let _ = format!("{:?}", deserialized);
        }
    }

    #[test]
    fn event_bus_capacity_limit() {
        let bus = EventBus::new(2);
        let _rx = bus.subscribe();
        bus.publish(SystemEvent::SystemHeartbeat).unwrap();
        bus.publish(SystemEvent::SystemHeartbeat).unwrap();
        // Third publish with no receiver reading — should still work (broadcast)
        let result = bus.publish(SystemEvent::SystemHeartbeat);
        assert!(result.is_ok());
    }
}
