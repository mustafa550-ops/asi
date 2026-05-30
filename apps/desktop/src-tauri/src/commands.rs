use tauri::State;
use serde_json::Value;

use adler_core::state::AppState;
use adler_core::protocol::AgentMessage;
use adler_core::events::SystemEvent;
use adler_core::error::Result as CoreResult;

#[tauri::command]
pub async fn get_system_status(state: State<'_, AppState>) -> Result<String, String> {
    let s = state.read().await;
    Ok(format!("ADLER Core is active with {} agents.", s.active_agents))
}

#[tauri::command]
pub async fn execute_agent_command(
    agent_id: String,
    payload: Value,
    state: State<'_, AppState>
) -> Result<String, String> {
    let s = state.read().await;
    
    // In a real flow, this would get the orchestrator and execute the command.
    // For now, we will just send an event to the bus to prove the bridge works.
    let msg = AgentMessage::new("tauri_ui".to_string(), agent_id, payload);
    
    let _ = s.event_bus.publish(SystemEvent::AgentMessage(msg));
    
    Ok("Command dispatched to core.".to_string())
}

#[tauri::command]
pub async fn get_recent_memory(
    agent_id: String,
    limit: usize,
    state: State<'_, AppState>
) -> Result<Value, String> {
    let s = state.read().await;
    
    let session_mem = adler_core::memory::session::SessionMemory::new(&s.memory);
    
    match session_mem.get_recent(&agent_id, limit) {
        Ok(msgs) => Ok(serde_json::to_value(msgs).unwrap_or(Value::Null)),
        Err(e) => Err(format!("Failed to retrieve memory: {}", e)),
    }
}
