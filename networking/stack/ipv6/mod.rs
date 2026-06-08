pub const IPV6_HEADER_SIZE: usize = 40;

#[derive(Clone, Copy)]
pub struct Ipv6Address {
    pub segments: [u16; 8],
}

pub struct Ipv6Packet {
    pub source: Ipv6Address,
    pub destination: Ipv6Address,
    pub next_header: u8,
    pub payload_length: usize,
}

impl Ipv6Packet {
    pub fn new(
        source: Ipv6Address,
        destination: Ipv6Address,
        next_header: u8,
        payload_length: usize,
    ) -> Self {
        Self {
            source,
            destination,
            next_header,
            payload_length,
        }
    }
}
