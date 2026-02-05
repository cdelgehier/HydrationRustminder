mod config;
mod notifier;
mod ui;

use config::Config;
use log::{error, info};
use notifier::Notifier;
use std::process;
use ui::{MenuManager, TrayManager};
use winit::event_loop::{ControlFlow, EventLoop};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    info!("HydrationRustminder starting...");

    // Load config
    let mut config = Config::load();
    info!("Config loaded: {:?}", config);

    // Create notifier
    let notifier = Notifier::new();
    if let Err(e) = notifier.send_startup_notification() {
        error!("Failed to send startup notification: {}", e);
    }

    // Create event loop (required for macOS tray icon)
    let event_loop = EventLoop::new()?;
    event_loop.set_control_flow(ControlFlow::Wait);

    // Build menu
    let menu_manager = MenuManager::new(&config)?;

    // Create tray icon on the main thread
    let tray = TrayManager::new(menu_manager.menu())?;
    info!("Tray icon created");

    // Run the event loop
    #[allow(deprecated)]
    event_loop.run(move |_event, elwt| {
        // Handle tray events
        if let Some(event) = tray.handle_events() {
            info!("Menu event: {:?}", event);

            let event_id = event.id.0.as_str();

            // Handle start hour changes
            if event_id.starts_with("start_") {
                if let Ok(hour) = event_id.strip_prefix("start_").unwrap().parse::<u8>() {
                    info!("Changing start hour to {}", hour);
                    config.start_hour = hour;
                    menu_manager.update_start_hour(hour);
                    if let Err(e) = config.save() {
                        error!("Failed to save config: {}", e);
                    }
                }
            }
            // Handle end hour changes
            else if event_id.starts_with("end_") {
                if let Ok(hour) = event_id.strip_prefix("end_").unwrap().parse::<u8>() {
                    info!("Changing end hour to {}", hour);
                    config.end_hour = hour;
                    menu_manager.update_end_hour(hour);
                    if let Err(e) = config.save() {
                        error!("Failed to save config: {}", e);
                    }
                }
            }
            // Handle interval changes
            else if event_id.starts_with("interval_") {
                if let Ok(interval) = event_id.strip_prefix("interval_").unwrap().parse::<u32>() {
                    info!("Changing interval to {} minutes", interval);
                    config.interval_minutes = interval;
                    menu_manager.update_interval(interval);
                    if let Err(e) = config.save() {
                        error!("Failed to save config: {}", e);
                    }
                }
            }
            // Handle pause
            else if event_id == "pause" {
                info!("Pause requested (not implemented yet)");
            }
            // Handle quit
            else if event_id == "quit" {
                info!("Quit requested");
                process::exit(0);
            }
        }

        // Keep the loop running
        elwt.set_control_flow(ControlFlow::Wait);
    })?;

    Ok(())
}
