use tokio::signal;
use tracing::{info, warn};

use crate::events::{EventBus, SystemEvent};

pub async fn wait_for_shutdown(event_bus: &EventBus) {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("Failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("Failed to install SIGTERM handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {
            info!("Ctrl+C received. Initiating graceful shutdown...");
        },
        _ = terminate => {
            info!("SIGTERM received. Initiating graceful shutdown...");
        },
    }

    // Publish shutdown event so all agents and subsystems can clean up
    if let Err(e) = event_bus.publish(SystemEvent::ShutdownInitiated) {
        warn!("Failed to broadcast shutdown event: {}", e);
    }
}
