use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone)]
pub enum MessageStatus {
    Sending,
    Delivered,
    Read,
}

#[derive(Clone)]
pub struct SecureMessage {
    pub id: u64,
    pub sender_id: String,
    pub recipient_id: String,

    pub encrypted_payload: Vec<u8>,

    pub timestamp: u64,

    pub status: MessageStatus,
}

pub struct SecureChatManager {
    conversations: Vec<SecureMessage>,
}

impl SecureChatManager {
    pub fn new() -> Self {
        Self {
            conversations: Vec::new(),
        }
    }

    pub fn send_secure_message(
        &mut self,
        recipient_id: String,
        plaintext: &str,
    ) {
        let encrypted_payload =
            self.encrypt_message(plaintext);

        let message = SecureMessage {
            id: timestamp(),
            sender_id: "local_user".to_string(),
            recipient_id,
            encrypted_payload,
            timestamp: timestamp(),
            status: MessageStatus::Sending,
        };

        self.conversations.push(message);
    }

    fn encrypt_message(
        &self,
        plaintext: &str,
    ) -> Vec<u8> {
        plaintext
            .as_bytes()
            .iter()
            .map(|b| b ^ 0xAA)
            .collect()
    }

    pub fn messages(
        &self,
    ) -> &[SecureMessage] {
        &self.conversations
    }
}

fn timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}
