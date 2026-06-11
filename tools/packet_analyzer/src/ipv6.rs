```rust id="pkt_ipv6"
use crate::capture::{Packet, Protocol};

pub fn analyze_packets(packets: &[Packet]) {
    let ipv6_packets: Vec<&Packet> = packets
        .iter()
        .filter(|p| matches!(p.protocol, Protocol::IPv6))
        .collect();

    println!("IPv6 Packet Analysis");
    println!("--------------------");

    for packet in ipv6_packets {
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
        .filter(|p| matches!(p.protocol, Protocol::IPv6))
        .map(|p| p.size_bytes)
        .sum();

    println!("Total IPv6 traffic: {} bytes", total_size);
}
```
