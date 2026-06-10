use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone, Debug)]
pub enum CallState {
    Idle,
    Ringing,
    Outgoing,
    Active,
    Held,
    Ended,
}

#[derive(Clone)]
pub struct ActiveCall {
    pub id: u64,
    pub phone_number: String,
    pub state: CallState,
    pub started_at: Option<u64>,
}

pub struct CallManager {
    current_call: Option<ActiveCall>,
}

impl CallManager {
    pub fn new() -> Self {
        Self {
            current_call: None,
        }
    }

    pub fn start_outgoing_call(
        &mut self,
        number: String,
    ) -> u64 {
        let id = current_timestamp();

        self.current_call = Some(ActiveCall {
            id,
            phone_number: number,
            state: CallState::Outgoing,
            started_at: Some(id),
        });

        println!("Starting outgoing call...");
        id
    }

    pub fn incoming_call(
        &mut self,
        number: String,
    ) -> u64 {
        let id = current_timestamp();

        self.current_call = Some(ActiveCall {
            id,
            phone_number: number,
            state: CallState::Ringing,
            started_at: None,
        });

        println!("Incoming call...");
        id
    }

    pub fn answer_call(&mut self) {
        if let Some(call) = &mut self.current_call {
            call.state = CallState::Active;
            call.started_at = Some(current_timestamp());
        }
    }

    pub fn hold_call(&mut self) {
        if let Some(call) = &mut self.current_call {
            call.state = CallState::Held;
        }
    }

    pub fn resume_call(&mut self) {
        if let Some(call) = &mut self.current_call {
            call.state = CallState::Active;
        }
    }

    pub fn end_call(&mut self) {
        if let Some(call) = &mut self.current_call {
            call.state = CallState::Ended;
        }

        self.current_call = None;
    }

    pub fn current_call(&self) -> Option<&ActiveCall> {
        self.current_call.as_ref()
    }
}

fn current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}
