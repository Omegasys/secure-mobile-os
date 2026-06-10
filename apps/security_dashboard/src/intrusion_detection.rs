use std::collections::VecDeque;

#[derive(Clone, Debug)]
pub enum IntrusionSeverity {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Clone)]
pub struct IntrusionEvent {
    pub id: u64,
    pub source: String,
    pub description: String,
    pub severity: IntrusionSeverity,
    pub timestamp: u64,
}

pub struct IntrusionDetectionSystem {
    enabled: bool,
    events: VecDeque<IntrusionEvent>,
}

impl IntrusionDetectionSystem {
    pub fn new() -> Self {
        Self {
            enabled: true,
            events: VecDeque::new(),
        }
    }

    pub fn enable(&mut self) {
        self.enabled = true;
    }

    pub fn disable(&mut self) {
        self.enabled = false;
    }

    pub fn report_event(&mut self, event: IntrusionEvent) {
        if !self.enabled {
            return;
        }

        self.events.push_front(event);

        if self.events.len() > 3000 {
            self.events.pop_back();
        }
    }

    pub fn recent_events(&self, count: usize) -> Vec<&IntrusionEvent> {
        self.events.iter().take(count).collect()
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
}
