use tauri::{AppHandle, Emitter};
use tokio::task;

use adler_core::events::{EventBus, SystemEvent};
use serde_json::json;

pub fn setup_event_bridge(app_handle: AppHandle, event_bus: EventBus) {
    task::spawn(async move {
        // Subscribe to the core's broadcast channel
        let mut rx = event_bus.subscribe();
        
        while let Ok(envelope) = rx.recv().await {
            match envelope.payload {
                SystemEvent::AgentMessage(msg) => {
                    let _ = app_handle.emit(
                        "core-agent-message",
                        json!({
                            "from": msg.from,
                            "to": msg.to,
                            "correlation_id": msg.correlation_id,
                            "payload": msg.payload
                        })
                    );
                }
                SystemEvent::AgentStateChanged { agent_id, old_state, new_state } => {
                    let _ = app_handle.emit(
                        "core-agent-state",
                        json!({
                            "agent_id": agent_id,
                            "old_state": format!("{:?}", old_state),
                            "new_state": format!("{:?}", new_state)
                        })
                    );
                }
                SystemEvent::Error(err) => {
                    let _ = app_handle.emit(
                        "core-error",
                        json!({ "error": err })
                    );
                }
                SystemEvent::HardwareAction { device_id, action } => {
                    let _ = app_handle.emit(
                        "core-hardware",
                        json!({
                            "device_id": device_id,
                            "action": action
                        })
                    );
                }
                _ => {} // Ignore other events
            }
        }
    });
}
