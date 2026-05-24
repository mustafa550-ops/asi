pub mod event_bus;
pub mod command_router;
pub mod fs_api;

use tauri::App;

pub fn register_commands(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    log::info!("Tauri Bridge: Command router initialized");
    let _app_handle = app.handle();
    Ok(())
}
