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
pub mod skill;

use std::sync::Mutex;
use tauri::{Manager, State};

use agents::intent_judge::IntentJudge;
use agents::diagnostic::DiagnosticAgent;
use agents::hardware::HardwareController;
use agents::system_manager::SystemManager;
use agents::document_analyst::DocumentAnalyst;
use agents::supervisor::SupervisorAgent;
use bridge::event_bus::EventBus;
use config::AppConfig;
use core::memory_manager::MemoryManager;
use core::orchestrator::Orchestrator;
use llm::context_manager::ContextManager;
use llm::claude::ClaudeClient;
use llm::OllamaClient;
use skill::parser::ManifestoParser;

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

#[tauri::command]
fn list_skills(state: State<AppState>) -> Result<Vec<skill::Skill>, String> {
    let orch = state.orchestrator.lock().map_err(|e| e.to_string())?;
    let registry = orch.skill_registry()
        .ok_or_else(|| "Skill registry aktif degil".to_string())?;
    registry.list()
}

#[tauri::command]
fn toggle_skill(state: State<AppState>, name: String) -> Result<String, String> {
    let orch = state.orchestrator.lock().map_err(|e| e.to_string())?;
    let registry = orch.skill_registry()
        .ok_or_else(|| "Skill registry aktif degil".to_string())?;
    let skill = registry.get_by_name(&name)?
        .ok_or_else(|| format!("Skill '{}' bulunamadi", name))?;
    if skill.active {
        registry.deactivate(&name)?;
        Ok(format!("Skill '{}' pasif edildi", name))
    } else {
        registry.activate(&name)?;
        Ok(format!("Skill '{}' aktif edildi", name))
    }
}

#[tauri::command]
fn delete_skill(state: State<AppState>, name: String) -> Result<String, String> {
    let orch = state.orchestrator.lock().map_err(|e| e.to_string())?;
    let registry = orch.skill_registry()
        .ok_or_else(|| "Skill registry aktif degil".to_string())?;
    registry.remove(&name)?;
    Ok(format!("Skill '{}' silindi", name))
}

#[tauri::command]
fn add_skill_md(state: State<AppState>, content: String) -> Result<String, String> {
    let manifesto = ManifestoParser::parse(&content, "inline")?;
    let steps = ManifestoParser::manifesto_to_steps(&manifesto);
    let triggers = manifesto.triggers.clone();
    let evolution = manifesto.evolution.clone();

    let orch = state.orchestrator.lock().map_err(|e| e.to_string())?;
    let registry = orch.skill_registry()
        .ok_or_else(|| "Skill registry aktif degil".to_string())?;

    let id = registry.register(
        &manifesto.name,
        &manifesto.description,
        &triggers,
        &manifesto.approval,
        &steps,
        manifesto.logic.as_deref(),
        &evolution,
    )?;

    Ok(format!("Skill '{}' eklendi (id: {})", manifesto.name, id))
}

#[tauri::command]
fn run_skill_by_name(state: State<AppState>, name: String) -> Result<String, String> {
    let memory = state.memory.lock().map_err(|e| e.to_string())?;
    let llm = &state.llm;
    let orch = state.orchestrator.lock().map_err(|e| e.to_string())?;
    orch.run_skill_direct(&name, llm, Some(&memory))
}

fn init_app_state(app: &tauri::App, config: &AppConfig) -> Result<(), Box<dyn std::error::Error>> {
    let db_path = app
        .path()
        .app_data_dir()
        .unwrap_or_else(|_| std::path::PathBuf::from("."))
        .join(&config.db_path);

    if let Some(parent) = db_path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    let conn = db::open(&db_path)?;
    let ollama = OllamaClient::new(config.ollama_url.clone(), config.ollama_model.clone());

    let claude = config.claude_api_key.clone().map(|key| {
        ClaudeClient::new_with_config(key, config.claude_model.clone())
    });

    let skill_conn = std::sync::Arc::clone(&conn);
    let skill_registry = skill::registry::SkillRegistry::new(skill_conn);

    let memory = MemoryManager::new(conn, ollama.clone());
    let context = ContextManager::new(8192);

    let mut orchestrator = Orchestrator::new(config.resolve_approval_level())
        .with_skill_registry(skill_registry);
    if let Some(c) = claude {
        orchestrator = orchestrator.with_claude(c);
    }
    orchestrator.register_agent(Box::new(IntentJudge));
    orchestrator.register_agent(Box::new(DiagnosticAgent));
    orchestrator.register_agent(Box::new(HardwareController));
    orchestrator.register_agent(Box::new(agents::market_analyst::MarketAnalyst::new()));
    orchestrator.register_agent(Box::new(SystemManager));
    orchestrator.register_agent(Box::new(DocumentAnalyst));
    orchestrator.register_agent(Box::new(agents::voice_handler::VoiceHandler::new()));
    orchestrator.register_agent(Box::new(SupervisorAgent));

    let event_bus = Some(EventBus::new(app.handle().clone()));

    let mcp_server = mcp::server::McpServer::new(config.mcp_port);
    let cloned = mcp_server.clone();
    std::thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().expect("Failed to create tokio runtime for MCP");
        rt.block_on(async move {
            if let Err(e) = cloned.start().await {
                log::error!("MCP server error: {}", e);
            }
        });
    });

    app.manage(AppState {
        memory: Mutex::new(memory),
        context: Mutex::new(context),
        orchestrator: Mutex::new(orchestrator),
        event_bus: Mutex::new(event_bus),
        llm: ollama,
    });

    log::info!(
        "ADLER ASI initialized — 8 agents, MCP on :{}, approval={:?}",
        config.mcp_port,
        config.resolve_approval_level()
    );
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    env_logger::init();
    let config = AppConfig::load();

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            send_command,
            approve_action,
            reject_action,
            get_context,
            list_skills,
            toggle_skill,
            delete_skill,
            add_skill_md,
            run_skill_by_name,
        ])
        .setup(move |app| {
            init_app_state(app, &config)?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

pub fn run_headless(config: &AppConfig) -> Result<String, String> {
    env_logger::init();
    log::info!("ADLER ASI headless mode — config loaded from {:?}", AppConfig::config_path());

    let db_path = std::path::Path::new(&config.db_path);
    if let Some(parent) = db_path.parent() {
        std::fs::create_dir_all(parent).ok();
    }

    let conn = db::open(db_path).map_err(|e| e.to_string())?;
    let ollama = OllamaClient::new(config.ollama_url.clone(), config.ollama_model.clone());

    let claude = config.claude_api_key.clone().map(|key| {
        ClaudeClient::new_with_config(key, config.claude_model.clone())
    });

    let skill_conn = std::sync::Arc::clone(&conn);
    let skill_registry = skill::registry::SkillRegistry::new(skill_conn);

    let memory = MemoryManager::new(conn, ollama.clone());
    let context = ContextManager::new(8192);

    let mut orchestrator = Orchestrator::new(config.resolve_approval_level())
        .with_skill_registry(skill_registry);
    if let Some(c) = claude {
        orchestrator = orchestrator.with_claude(c);
    }
    orchestrator.register_agent(Box::new(IntentJudge));
    orchestrator.register_agent(Box::new(DiagnosticAgent));
    orchestrator.register_agent(Box::new(HardwareController));
    orchestrator.register_agent(Box::new(agents::market_analyst::MarketAnalyst::new()));
    orchestrator.register_agent(Box::new(SystemManager));
    orchestrator.register_agent(Box::new(DocumentAnalyst));
    orchestrator.register_agent(Box::new(agents::voice_handler::VoiceHandler::new()));
    orchestrator.register_agent(Box::new(SupervisorAgent));

    log::info!("Headless orchestrator ready — {} agents, skill registry active", 8);
    drop(context);

    let mcp_server = mcp::server::McpServer::new(config.mcp_port);
    std::thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().expect("Failed to create tokio runtime for MCP");
        rt.block_on(async move {
            if let Err(e) = mcp_server.start().await {
                log::error!("MCP server error (headless): {}", e);
            }
        });
    });

    let result = orchestrator.run_pipeline("sistem durumu", &ollama, Some(&memory), None)?;
    Ok(result)
}
