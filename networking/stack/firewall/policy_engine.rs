use super::packet_filter::{Action, Packet};

#[derive(Clone, Copy)]
pub enum SecurityLevel {
    Low,
    Standard,
    High,
    Paranoid,
}

pub struct PolicyEngine {
    pub level: SecurityLevel,
}

impl PolicyEngine {
    pub const fn new() -> Self {
        Self {
            level: SecurityLevel::Standard,
        }
    }

    pub fn set_level(&mut self, level: SecurityLevel) {
        self.level = level;
    }

    pub fn evaluate_packet(&self, packet: &Packet) -> Action {
        match self.level {
            SecurityLevel::Low => Action::Allow,

            SecurityLevel::Standard => {
                if packet.destination_port == 0 {
                    Action::Drop
                } else {
                    Action::Allow
                }
            }

            SecurityLevel::High => {
                if packet.size > 4096 {
                    Action::Drop
                } else {
                    Action::Allow
                }
            }

            SecurityLevel::Paranoid => Action::Deny,
        }
    }
}
