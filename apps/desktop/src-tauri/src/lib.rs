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
pub mod server;
pub mod commands;
pub mod events;
pub mod nlu;
pub mod rag;

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
use db::embeddings::EmbeddingStore;
use db::edge_history::EdgeHistory;
use db::strategic_memory::StrategicMemory;
use db::fts::FullTextSearch;
use db::sessions::SessionsStore;
use rag::pipeline::RagPipeline;

pub struct AppState {
    pub memory: Mutex<MemoryManager>,
    pub context: Mutex<ContextManager>,
    pub orchestrator: Mutex<Orchestrator>,
    pub event_bus: Mutex<Option<EventBus>>,
    pub llm: OllamaClient,
    pub rag: Mutex<Option<RagPipeline>>,
    pub sessions: Mutex<SessionsStore>,
    pub hardware: Mutex<Option<agents::hardware::HardwareController>>,
}

#[tauri::command]
fn send_command(state: State<AppState>, command: String) -> Result<String, String> {
    let session_id = "default";
    let llm = &state.llm;
    let memory = state.memory.lock().map_err(|e| e.to_string())?;
    let mut context = state.context.lock().map_err(|e| e.to_string())?;
    let event_bus_guard = state.event_bus.lock().map_err(|e| e.to_string())?;
    let orchestrator = state.orchestrator.lock().map_err(|e| e.to_string())?;
    let sessions = state.sessions.lock().map_err(|e| e.to_string())?;

    sessions.create_session(session_id, &command.chars().take(40).collect::<String>()).ok();
    sessions.add_message(session_id, "user", &command).ok();

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
        sessions.add_message(session_id, "assistant", report).ok();
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

#[tauri::command]
fn run_code(_state: State<AppState>, code: String) -> Result<String, String> {
    let executor = crate::skill::executor::SkillExecutor::new();
    let result = executor.execute_code(&code)?;
    Ok(result)
}

#[tauri::command]
fn start_listening() -> Result<String, String> {
    log::info!("Tauri command: start_listening");
    Ok("Started listening".into())
}

#[tauri::command]
fn stop_listening() -> Result<String, String> {
    log::info!("Tauri command: stop_listening");
    Ok("Mock recorded text from Tauri".into())
}

#[tauri::command]
fn synthesize_speech(text: String) -> Result<String, String> {
    log::info!("Tauri command: synthesize_speech with text: {}", text);
    Ok("Synthesizing".into())
}

#[tauri::command]
async fn process_voice_command(transcript: String) -> Result<String, String> {
    log::info!("Tauri command: process_voice_command: {}", transcript);
    // crate::assimilation::jarvis_voice_chat::VoiceProcessor::process_command(&transcript).await
    Ok(format!("Processed: {}", transcript))
}

#[tauri::command]
fn hybrid_search(state: State<AppState>, query: String, limit: usize) -> Result<String, String> {
    let rag_guard = state.rag.lock().map_err(|e| e.to_string())?;
    match rag_guard.as_ref() {
        Some(pipeline) => {
            let results = pipeline.hybrid_search(&query, limit)?;
            serde_json::to_string(&results).map_err(|e| e.to_string())
        }
        None => Err("RAG sistemi hazir degil".to_string()),
    }
}

#[tauri::command]
fn query_strategic(state: State<AppState>, context: String) -> Result<String, String> {
    let rag_guard = state.rag.lock().map_err(|e| e.to_string())?;
    match rag_guard.as_ref() {
        Some(pipeline) => {
            let decisions = pipeline.query_strategic(&context)?;
            Ok(decisions.join("\n"))
        }
        None => Err("RAG sistemi hazir degil".to_string()),
    }
}

#[tauri::command]
fn get_knowledge_graph(state: State<AppState>, node_id: i64) -> Result<String, String> {
    let rag_guard = state.rag.lock().map_err(|e| e.to_string())?;
    match rag_guard.as_ref() {
        Some(pipeline) => {
            let graph = pipeline.get_knowledge_graph(node_id)?;
            serde_json::to_string(&graph).map_err(|e| e.to_string())
        }
        None => Err("RAG sistemi hazir degil".to_string()),
    }
}

#[tauri::command]
fn save_setting(state: State<AppState>, key: String, value: String) -> Result<String, String> {
    let mut memory = state.memory.lock().map_err(|e| e.to_string())?;
    memory.push_short_term(format!("config:{}:{}", key, value));
    log::info!("Setting saved: {} = {}", key, value);
    Ok(format!("{} kaydedildi", key))
}

#[tauri::command]
fn list_chat_sessions(state: State<AppState>, limit: usize) -> Result<String, String> {
    let sessions = state.sessions.lock().map_err(|e| e.to_string())?;
    let list = sessions.list_sessions(limit).map_err(|e| e.to_string())?;
    serde_json::to_string(&list).map_err(|e| e.to_string())
}

#[tauri::command]
fn delete_chat_session(state: State<AppState>, session_id: String) -> Result<String, String> {
    let sessions = state.sessions.lock().map_err(|e| e.to_string())?;
    sessions.delete_session(&session_id).map_err(|e| e.to_string())?;
    Ok("Silindi".into())
}

#[tauri::command]
fn gpio_read(state: State<AppState>, pin: u8) -> Result<String, String> {
    let hw = state.hardware.lock().map_err(|e| e.to_string())?;
    match hw.as_ref() {
        Some(ctrl) => ctrl.gpio.read(pin),
        None => Err("Hardware controller not initialized".into()),
    }
}

#[tauri::command]
fn gpio_write(state: State<AppState>, pin: u8, value: bool) -> Result<String, String> {
    let hw = state.hardware.lock().map_err(|e| e.to_string())?;
    match hw.as_ref() {
        Some(ctrl) => ctrl.gpio.write(pin, value).map(|_| format!("GPIO pin {} -> {}", pin, if value { "HIGH" } else { "LOW" })),
        None => Err("Hardware controller not initialized".into()),
    }
}

#[tauri::command]
fn sensor_read(state: State<AppState>) -> Result<String, String> {
    let hw = state.hardware.lock().map_err(|e| e.to_string())?;
    match hw.as_ref() {
        Some(ctrl) => ctrl.sensor.read_all(),
        None => Err("Hardware controller not initialized".into()),
    }
}

#[tauri::command]
fn relay_set(state: State<AppState>, pin: u8, on: bool) -> Result<String, String> {
    let hw = state.hardware.lock().map_err(|e| e.to_string())?;
    match hw.as_ref() {
        Some(ctrl) => ctrl.relay.set(pin, on).map(|_| format!("Relay pin {} -> {}", pin, if on { "ON" } else { "OFF" })),
        None => Err("Hardware controller not initialized".into()),
    }
}

#[tauri::command]
fn hw_config_get() -> Result<String, String> {
    let cfg = crate::config::AppConfig::load();
    serde_json::to_string(&cfg.hardware).map_err(|e| e.to_string())
}

#[tauri::command]
fn hw_detect() -> Result<String, String> {
    let devices = agents::hardware::detect::auto_detect()?;
    serde_json::to_string(&devices).map_err(|e| e.to_string())
}

#[tauri::command]
fn get_system_metrics(state: State<AppState>) -> Result<String, String> {
    let memory = state.memory.lock().map_err(|e| e.to_string())?;
    let context = memory.get_short_term_context();
    let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();
    let cpu: f64 = 45.0 + ((now % 30) as f64 / 30.0) * 30.0;
    let mem: f64 = 60.0 + ((now % 20) as f64 / 20.0) * 20.0;
    let metrics = serde_json::json!({
        "cpu": cpu,
        "memory": mem,
        "uptime": format!("{}s", context.lines().count()),
        "active_agents": 8u32,
    });
    serde_json::to_string(&metrics).map_err(|e| e.to_string())
}

#[tauri::command]
fn get_agent_statuses(state: State<AppState>) -> Result<String, String> {
    let orchestrator = state.orchestrator.lock().map_err(|e| e.to_string())?;
    let agents = orchestrator.list_agents();
    serde_json::to_string(&agents).map_err(|e| e.to_string())
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

    let rag_conn = std::sync::Arc::clone(&conn);
    let sessions_conn = std::sync::Arc::clone(&conn);
    let embeddings = EmbeddingStore::new(rag_conn.clone());
    let fts = FullTextSearch::new(rag_conn.clone());
    let edge = EdgeHistory::new(rag_conn.clone());
    let strategic = StrategicMemory::new(rag_conn);
    let rag_pipeline = RagPipeline::new(embeddings, fts, edge, strategic, ollama.clone());

    let sessions = SessionsStore::new(sessions_conn);

    let memory = MemoryManager::new(conn, ollama.clone());
    let context = ContextManager::new(8192);

    let mut orchestrator = Orchestrator::new(config.resolve_approval_level())
        .with_skill_registry(skill_registry);
    if let Some(c) = claude {
        orchestrator = orchestrator.with_claude(c);
    }
    orchestrator.register_agent(Box::new(IntentJudge));
    orchestrator.register_agent(Box::new(DiagnosticAgent));
    orchestrator.register_agent(Box::new(HardwareController::new_real()));
    orchestrator.register_agent(Box::new(agents::market_analyst::MarketAnalyst::new()));
    orchestrator.register_agent(Box::new(SystemManager));
    orchestrator.register_agent(Box::new(DocumentAnalyst));
    orchestrator.register_agent(Box::new(agents::voice_handler::VoiceHandler::new()));
    orchestrator.register_agent(Box::new(SupervisorAgent));

    let event_bus = Some(EventBus::new(app.handle().clone()));

    let mcp_server = mcp::server::McpServer::new(config.mcp_port);
    let cloned = mcp_server.clone();
    tauri::async_runtime::spawn(async move {
        if let Err(e) = cloned.start().await {
            log::error!("MCP server error: {}", e);
        }
    });

    app.manage(AppState {
        memory: Mutex::new(memory),
        context: Mutex::new(context),
        orchestrator: Mutex::new(orchestrator),
        event_bus: Mutex::new(event_bus),
        llm: ollama,
        rag: Mutex::new(Some(rag_pipeline)),
        sessions: Mutex::new(sessions),
        hardware: Mutex::new(Some(agents::hardware::HardwareController::new_real())),
    });

    tauri::async_runtime::spawn(server::start_server(app.handle().clone()));

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
            run_code,
            start_listening,
            stop_listening,
            synthesize_speech,
            process_voice_command,
            hybrid_search,
            query_strategic,
            get_knowledge_graph,
            save_setting,
            get_system_metrics,
            commands::get_system_status,
            commands::execute_agent_command,
            commands::get_recent_memory,
            list_chat_sessions,
            delete_chat_session,
            get_agent_statuses,
            gpio_read,
            gpio_write,
            sensor_read,
            relay_set,
            hw_config_get,
            hw_detect,
        ])
        .setup(move |app| {
            // Setup Adler Core Bridge
            let adler_config = adler_core::config::AppConfig::load("../../config/adler.yaml").expect("Failed to load adler core config");
            let db_path = app
                .path()
                .app_data_dir()
                .unwrap_or_else(|_| std::path::PathBuf::from("."))
                .join(&adler_config.memory.db_path);
                
            let memory_manager = adler_core::memory::MemoryManager::new(&db_path)
                .expect("Failed to initialize Adler Core Memory");
                
            let core_state = adler_core::state::create_shared_state(adler_config, memory_manager);
            
            // Bridge EventBus to Tauri
            // We need a way to get the event bus from the lock. Let's clone it.
            let core_event_bus = tokio::task::block_in_place(|| {
                tokio::runtime::Handle::current().block_on(async {
                    core_state.read().await.event_bus.clone()
                })
            });
            
            events::setup_event_bridge(app.handle().clone(), core_event_bus);
            app.manage(core_state);

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
    orchestrator.register_agent(Box::new(HardwareController::new_real()));
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
