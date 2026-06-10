use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone)]
pub struct Voicemail {
    pub id: u64,
    pub caller_number: String,
    pub timestamp: u64,
    pub duration_seconds: u32,

    pub listened: bool,

    pub audio_path: String,
}

pub struct VoicemailManager {
    messages: Vec<Voicemail>,
}

impl VoicemailManager {
    pub fn new() -> Self {
        Self {
            messages: Vec::new(),
        }
    }

    pub fn add_voicemail(
        &mut self,
        caller_number: String,
        duration_seconds: u32,
        audio_path: String,
    ) {
        self.messages.push(Voicemail {
            id: current_timestamp(),
            caller_number,
            timestamp: current_timestamp(),
            duration_seconds,
            listened: false,
            audio_path,
        });
    }

    pub fn mark_listened(
        &mut self,
        id: u64,
    ) {
        if let Some(message) =
            self.messages.iter_mut().find(|m| m.id == id)
        {
            message.listened = true;
        }
    }

    pub fn delete_voicemail(
        &mut self,
        id: u64,
    ) {
        self.messages.retain(|m| m.id != id);
    }

    pub fn unread_count(&self) -> usize {
        self.messages
            .iter()
            .filter(|m| !m.listened)
            .count()
    }

    pub fn messages(&self) -> &[Voicemail] {
        &self.messages
    }

    pub fn print_messages(&self) {
        println!("--- Voicemail ---");

        for vm in &self.messages {
            println!(
                "{} | {} sec | listened={}",
                vm.caller_number,
                vm.duration_seconds,
                vm.listened
            );
        }
    }
}

fn current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}
