pub mod bridge;
pub mod core;
pub mod agents;
pub mod db;
pub mod llm;
pub mod assimilation;
pub mod mcp;
pub mod cli;
pub mod security;
pub mod config;

use std::sync::Mutex;
use tauri::{Manager, State};

use agents::intent_judge::IntentJudge;
use agents::diagnostic::DiagnosticAgent;
use agents::hardware::HardwareController;
use agents::market_analyst::MarketAnalyst;
use agents::system_manager::SystemManager;
use agents::document_analyst::DocumentAnalyst;
use agents::voice_handler::VoiceHandler;
use agents::supervisor::SupervisorAgent;
use agents::ApprovalLevel;
use bridge::event_bus::EventBus;
use core::memory_manager::MemoryManager;
use core::orchestrator::Orchestrator;
use llm::context_manager::ContextManager;
use llm::OllamaClient;

pub struct AppState {
    pub memory: Mutex<MemoryManager>,
    pub context: Mutex<ContextManager>,
    pub orchestrator: Mutex<Orchestrator>,
    pub event_bus: Mutex<Option<EventBus>>,
    pub llm: OllamaClient,
}

#[tauri::command]
fn send_command(state: State<AppState>, command: String) -> Result<String, String> {
    let llm = &state.llm;
    let memory = state.memory.lock().map_err(|e| e.to_string())?;
    let mut context = state.context.lock().map_err(|e| e.to_string())?;
    let event_bus_guard = state.event_bus.lock().map_err(|e| e.to_string())?;
    let orchestrator = state.orchestrator.lock().map_err(|e| e.to_string())?;

    context.push("user", &command);
    if let Some(bus) = event_bus_guard.as_ref() {
        bus.emit("user-command", &command);
    }

    let result = orchestrator.run_pipeline(
        &command,
        llm,
        Some(&memory),
        event_bus_guard.as_ref(),
    );

    if let Ok(ref report) = result {
        context.push("adler", report);
    }

    result
}

#[tauri::command]
fn approve_action(state: State<AppState>, id: String) -> Result<String, String> {
    let orchestrator = state.orchestrator.lock().map_err(|e| e.to_string())?;
    orchestrator.approve(&id)
}

#[tauri::command]
fn reject_action(state: State<AppState>, id: String) -> Result<String, String> {
    let orchestrator = state.orchestrator.lock().map_err(|e| e.to_string())?;
    orchestrator.reject(&id).map(|_| "Rejected".into())
}

#[tauri::command]
fn get_context(state: State<AppState>) -> Result<String, String> {
    let context = state.context.lock().map_err(|e| e.to_string())?;
    Ok(context.build_prompt())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    env_logger::init();

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            send_command,
            approve_action,
            reject_action,
            get_context,
        ])
        .setup(|app| {
            let db_path = app
                .path()
                .app_data_dir()
                .unwrap_or_else(|_| std::path::PathBuf::from("."))
                .join("adler.db");

            if let Some(parent) = db_path.parent() {
                std::fs::create_dir_all(parent).ok();
            }

            let conn = db::open(&db_path).expect("Failed to initialize database");
            let ollama = OllamaClient::new("http://localhost:11434".to_string());

            let memory = MemoryManager::new(conn, ollama.clone());
            let context = ContextManager::new(8192);

            let mut orchestrator = Orchestrator::new(ApprovalLevel::SemiAutonomous);
            orchestrator.register_agent(Box::new(IntentJudge));
            orchestrator.register_agent(Box::new(DiagnosticAgent));
            orchestrator.register_agent(Box::new(HardwareController));
            orchestrator.register_agent(Box::new(MarketAnalyst));
            orchestrator.register_agent(Box::new(SystemManager));
            orchestrator.register_agent(Box::new(DocumentAnalyst));
            orchestrator.register_agent(Box::new(VoiceHandler));
            orchestrator.register_agent(Box::new(SupervisorAgent));

            let event_bus = Some(EventBus::new(app.handle().clone()));

            app.manage(AppState {
                memory: Mutex::new(memory),
                context: Mutex::new(context),
                orchestrator: Mutex::new(orchestrator),
                event_bus: Mutex::new(event_bus),
                llm: ollama,
            });

            log::info!("ADLER ASI initialized — 8 agents registered, all systems go");
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
