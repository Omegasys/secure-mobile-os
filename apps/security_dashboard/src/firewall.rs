#[derive(Clone)]
pub struct FirewallRule {
    pub id: u64,
    pub name: String,
    pub enabled: bool,
}

pub struct FirewallDashboard {
    enabled: bool,
    rules: Vec<FirewallRule>,
}

impl FirewallDashboard {
    pub fn new() -> Self {
        Self {
            enabled: true,
            rules: Vec::new(),
        }
    }

    pub fn enable(&mut self) {
        self.enabled = true;
    }

    pub fn disable(&mut self) {
        self.enabled = false;
    }

    pub fn enabled(&self) -> bool {
        self.enabled
    }

    pub fn add_rule(
        &mut self,
        rule: FirewallRule,
    ) {
        self.rules.push(rule);
    }

    pub fn remove_rule(
        &mut self,
        id: u64,
    ) {
        self.rules.retain(|r| r.id != id);
    }

    pub fn rules(&self) -> &[FirewallRule] {
        &self.rules
    }
}

impl Default for FirewallDashboard {
    fn default() -> Self {
        Self::new()
    }
}
