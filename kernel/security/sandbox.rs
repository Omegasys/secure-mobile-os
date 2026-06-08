pub struct Sandbox {
    pub pid: usize,
    pub network_enabled: bool,
    pub filesystem_enabled: bool,
}

impl Sandbox {
    pub fn new(pid: usize) -> Self {
        Self {
            pid,
            network_enabled: false,
            filesystem_enabled: false,
        }
    }

    pub fn allow_network(&mut self) {
        self.network_enabled = true;
    }

    pub fn allow_filesystem(&mut self) {
        self.filesystem_enabled = true;
    }
}
