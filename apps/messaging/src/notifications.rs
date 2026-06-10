use std::collections::VecDeque;

#[derive(Clone)]
pub enum NotificationType {
    Sms,
    Mms,
    SecureMessage,
    GroupMessage,
}

#[derive(Clone)]
pub struct Notification {
    pub id: u64,
    pub title: String,
    pub body: String,
    pub notification_type: NotificationType,
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

    pub fn push(
        &mut self,
        notification: Notification,
    ) {
        self.notifications.push_front(notification);
    }

    pub fn mark_read(
        &mut self,
        id: u64,
    ) {
        if let Some(notification) =
            self.notifications
                .iter_mut()
                .find(|n| n.id == id)
        {
            notification.read = true;
        }
    }

    pub fn unread_count(&self) -> usize {
        self.notifications
            .iter()
            .filter(|n| !n.read)
            .count()
    }

    pub fn clear(&mut self) {
        self.notifications.clear();
    }

    pub fn all(
        &self,
    ) -> &VecDeque<Notification> {
        &self.notifications
    }
}
