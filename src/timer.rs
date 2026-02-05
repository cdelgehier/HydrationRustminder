use crate::config::Config;
use chrono::Local;
use chrono::Timelike;
use log::info;
use std::time::{Duration, Instant};

pub struct HydrationTimer {
    last_notification: Option<Instant>,
}

impl HydrationTimer {
    pub fn new() -> Self {
        HydrationTimer {
            last_notification: None,
        }
    }

    /// Check if current time is within work hours
    pub fn is_within_work_hours(&self, config: &Config) -> bool {
        let now = Local::now();
        let current_hour = now.hour() as u8;

        current_hour >= config.start_hour && current_hour < config.end_hour
    }

    /// Check if enough time has passed since last notification
    pub fn should_send_notification(&self, config: &Config) -> bool {
        // Not within work hours
        if !self.is_within_work_hours(config) {
            return false;
        }

        // Never sent a notification before
        if self.last_notification.is_none() {
            return true;
        }

        // Check if interval has passed
        let elapsed = self.last_notification.unwrap().elapsed();
        let interval = Duration::from_secs(config.interval_minutes as u64 * 60);

        elapsed >= interval
    }

    /// Mark that a notification was just sent
    pub fn mark_notification_sent(&mut self) {
        self.last_notification = Some(Instant::now());
        info!("Notification marked as sent");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timer_initial_state() {
        let timer = HydrationTimer::new();
        assert!(timer.last_notification.is_none());
    }

    #[test]
    fn test_should_send_first_notification() {
        let timer = HydrationTimer::new();
        let config = Config {
            start_hour: 0,
            end_hour: 23,
            interval_minutes: 30,
            reminder_minutes: 5,
        };

        // First notification should always be sent (if within hours)
        assert!(timer.should_send_notification(&config));
    }

    #[test]
    fn test_mark_notification_sent() {
        let mut timer = HydrationTimer::new();
        timer.mark_notification_sent();

        assert!(timer.last_notification.is_some());
    }
}
