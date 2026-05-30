use std::sync::Arc;
use axum::{routing::get, Router};
use tracing::{info, error};

use adler_core::config::AppConfig;
use adler_core::state::create_shared_state;
use adler_core::events::EventBus;
use adler_core::memory::MemoryManager;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Initialize logging
    adler_core::logging::init_logging();
    info!("Starting ADLER ASI Core Kernel...");

    // 2. Load configuration
    let config_path = std::env::var("ADLER_CONFIG_PATH").unwrap_or_else(|_| "../../config/adler.yaml".to_string());
    let app_config = match AppConfig::load(&config_path) {
        Ok(c) => c,
        Err(e) => {
            error!("Failed to load configuration from {}: {}", config_path, e);
            std::process::exit(1);
        }
    };
    info!("Configuration loaded successfully.");

    // 3. Initialize Memory Manager
    let db_path = &app_config.memory.db_path;
    let memory_manager = match MemoryManager::new(db_path) {
        Ok(m) => m,
        Err(e) => {
            error!("Failed to initialize Memory Manager at {}: {}", db_path, e);
            std::process::exit(1);
        }
    };
    info!("Memory Manager (SQLite) initialized at {}.", db_path);

    // 4. Initialize Global State and Event Bus
    let state = create_shared_state(app_config, memory_manager);
    
    // We clone the sender directly from state to pass to scheduler
    let event_bus = {
        let s = state.read().await;
        Arc::new(s.event_bus.clone())
    };

    // 4. Start Health Check HTTP server
    let app = Router::new()
        .route("/health", get(adler_core::health::health_check))
        .with_state(state.clone());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:9876").await?;
    info!("Health check server listening on http://127.0.0.1:9876/health");
    
    // Run the server in background
    let server_handle = tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });

    // 5. Start Heartbeat Scheduler
    let event_bus_scheduler = event_bus.clone();
    tokio::spawn(async move {
        adler_core::scheduler::start_heartbeat(event_bus_scheduler).await;
    });

    // 6. Wait for Shutdown Signal
    adler_core::shutdown::wait_for_shutdown(&event_bus).await;

    server_handle.abort();
    info!("ADLER ASI Core Kernel stopped gracefully.");

    Ok(())
}
