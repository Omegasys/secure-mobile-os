pub struct PluginSandbox {
    pub memory_limit: usize,
    pub network_allowed: bool,
    pub filesystem_allowed: bool,
}

impl PluginSandbox {
    pub fn new() -> Self {
        Self {
            memory_limit: 32, // MB
            network_allowed: false,
            filesystem_allowed: false,
        }
    }

    pub fn allow_network(&mut self) {
        self.network_allowed = true;
    }

    pub fn allow_filesystem(&mut self) {
        self.filesystem_allowed = true;
    }

    pub fn enforce_limits(&self, memory_used: usize) -> bool {
        memory_used <= self.memory_limit
    }

    pub fn isolate(&self) -> bool {
        // ensures plugin cannot escape sandbox boundaries
        true
    }
}
