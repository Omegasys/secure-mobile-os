```rust id="flt001"
use crate::capture::{Packet, Protocol};

#[derive(Debug)]
pub enum FilterRule {
    AllowIPv4,
    AllowIPv6,
    BlockLargePackets(u32),
    AllowSource(String),
}

pub fn apply_filters(
    packets: &[Packet],
    rules: &[FilterRule],
) -> Vec<Packet> {
    let mut result = Vec::new();

    'packet_loop: for p in packets {
        for rule in rules {
            match rule {
                FilterRule::AllowIPv4 => {
                    if !matches!(p.protocol, Protocol::IPv4) {
                        continue 'packet_loop;
                    }
                }

                FilterRule::AllowIPv6 => {
                    if !matches!(p.protocol, Protocol::IPv6) {
                        continue 'packet_loop;
                    }
                }

                FilterRule::BlockLargePackets(max) => {
                    if p.size_bytes > *max {
                        continue 'packet_loop;
                    }
                }

                FilterRule::AllowSource(src) => {
                    if &p.source != src {
                        continue 'packet_loop;
                    }
                }
            }
        }

        result.push(p.clone());
    }

    result
}

pub fn print_filtered(packets: &[Packet]) {
    println!("Filtered Packets");
    println!("----------------");

    for p in packets {
        println!(
            "ID:{} {:?} {} -> {} ({} bytes)",
            p.id,
            p.protocol,
            p.source,
            p.destination,
            p.size_bytes
        );
    }
}
```
