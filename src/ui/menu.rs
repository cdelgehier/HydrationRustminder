use crate::config::Config;
use log::info;
use muda::{CheckMenuItem, Menu, MenuId, MenuItem, PredefinedMenuItem};
use std::collections::HashMap;

pub struct MenuManager {
    menu: Menu,
    start_items: HashMap<u8, CheckMenuItem>,
    end_items: HashMap<u8, CheckMenuItem>,
    interval_items: HashMap<u32, CheckMenuItem>,
    #[allow(dead_code)]
    pause_item: MenuItem,
    #[allow(dead_code)]
    quit_item: MenuItem,
}

impl MenuManager {
    pub fn new(config: &Config) -> Result<Self, Box<dyn std::error::Error>> {
        info!("Building menu...");

        let menu = Menu::new();

        let mut start_items = HashMap::new();
        let mut end_items = HashMap::new();
        let mut interval_items = HashMap::new();

        // Start hour options
        for hour in 6..=12 {
            let label = format!("Start: {}h", hour);
            let item = CheckMenuItem::with_id(
                MenuId::new(format!("start_{}", hour)),
                label,
                true,
                hour == config.start_hour,
                None,
            );
            menu.append(&item)?;
            start_items.insert(hour, item);
        }

        menu.append(&PredefinedMenuItem::separator())?;

        // End hour options
        for hour in 16..=20 {
            let label = format!("End: {}h", hour);
            let item = CheckMenuItem::with_id(
                MenuId::new(format!("end_{}", hour)),
                label,
                true,
                hour == config.end_hour,
                None,
            );
            menu.append(&item)?;
            end_items.insert(hour, item);
        }

        menu.append(&PredefinedMenuItem::separator())?;

        // Interval options
        for interval in [15, 30, 45, 60] {
            let label = format!("Interval: {}min", interval);
            let item = CheckMenuItem::with_id(
                MenuId::new(format!("interval_{}", interval)),
                label,
                true,
                interval == config.interval_minutes,
                None,
            );
            menu.append(&item)?;
            interval_items.insert(interval, item);
        }

        menu.append(&PredefinedMenuItem::separator())?;

        // Pause option
        let pause_item = MenuItem::with_id(MenuId::new("pause"), "Pause 1h", true, None);
        menu.append(&pause_item)?;

        menu.append(&PredefinedMenuItem::separator())?;

        // Quit option
        let quit_item = MenuItem::with_id(MenuId::new("quit"), "Quit", true, None);
        menu.append(&quit_item)?;

        info!("Menu built successfully");

        Ok(MenuManager {
            menu,
            start_items,
            end_items,
            interval_items,
            pause_item,
            quit_item,
        })
    }

    pub fn menu(&self) -> &Menu {
        &self.menu
    }

    pub fn update_start_hour(&self, new_hour: u8) {
        // Uncheck all start items
        for item in self.start_items.values() {
            item.set_checked(false);
        }

        // Check the new one
        if let Some(item) = self.start_items.get(&new_hour) {
            item.set_checked(true);
        }
    }

    pub fn update_end_hour(&self, new_hour: u8) {
        // Uncheck all end items
        for item in self.end_items.values() {
            item.set_checked(false);
        }

        // Check the new one
        if let Some(item) = self.end_items.get(&new_hour) {
            item.set_checked(true);
        }
    }

    pub fn update_interval(&self, new_interval: u32) {
        // Uncheck all interval items
        for item in self.interval_items.values() {
            item.set_checked(false);
        }

        // Check the new one
        if let Some(item) = self.interval_items.get(&new_interval) {
            item.set_checked(true);
        }
    }
}
