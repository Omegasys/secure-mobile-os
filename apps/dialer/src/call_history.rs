use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone)]
pub enum CallType {
    Incoming,
    Outgoing,
    Missed,
}

#[derive(Clone)]
pub struct CallRecord {
    pub id: u64,
    pub phone_number: String,
    pub duration_seconds: u64,
    pub timestamp: u64,
    pub call_type: CallType,
}

pub struct CallHistory {
    records: Vec<CallRecord>,
}

impl CallHistory {
    pub fn new() -> Self {
        Self {
            records: Vec::new(),
        }
    }

    pub fn add_record(
        &mut self,
        phone_number: String,
        duration_seconds: u64,
        call_type: CallType,
    ) {
        self.records.push(CallRecord {
            id: current_timestamp(),
            phone_number,
            duration_seconds,
            timestamp: current_timestamp(),
            call_type,
        });
    }

    pub fn clear_history(&mut self) {
        self.records.clear();
    }

    pub fn records(&self) -> &[CallRecord] {
        &self.records
    }

    pub fn recent_calls(
        &self,
        count: usize,
    ) -> Vec<&CallRecord> {
        self.records
            .iter()
            .rev()
            .take(count)
            .collect()
    }

    pub fn print_history(&self) {
        println!("--- Call History ---");

        for call in &self.records {
            println!(
                "{} | {} sec",
                call.phone_number,
                call.duration_seconds
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
