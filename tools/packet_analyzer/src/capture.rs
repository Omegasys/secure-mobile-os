```rust id="pkt_cap"
#[derive(Debug, Clone)]
pub enum Protocol {
    IPv4,
    IPv6,
    Unknown,
}

#[derive(Debug, Clone)]
pub struct Packet {
    pub id: u32,
    pub protocol: Protocol,
    pub size_bytes: u32,
    pub source: String,
    pub destination: String,
}

pub struct PacketCapture {
    pub packets: Vec<Packet>,
}

impl PacketCapture {
    pub fn new() -> Self {
        Self {
            packets: Vec::new(),
        }
    }

    pub fn add_packet(&mut self, packet: Packet) {
        self.packets.push(packet);
    }

    pub fn add_dummy_packets(&mut self) {
        self.packets = vec![
            Packet {
                id: 1,
                protocol: Protocol::IPv4,
                size_bytes: 128,
                source: "192.168.1.10".to_string(),
                destination: "8.8.8.8".to_string(),
            },
            Packet {
                id: 2,
                protocol: Protocol::IPv6,
                size_bytes: 256,
                source: "fe80::1".to_string(),
                destination: "2001:4860::8888".to_string(),
            },
            Packet {
                id: 3,
                protocol: Protocol::IPv4,
                size_bytes: 64,
                source: "10.0.0.5".to_string(),
                destination: "10.0.0.1".to_string(),
            },
        ];
    }

    pub fn show_summary(&self) {
        println!("Packet Capture Summary");
        println!("----------------------");
        println!("Total packets: {}", self.packets.len());

        let ipv4 = self.packets.iter().filter(|p| matches!(p.protocol, Protocol::IPv4)).count();
        let ipv6 = self.packets.iter().filter(|p| matches!(p.protocol, Protocol::IPv6)).count();

        println!("IPv4 packets: {}", ipv4);
        println!("IPv6 packets: {}", ipv6);
    }
}
```
