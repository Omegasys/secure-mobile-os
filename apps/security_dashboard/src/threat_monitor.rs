use std::collections::VecDeque;

#[derive(Clone)]
pub enum ThreatLevel {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Clone)]
pub struct ThreatEvent {
    pub id: u64,
    pub source: String,
    pub description: String,
    pub level: ThreatLevel,
    pub timestamp: u64,
}

pub struct ThreatMonitor {
    enabled: bool,
    events: VecDeque<ThreatEvent>,
}

impl ThreatMonitor {
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

    pub fn report_threat(&mut self, event: ThreatEvent) {
        if !self.enabled {
            return;
        }

        self.events.push_front(event);

        if self.events.len() > 2000 {
            self.events.pop_back();
        }
    }

    pub fn recent_threats(&self, count: usize) -> Vec<&ThreatEvent> {
        self.events.iter().take(count).collect()
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
}
