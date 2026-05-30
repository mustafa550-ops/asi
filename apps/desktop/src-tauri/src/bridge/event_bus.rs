use tauri::Emitter;
use serde::Serialize;

/// Broadcast Event Bus — Rust'taki olayları React'e canlı aktarır.
/// Arayüz hiçbir zaman "sorgu yapmaz", sadece "gelen veriyi dinler" (§3.2).
#[derive(Clone)]
pub struct EventBus {
    app_handle: tauri::AppHandle,
}

#[derive(Debug, Clone, Serialize)]
pub struct AppEvent {
    pub event_type: String,
    pub payload: String,
    pub timestamp: u64,
}

impl EventBus {
    pub fn new(app_handle: tauri::AppHandle) -> Self {
        Self { app_handle }
    }

    pub fn emit(&self, event_type: &str, payload: &str) {
        let event = AppEvent {
            event_type: event_type.to_string(),
            payload: payload.to_string(),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
        };
        let _ = self.app_handle.emit("adler-event", event);
    }
}
