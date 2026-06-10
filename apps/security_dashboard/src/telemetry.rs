use std::collections::VecDeque;

#[derive(Clone)]
pub struct TelemetryEvent {
    pub id: u64,
    pub event_type: String,
    pub severity: String,
    pub timestamp: u64,
}

pub struct TelemetryMonitor {
    enabled: bool,
    buffer: VecDeque<TelemetryEvent>,
    max_buffer: usize,
}

impl TelemetryMonitor {
    pub fn new() -> Self {
        Self {
            enabled: false,
            buffer: VecDeque::new(),
            max_buffer: 1000,
        }
    }

    pub fn enable(&mut self) {
        self.enabled = true;
    }

    pub fn disable(&mut self) {
        self.enabled = false;
    }

    pub fn log_event(&mut self, event: TelemetryEvent) {
        if !self.enabled {
            return;
        }

        if self.buffer.len() >= self.max_buffer {
            self.buffer.pop_back();
        }

        self.buffer.push_front(event);
    }

    pub fn recent_events(&self, count: usize) -> Vec<&TelemetryEvent> {
        self.buffer.iter().take(count).collect()
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
}
