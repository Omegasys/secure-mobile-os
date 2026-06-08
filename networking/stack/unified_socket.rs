use crate::networking::stack::ipv4::Ipv4Address;
use crate::networking::stack::ipv6::Ipv6Address;

#[derive(Clone, Copy)]
pub enum NetworkProtocol {
    IPv4,
    IPv6,
    SecureOverlay,
}

pub enum NetworkAddress {
    IPv4(Ipv4Address),
    IPv6(Ipv6Address),
    Overlay(u64),
}

pub struct UnifiedSocket {
    protocol: NetworkProtocol,
    local_port: u16,
}

impl UnifiedSocket {
    pub fn new(
        protocol: NetworkProtocol,
        port: u16,
    ) -> Self {
        Self {
            protocol,
            local_port: port,
        }
    }

    pub fn connect(
        &self,
        address: NetworkAddress,
    ) -> Result<(), &'static str> {
        let _ = address;

        Ok(())
    }

    pub fn send(
        &self,
        buffer: &[u8],
    ) -> Result<usize, &'static str> {
        Ok(buffer.len())
    }

    pub fn receive(
        &self,
        buffer: &mut [u8],
    ) -> Result<usize, &'static str> {
        let _ = buffer;

        Ok(0)
    }

    pub fn protocol(&self) -> NetworkProtocol {
        self.protocol
    }

    pub fn port(&self) -> u16 {
        self.local_port
    }
}
