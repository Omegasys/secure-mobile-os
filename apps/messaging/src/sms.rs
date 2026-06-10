use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone)]
pub struct SmsMessage {
    pub id: u64,
    pub sender: String,
    pub recipient: String,
    pub content: String,
    pub timestamp: u64,
}

pub struct SmsManager {
    messages: Vec<SmsMessage>,
}

impl SmsManager {
    pub fn new() -> Self {
        Self {
            messages: Vec::new(),
        }
    }

    pub fn send_message(
        &mut self,
        recipient: String,
        content: String,
    ) {
        let message = SmsMessage {
            id: timestamp(),
            sender: "local_device".to_string(),
            recipient,
            content,
            timestamp: timestamp(),
        };

        self.messages.push(message);

        println!("SMS queued for delivery");
    }

    pub fn receive_message(
        &mut self,
        sender: String,
        content: String,
    ) {
        let message = SmsMessage {
            id: timestamp(),
            sender,
            recipient: "local_device".to_string(),
            content,
            timestamp: timestamp(),
        };

        self.messages.push(message);
    }

    pub fn messages(&self) -> &[SmsMessage] {
        &self.messages
    }
}

fn timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}
