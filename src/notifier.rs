use log::info;
use notify_rust::Notification;
use std::error::Error;

pub struct Notifier {
    app_name: String,
}

impl Notifier {
    pub fn new() -> Self {
        info!("Creating notifier");
        Notifier {
            app_name: "HydrationRustminder".to_string(),
        }
    }

    #[allow(dead_code)]
    pub fn send_hydration_reminder(&self) -> Result<(), Box<dyn Error>> {
        info!("Sending hydration reminder");
        Notification::new()
            .summary(&self.app_name)
            .body("💧 Time to drink water!")
            .timeout(0) // Stay until clicked
            .show()?;

        Ok(())
    }

    #[allow(dead_code)]
    pub fn send_followup_reminder(&self) -> Result<(), Box<dyn Error>> {
        info!("Sending followup reminder");
        Notification::new()
            .summary(&self.app_name)
            .body("💧 Don't forget to drink water!")
            .sound_name("Ping")
            .timeout(0)
            .show()?;

        Ok(())
    }

    pub fn send_startup_notification(&self) -> Result<(), Box<dyn Error>> {
        info!("Sending startup notification");
        Notification::new()
            .summary(&self.app_name)
            .body("💧 Water reminder started!")
            .timeout(5000) // 5 seconds
            .show()?;

        Ok(())
    }
}
