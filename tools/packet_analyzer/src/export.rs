```rust id="exp001"
use crate::capture::{Packet, Protocol};
use std::fs::File;
use std::io::Write;

pub fn export_as_text(
    packets: &[Packet],
    path: &str,
) -> Result<(), String> {
    let mut file =
        File::create(path).map_err(|e| e.to_string())?;

    writeln!(file, "Secure Mobile OS Packet Export")
        .map_err(|e| e.to_string())?;
    writeln!(file, "==============================")
        .map_err(|e| e.to_string())?;

    for p in packets {
        writeln!(
            file,
            "ID:{} {:?} {} -> {} ({} bytes)",
            p.id,
            p.protocol,
            p.source,
            p.destination,
            p.size_bytes
        )
        .map_err(|e| e.to_string())?;
    }

    Ok(())
}

pub fn export_summary(
    packets: &[Packet],
) -> String {
    let total = packets.len();
    let ipv4 = packets
        .iter()
        .filter(|p| matches!(p.protocol, Protocol::IPv4))
        .count();
    let ipv6 = packets
        .iter()
        .filter(|p| matches!(p.protocol, Protocol::IPv6))
        .count();

    format!(
        "Summary:\nTotal: {}\nIPv4: {}\nIPv6: {}",
        total, ipv4, ipv6
    )
}
```
