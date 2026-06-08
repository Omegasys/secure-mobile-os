use crate::networking::stack::unified_socket::NetworkProtocol;

#[derive(Clone, Copy)]
pub enum Action {
    Allow,
    Deny,
    Drop,
}

pub struct Packet {
    pub source_port: u16,
    pub destination_port: u16,
    pub size: usize,
}

pub struct PacketFilterRule {
    pub protocol: NetworkProtocol,
    pub port: u16,
    pub action: Action,
}

pub struct PacketFilter {
    rules: [Option<PacketFilterRule>; 256],
    rule_count: usize,
}

impl PacketFilter {
    pub const fn new() -> Self {
        Self {
            rules: [None; 256],
            rule_count: 0,
        }
    }

    pub fn add_rule(&mut self, rule: PacketFilterRule) -> Result<(), &'static str> {
        if self.rule_count >= 256 {
            return Err("rule limit reached");
        }

        self.rules[self.rule_count] = Some(rule);
        self.rule_count += 1;

        Ok(())
    }

    pub fn evaluate(&self, packet: &Packet, protocol: NetworkProtocol) -> Action {
        for rule in self.rules.iter().flatten() {
            if rule.protocol == protocol && rule.port == packet.destination_port {
                return rule.action;
            }
        }

        Action::Allow
    }
}
