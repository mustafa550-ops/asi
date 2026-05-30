use tokio::time::{interval, Duration};
use tracing::info;

use crate::events::{EventBus, SystemEvent};

pub async fn start_heartbeat(event_bus: std::sync::Arc<EventBus>) {
    let mut ticker = interval(Duration::from_secs(60));
    
    // First tick fires immediately, we can skip it if we want
    ticker.tick().await;

    loop {
        ticker.tick().await;
        info!("System heartbeat tick");
        let _ = event_bus.publish(SystemEvent::SystemHeartbeat);
    }
}
