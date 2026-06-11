```rust id="dns001"
use crate::capture::{Packet, Protocol};

#[derive(Debug)]
pub struct DnsQuery {
    pub packet_id: u32,
    pub domain: String,
    pub query_type: String,
}

pub fn analyze_dns(packets: &[Packet]) {
    println!("DNS Analysis");
    println!("-----------");

    let mut queries: Vec<DnsQuery> = Vec::new();

    for packet in packets {
        // Simulated DNS detection based on destination
        if matches!(packet.protocol, Protocol::IPv4)
            && packet.destination.contains("8.8.8.8")
        {
            queries.push(DnsQuery {
                packet_id: packet.id,
                domain: "example.com".to_string(),
                query_type: "A".to_string(),
            });
        }
    }

    for q in &queries {
        println!(
            "Packet {} | {} | type {}",
            q.packet_id,
            q.domain,
            q.query_type
        );
    }

    println!("DNS queries analyzed: {}", queries.len());
}
```
