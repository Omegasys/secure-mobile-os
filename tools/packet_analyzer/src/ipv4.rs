```rust id="pkt_ipv4"
use crate::capture::{Packet, Protocol};

pub fn analyze_packets(packets: &[Packet]) {
    let ipv4_packets: Vec<&Packet> = packets
        .iter()
        .filter(|p| matches!(p.protocol, Protocol::IPv4))
        .collect();

    println!("IPv4 Packet Analysis");
    println!("--------------------");

    for packet in ipv4_packets {
        println!(
            "ID:{} {} -> {} ({} bytes)",
            packet.id,
            packet.source,
            packet.destination,
            packet.size_bytes
        );
    }

    let total_size: u32 = packets
        .iter()
        .filter(|p| matches!(p.protocol, Protocol::IPv4))
        .map(|p| p.size_bytes)
        .sum();

    println!("Total IPv4 traffic: {} bytes", total_size);
}
```
