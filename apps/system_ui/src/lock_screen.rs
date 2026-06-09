use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone, Copy, PartialEq)]
pub enum LockState {
    Locked,
    Unlocking,
    Unlocked,
}

pub struct NotificationPreview {
    pub id: u64,
    pub app_name: String,
    pub title: String,
}

pub struct LockScreen {
    state: LockState,
    notifications: Vec<NotificationPreview>,
    failed_attempts: u32,
    last_unlock_time: Option<u64>,
}

impl LockScreen {
    pub fn new() -> Self {
        Self {
            state: LockState::Locked,
            notifications: Vec::new(),
            failed_attempts: 0,
            last_unlock_time: None,
        }
    }

    pub fn lock(&mut self) {
        self.state = LockState::Locked;
    }

    pub fn unlock_with_pin(
        &mut self,
        entered_pin: &str,
        stored_pin: &str,
    ) -> bool {
        self.state = LockState::Unlocking;

        if entered_pin == stored_pin {
            self.state = LockState::Unlocked;
            self.failed_attempts = 0;

            self.last_unlock_time = Some(
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
            );

            true
        } else {
            self.failed_attempts += 1;
            self.state = LockState::Locked;
            false
        }
    }

    pub fn unlock_with_biometric(
        &mut self,
        biometric_result: bool,
    ) -> bool {
        self.state = LockState::Unlocking;

        if biometric_result {
            self.state = LockState::Unlocked;

            self.last_unlock_time = Some(
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
            );

            true
        } else {
            self.failed_attempts += 1;
            self.state = LockState::Locked;
            false
        }
    }

    pub fn add_notification(
        &mut self,
        notification: NotificationPreview,
    ) {
        self.notifications.push(notification);
    }

    pub fn clear_notifications(&mut self) {
        self.notifications.clear();
    }

    pub fn notifications(&self) -> &[NotificationPreview] {
        &self.notifications
    }

    pub fn failed_attempts(&self) -> u32 {
        self.failed_attempts
    }

    pub fn state(&self) -> LockState {
        self.state
    }

    pub fn last_unlock_time(&self) -> Option<u64> {
        self.last_unlock_time
    }
}

impl Default for LockScreen {
    fn default() -> Self {
        Self::new()
    }
}
