use std::collections::VecDeque;

#[derive(Clone)]
pub struct AuditLogEntry {
    pub id: u64,
    pub actor: String,
    pub action: String,
    pub resource: String,
    pub timestamp: u64,
}

pub struct AuditLogManager {
    logs: VecDeque<AuditLogEntry>,
    max_logs: usize,
}

impl AuditLogManager {
    pub fn new() -> Self {
        Self {
            logs: VecDeque::new(),
            max_logs: 5000,
        }
    }

    pub fn record(&mut self, entry: AuditLogEntry) {
        if self.logs.len() >= self.max_logs {
            self.logs.pop_back();
        }

        self.logs.push_front(entry);
    }

    pub fn recent_logs(&self, count: usize) -> Vec<&AuditLogEntry> {
        self.logs.iter().take(count).collect()
    }

    pub fn clear(&mut self) {
        self.logs.clear();
    }

    pub fn total_logs(&self) -> usize {
        self.logs.len()
    }
}
