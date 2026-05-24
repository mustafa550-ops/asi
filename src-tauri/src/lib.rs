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
use tauri::Manager;

use core::memory_manager::MemoryManager;
use llm::context_manager::ContextManager;
use llm::OllamaClient;

pub struct AppState {
    pub memory: Mutex<MemoryManager>,
    pub context: Mutex<ContextManager>,
    pub llm: OllamaClient,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    env_logger::init();

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
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

            app.manage(AppState {
                memory: Mutex::new(memory),
                context: Mutex::new(context),
                llm: ollama,
            });

            bridge::register_commands(app)?;

            log::info!("ADLER ASI initialized successfully");
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
