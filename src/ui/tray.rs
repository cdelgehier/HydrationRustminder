use log::info;
use muda::{Menu, MenuEvent};
use std::error::Error;
use tray_icon::{Icon, TrayIcon, TrayIconBuilder};

pub struct TrayManager {
    _tray_icon: TrayIcon,
}

impl TrayManager {
    pub fn new(menu: &Menu) -> Result<Self, Box<dyn Error>> {
        info!("Creating tray icon...");

        // Load icon
        let icon_bytes = include_bytes!("../../droplet.png");
        info!("Icon loaded, size: {} bytes", icon_bytes.len());

        let img = image::load_from_memory(icon_bytes)?;
        info!("Image dimensions: {}x{}", img.width(), img.height());

        let icon = Icon::from_rgba(img.to_rgba8().into_raw(), img.width(), img.height())?;

        // Create tray icon
        let tray_icon = TrayIconBuilder::new()
            .with_menu(Box::new(menu.clone()))
            .with_tooltip("HydrationRustminder")
            .with_icon(icon)
            .build()?;

        info!("Tray icon created successfully");

        Ok(TrayManager {
            _tray_icon: tray_icon,
        })
    }

    pub fn handle_events(&self) -> Option<MenuEvent> {
        MenuEvent::receiver().try_recv().ok()
    }
}
