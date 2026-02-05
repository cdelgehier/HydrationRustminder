mod config;
mod notifier;
mod tray;

use config::Config;
use log::{error, info};
use notifier::Notifier;
use tray::TrayManager;
use winit::event_loop::{ControlFlow, EventLoop};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    info!("HydrationRustminder starting...");

    // Load config
    let config = Config::load();
    info!("Config loaded: {:?}", config);

    // Create notifier
    let notifier = Notifier::new();
    if let Err(e) = notifier.send_startup_notification() {
        error!("Failed to send startup notification: {}", e);
    }

    // Create event loop (required for macOS tray icon)
    let event_loop = EventLoop::new()?;
    event_loop.set_control_flow(ControlFlow::Wait);

    // Create tray icon on the main thread
    let tray = TrayManager::new()?;
    info!("Tray icon created");

    // Run the event loop
    #[allow(deprecated)]
    event_loop.run(move |_event, elwt| {
        // Handle tray events
        tray.handle_events();

        // Keep the loop running
        elwt.set_control_flow(ControlFlow::Wait);
    })?;

    Ok(())
}
