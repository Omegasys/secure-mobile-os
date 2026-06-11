```rust id="stats001"
use crate::capture::{Packet, Protocol};

pub struct TrafficStats {
    pub total_packets: usize,
    pub ipv4_packets: usize,
    pub ipv6_packets: usize,
    pub total_bytes: u64,
}

pub fn compute_stats(packets: &[Packet]) -> TrafficStats {
    let mut stats = TrafficStats {
        total_packets: packets.len(),
        ipv4_packets: 0,
        ipv6_packets: 0,
        total_bytes: 0,
    };

    for p in packets {
        stats.total_bytes += p.size_bytes as u64;

        match p.protocol {
            Protocol::IPv4 => stats.ipv4_packets += 1,
            Protocol::IPv6 => stats.ipv6_packets += 1,
            Protocol::Unknown => {}
        }
    }

    stats
}

pub fn print_stats(stats: &TrafficStats) {
    println!("Traffic Statistics");
    println!("------------------");
    println!("Total packets: {}", stats.total_packets);
    println!("IPv4 packets: {}", stats.ipv4_packets);
    println!("IPv6 packets: {}", stats.ipv6_packets);
    println!("Total bytes: {}", stats.total_bytes);
}
```
