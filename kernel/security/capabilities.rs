#[derive(Clone, Copy)]
pub enum Capability {
    Network,
    FileSystem,
    Camera,
    Microphone,
    Location,
    KernelAdmin,
}

pub struct CapabilitySet {
    capabilities: [bool; 6],
}

impl CapabilitySet {
    pub const fn new() -> Self {
        Self {
            capabilities: [false; 6],
        }
    }

    pub fn grant(&mut self, capability: Capability) {
        self.capabilities[capability as usize] = true;
    }

    pub fn has(&self, capability: Capability) -> bool {
        self.capabilities[capability as usize]
    }
}
