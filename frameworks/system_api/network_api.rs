use crate::networking::stack::unified_socket::{UnifiedSocket, NetworkProtocol, NetworkAddress};

pub struct NetworkAPI {
    socket: UnifiedSocket,
    allowed: bool,
}

impl NetworkAPI {
    pub fn new(protocol: NetworkProtocol, port: u16) -> Self {
        Self {
            socket: UnifiedSocket::new(protocol, port),
            allowed: false,
        }
    }

    pub fn grant_access(&mut self) {
        self.allowed = true;
    }

    pub fn connect(&self, address: NetworkAddress) -> Result<(), &'static str> {
        if !self.allowed {
            return Err("network permission denied");
        }

        self.socket.connect(address)
    }

    pub fn send(&self, data: &[u8]) -> Result<usize, &'static str> {
        if !self.allowed {
            return Err("network permission denied");
        }

        self.socket.send(data)
    }

    pub fn receive(&self, buffer: &mut [u8]) -> Result<usize, &'static str> {
        if !self.allowed {
            return Err("network permission denied");
        }

        self.socket.receive(buffer)
    }
}
