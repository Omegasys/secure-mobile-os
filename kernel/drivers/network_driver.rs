pub const MAX_PACKET_SIZE: usize = 4096;

pub struct Packet {
    pub length: usize,
    pub data: [u8; MAX_PACKET_SIZE],
}

pub struct NetworkDriver {
    initialized: bool,
}

impl NetworkDriver {
    pub const fn new() -> Self {
        Self {
            initialized: false,
        }
    }

    pub fn initialize(&mut self) {
        self.initialized = true;
    }

    pub fn send(&self, packet: &Packet) -> Result<(), &'static str> {
        if !self.initialized {
            return Err("network not initialized");
        }

        let _ = packet;

        Ok(())
    }

    pub fn receive(&self) -> Option<Packet> {
        None
    }

    pub fn is_initialized(&self) -> bool {
        self.initialized
    }
}
