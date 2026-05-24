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

use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    env_logger::init();

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            let db_path = app.path().app_data_dir()
                .unwrap_or_else(|_| std::path::PathBuf::from("."))
                .join("adler.db");
            let _db = db::init(&db_path)
                .expect("Failed to initialize database");

            let _llm = llm::OllamaClient::new("http://localhost:11434".to_string());

            bridge::register_commands(app)?;

            log::info!("ADLER ASI initialized successfully");
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
