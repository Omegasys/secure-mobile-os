use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone)]
pub struct MediaAttachment {
    pub filename: String,
    pub mime_type: String,
    pub size_bytes: u64,
}

#[derive(Clone)]
pub struct MmsMessage {
    pub id: u64,
    pub sender: String,
    pub recipient: String,
    pub text: String,
    pub attachments: Vec<MediaAttachment>,
    pub timestamp: u64,
}

pub struct MmsManager {
    messages: Vec<MmsMessage>,
}

impl MmsManager {
    pub fn new() -> Self {
        Self {
            messages: Vec::new(),
        }
    }

    pub fn send_message(
        &mut self,
        recipient: String,
        text: String,
        attachments: Vec<MediaAttachment>,
    ) {
        let message = MmsMessage {
            id: timestamp(),
            sender: "local_device".to_string(),
            recipient,
            text,
            attachments,
            timestamp: timestamp(),
        };

        self.messages.push(message);

        println!("MMS queued for delivery");
    }

    pub fn messages(&self) -> &[MmsMessage] {
        &self.messages
    }
}

fn timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}
