pub const IPV4_HEADER_SIZE: usize = 20;

#[derive(Clone, Copy)]
pub struct Ipv4Address {
    pub octets: [u8; 4],
}

pub struct Ipv4Packet {
    pub source: Ipv4Address,
    pub destination: Ipv4Address,
    pub protocol: u8,
    pub payload_length: usize,
}

impl Ipv4Packet {
    pub fn new(
        source: Ipv4Address,
        destination: Ipv4Address,
        protocol: u8,
        payload_length: usize,
    ) -> Self {
        Self {
            source,
            destination,
            protocol,
            payload_length,
        }
    }
}
