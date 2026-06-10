mod sms;
mod mms;
mod secure_chat;

use sms::SmsManager;
use mms::MmsManager;
use secure_chat::SecureChatManager;

pub struct MessagingApp {
    pub sms: SmsManager,
    pub mms: MmsManager,
    pub secure_chat: SecureChatManager,
}

impl MessagingApp {
    pub fn new() -> Self {
        Self {
            sms: SmsManager::new(),
            mms: MmsManager::new(),
            secure_chat: SecureChatManager::new(),
        }
    }

    pub fn start(&self) {
        println!("Messaging application started");
    }
}

fn main() {
    let app = MessagingApp::new();
    app.start();
}
