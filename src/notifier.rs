use log::info;
use std::error::Error;

pub struct Notifier {
    app_name: String,
}

impl Notifier {
    pub fn new() -> Self {
        info!("Creating notifier");

        // Set application bundle identifier once on macOS
        #[cfg(target_os = "macos")]
        {
            // Use our bundle ID directly instead of searching
            let bundle = "com.cdelgehier.hydration-rustminder";
            log::info!("Using bundle identifier: {}", bundle);
            if let Err(e) = mac_notification_sys::set_application(bundle) {
                log::warn!("Failed to set application bundle: {}", e);
            }
        }

        Notifier {
            app_name: "HydrationRustminder".to_string(),
        }
    }

    #[allow(dead_code)]
    pub fn send_hydration_reminder(&self) -> Result<(), Box<dyn Error>> {
        info!("Sending hydration reminder");

        #[cfg(target_os = "macos")]
        {
            mac_notification_sys::Notification::new()
                .title(&self.app_name)
                .message("💧 Time to drink water!")
                .send()?;
        }

        #[cfg(not(target_os = "macos"))]
        {
            use notify_rust::Notification;
            Notification::new()
                .summary(&self.app_name)
                .body("💧 Time to drink water!")
                .timeout(0)
                .show()?;
        }

        Ok(())
    }

    #[allow(dead_code)]
    pub fn send_followup_reminder(&self) -> Result<(), Box<dyn Error>> {
        info!("Sending followup reminder");

        #[cfg(target_os = "macos")]
        {
            mac_notification_sys::Notification::new()
                .title(&self.app_name)
                .message("💧 Don't forget to drink water!")
                .sound("Ping")
                .send()?;
        }

        #[cfg(not(target_os = "macos"))]
        {
            use notify_rust::Notification;
            Notification::new()
                .summary(&self.app_name)
                .body("💧 Don't forget to drink water!")
                .sound_name("Ping")
                .timeout(0)
                .show()?;
        }

        Ok(())
    }

    pub fn send_startup_notification(&self) -> Result<(), Box<dyn Error>> {
        info!("Sending startup notification");

        #[cfg(target_os = "macos")]
        {
            mac_notification_sys::Notification::new()
                .title(&self.app_name)
                .message("💧 Water reminder started!")
                .send()?;
        }

        #[cfg(not(target_os = "macos"))]
        {
            use notify_rust::Notification;
            Notification::new()
                .summary(&self.app_name)
                .body("💧 Water reminder started!")
                .timeout(5000)
                .show()?;
        }

        Ok(())
    }
}
