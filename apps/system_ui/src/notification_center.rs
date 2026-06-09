use std::collections::VecDeque;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone)]
pub enum NotificationPriority {
    Low,
    Normal,
    High,
    Critical,
}

#[derive(Clone)]
pub struct Notification {
    pub id: u64,
    pub app_name: String,
    pub title: String,
    pub message: String,
    pub timestamp: u64,
    pub priority: NotificationPriority,
    pub read: bool,
}

pub struct NotificationCenter {
    notifications: VecDeque<Notification>,
}

impl NotificationCenter {
    pub fn new() -> Self {
        Self {
            notifications: VecDeque::new(),
        }
    }

    pub fn add_notification(
        &mut self,
        app_name: String,
        title: String,
        message: String,
        priority: NotificationPriority,
    ) {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let notification = Notification {
            id: timestamp,
            app_name,
            title,
            message,
            timestamp,
            priority,
            read: false,
        };

        self.notifications.push_front(notification);
    }

    pub fn mark_read(&mut self, id: u64) {
        for notification in &mut self.notifications {
            if notification.id == id {
                notification.read = true;
            }
        }
    }

    pub fn remove_notification(&mut self, id: u64) {
        self.notifications.retain(|n| n.id != id);
    }

    pub fn clear_all(&mut self) {
        self.notifications.clear();
    }

    pub fn unread_count(&self) -> usize {
        self.notifications
            .iter()
            .filter(|n| !n.read)
            .count()
    }

    pub fn notifications(&self) -> &VecDeque<Notification> {
        &self.notifications
    }

    pub fn critical_notifications(&self) -> Vec<&Notification> {
        self.notifications
            .iter()
            .filter(|n| matches!(n.priority, NotificationPriority::Critical))
            .collect()
    }
}

impl Default for NotificationCenter {
    fn default() -> Self {
        Self::new()
    }
}
