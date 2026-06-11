```rust id="pkt_main"
mod capture;
mod ipv4;
mod ipv6;

use capture::PacketCapture;

fn main() {
    println!("Secure Mobile OS Packet Analyzer");
    println!("================================");

    let mut cap = PacketCapture::new();

    cap.add_dummy_packets();
    cap.show_summary();

    println!("\nIPv4 Analysis:");
    ipv4::analyze_packets(&cap.packets);

    println!("\nIPv6 Analysis:");
    ipv6::analyze_packets(&cap.packets);
}
```
