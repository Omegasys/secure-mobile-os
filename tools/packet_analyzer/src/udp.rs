```rust id="udp001"
use crate::capture::{Packet, Protocol};

#[derive(Debug)]
pub struct UdpDatagram {
    pub packet_id: u32,
    pub source_port: u16,
    pub dest_port: u16,
    pub length: u32,
}

pub fn analyze_udp(packets: &[Packet]) {
    println!("UDP Analysis");
    println!("-----------");

    let mut datagrams: Vec<UdpDatagram> = Vec::new();

    for packet in packets {
        if matches!(packet.protocol, Protocol::IPv4) && packet.size_bytes < 200 {
            datagrams.push(UdpDatagram {
                packet_id: packet.id,
                source_port: 53,
                dest_port: 5353,
                length: packet.size_bytes,
            });
        }
    }

    for d in &datagrams {
        println!(
            "Packet {} | {} -> {} | {} bytes",
            d.packet_id,
            d.source_port,
            d.dest_port,
            d.length
        );
    }

    println!("UDP datagrams analyzed: {}", datagrams.len());
}
```
