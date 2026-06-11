```rust id="ovl001"
use crate::capture::{Packet, Protocol};

#[derive(Debug)]
pub struct OverlayFlow {
    pub id: u32,
    pub encapsulated_protocol: String,
    pub encrypted: bool,
    pub tunnel_type: String,
}

pub fn analyze_overlay_packets(packets: &[Packet]) {
    println!("Overlay Network Analysis");
    println!("------------------------");

    let mut flows: Vec<OverlayFlow> = Vec::new();

    for packet in packets {
        // Simulated detection of overlay / VPN-like traffic
        let is_overlay = matches!(packet.protocol, Protocol::IPv6)
            && packet.size_bytes > 200;

        if is_overlay {
            flows.push(OverlayFlow {
                id: packet.id,
                encapsulated_protocol: "IPv6-in-Overlay".to_string(),
                encrypted: true,
                tunnel_type: "Secure Tunnel (Simulated)".to_string(),
            });
        }
    }

    for flow in flows {
        println!(
            "Flow {} | {} | Encrypted: {} | {}",
            flow.id,
            flow.encapsulated_protocol,
            flow.encrypted,
            flow.tunnel_type
        );
    }

    println!("Overlay flows detected: {}", flows.len());
}
```
