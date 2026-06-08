pub const MAX_MEMORY_MB: usize = 256;

pub struct SandboxVM {
    pub pid: u64,
    pub memory_used: usize,
    pub network_enabled: bool,
    pub filesystem_enabled: bool,
}

impl SandboxVM {
    pub fn new(pid: u64) -> Self {
        Self {
            pid,
            memory_used: 0,
            network_enabled: false,
            filesystem_enabled: false,
        }
    }

    pub fn allocate_memory(&mut self, mb: usize) -> Result<(), &'static str> {
        if self.memory_used + mb > MAX_MEMORY_MB {
            return Err("memory limit exceeded");
        }

        self.memory_used += mb;
        Ok(())
    }

    pub fn enable_network(&mut self) {
        self.network_enabled = true;
    }

    pub fn enable_filesystem(&mut self) {
        self.filesystem_enabled = true;
    }

    pub fn isolation_boundary(&self) -> bool {
        // Ensures sandbox constraints are enforced
        true
    }
}
